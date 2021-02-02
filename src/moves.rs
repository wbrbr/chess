use crate::{board::{Board, Color, Piece, PieceType, RANK_1, RANK_2, RANK_4, RANK_8}, game::Game, square::Square};

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Normal {
        from: Square,
        to: Square,
        piece: Piece,
        capture: Option<Piece>,
        promotion: Option<Piece>,
    },
    Castling {
        from: Square,
        to: Square,
        from_rook: Square,
        to_rook: Square,
        color: Color,
    },
}

impl Move {
    pub fn new(board: &Board, from: Square, to: Square, promotion: Option<PieceType>) -> Self {
        let piece = board.get(from).expect("the from square is empty");
        let capture = board.get(to);

        Move::Normal {
            from: from,
            to: to,
            piece: piece,
            capture: capture,
            promotion: promotion.map(|typ| Piece::new(typ, piece.color)),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                capture,
                promotion,
            } => {
                let mut res: String = from.to_string() + &to.to_string();

                if let Some(p) = promotion {
                    res.push(match p.typ {
                        PieceType::Queen => 'q',
                        PieceType::Bishop => 'b',
                        PieceType::Rook => 'r',
                        PieceType::Knight => 'n',
                        _ => unreachable!(),
                    })
                }

                res
            }
            Move::Castling {
                from,
                to,
                from_rook,
                to_rook,
                color,
            } => from.to_string() + &to.to_string(),
        }
    }

    // TODO: take game and update castling rights
    pub fn make(&self, game: &mut Game) {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                capture,
                promotion,
            } => {
                game.board.set(from, None);

                let piece = match promotion {
                    Some(x) => x,
                    None => piece,
                };

                game.board.set(to, Some(piece));
            }
            Move::Castling {
                from,
                to,
                from_rook,
                to_rook,
                color,
            } => {
                game.board.set(from, None);
                game.board.set(from_rook, None);
                game.board.set(to, Some(Piece::new(PieceType::King, color)));
                game.board.set(to_rook, Some(Piece::new(PieceType::Rook, color)));
            }
        }

        game.player = game.player.opposite();
    }

    pub fn unmake(&self, game: &mut Game) {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                capture,
                promotion,
            } => {
                game.board.set(from, Some(piece));
                game.board.set(to, capture);
            }
            Move::Castling {
                from,
                to,
                from_rook,
                to_rook,
                color,
            } => {
                game.board.set(from, Some(Piece::new(PieceType::King, color)));
                game.board.set(to, None);
                game.board.set(from_rook, Some(Piece::new(PieceType::Rook, color)));
                game.board.set(to_rook, None);
            }
        }

        game.player = game.player.opposite();
    }
}

/// Generate pseudo legal moves
pub fn enumerate_moves(game: &Game) -> Vec<Move> {
    let mut moves = Vec::with_capacity(1000);
    let mut has_king = false;
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_nocheck(file, rank);
            if let Some(piece) = game.board.get(sq) {
                if piece.color == game.player {
                    match piece.typ {
                        PieceType::King => {
                            has_king = true;
                            enumerate_king(&game.board, game.player, sq, &mut moves);
                        }
                        PieceType::Queen => enumerate_queen(&game.board, game.player, sq, &mut moves),
                        PieceType::Rook => enumerate_rook(&game.board, game.player, sq, &mut moves),
                        PieceType::Bishop => enumerate_bishop(&game.board, game.player, sq, &mut moves),
                        PieceType::Knight => enumerate_knight(&game.board, game.player, sq, &mut moves),
                        PieceType::Pawn => enumerate_pawn(&game.board, game.player, sq, &mut moves),
                    }
                }
            }
        }
    }

    if !has_king {
        unreachable!();
    }

    moves
}

fn enumerate_promotions(
    board: &Board,
    color: Color,
    from: Square,
    to: Square,
    moves: &mut Vec<Move>,
) {
    for typ in &[
        PieceType::Queen,
        PieceType::Knight,
        PieceType::Rook,
        PieceType::Bishop,
    ] {
        moves.push(Move::new(board, from, to, Some(*typ)));
    }
}

fn enumerate_pawn(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let off_rank = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let simple = (0, off_rank);

    if let Some(simple_sq) = from.offset(simple) {
        if board.get(simple_sq).is_none() {
            if simple_sq.rank() == RANK_1 && color == Color::Black
                || simple_sq.rank() == RANK_8 && color == Color::White
            {
                enumerate_promotions(board, color, from, simple_sq, moves);
            } else {
                moves.push(Move::new(board, from, simple_sq, None));
            }

            if from.rank() == RANK_2 {
                let double_sq = Square::new_nocheck(from.file(), RANK_4);
                if board.get(double_sq).is_none() {
                    moves.push(Move::new(board, from, double_sq, None));
                }
            }
        }
    }

    let captures_off = [(-1, off_rank), (1, off_rank)];
    for off in captures_off.iter() {
        if let Some(sq) = from.offset(*off) {
            match board.get(sq) {
                Some(p) if p.color != color => {
                    if (sq.rank() == RANK_1 && color == Color::Black)
                        || (sq.rank() == RANK_8 && color == Color::White)
                    {
                        enumerate_promotions(board, color, from, sq, moves);
                    } else {
                        moves.push(Move::new(board, from, sq, None))
                    }
                }
                _ => {}
            }
        }
    }
}

fn enumerate_king(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(sq) = from.offset((i, j)) {
                if !board.contains_ally(sq, color) {
                    moves.push(Move::new(board, from, sq, None));
                }
            }
        }
    }
}

fn enumerate_straight_line(
    board: &Board,
    color: Color,
    from: Square,
    moves: &mut Vec<Move>,
    dir: (i8, i8),
) {
    let mut i = 1i8;
    while let Some(sq) = from.offset((i * dir.0, i * dir.1)) {
        match board.get(sq) {
            Some(p) if p.color == color => break,
            Some(p) if p.color != color => {
                moves.push(Move::new(board, from, sq, None));
                break;
            }
            None => moves.push(Move::new(board, from, sq, None)),
            Some(_) => unreachable!(),
        }
        i += 1;
    }
}

fn enumerate_queen(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            enumerate_straight_line(board, color, from, moves, (i, j));
        }
    }
}

fn enumerate_rook(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    enumerate_straight_line(board, color, from, moves, (-1, 0));
    enumerate_straight_line(board, color, from, moves, (1, 0));
    enumerate_straight_line(board, color, from, moves, (0, -1));
    enumerate_straight_line(board, color, from, moves, (0, 1));
}

fn enumerate_bishop(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    enumerate_straight_line(board, color, from, moves, (-1, -1));
    enumerate_straight_line(board, color, from, moves, (-1, 1));
    enumerate_straight_line(board, color, from, moves, (1, -1));
    enumerate_straight_line(board, color, from, moves, (1, 1));
}

fn enumerate_knight(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (-1, -2),
        (1, 2),
        (1, -2),
        (2, -1),
        (2, 1),
    ];

    for off in offsets.iter() {
        if let Some(sq) = from.offset(*off) {
            if !board.contains_ally(sq, color) {
                moves.push(Move::new(board, from, sq, None));
            }
        }
    }
}
