use crate::{
    board::{Board, Color, Piece, PieceType, RANK_1, RANK_2, RANK_4, RANK_8},
    square::Square,
};

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

pub fn is_in_check(color: Color, opponent_moves: &[Move]) -> bool {
    opponent_moves.iter().any(|m| match m {
        Move::Normal {
            capture: Some(p), ..
        } if p.typ == PieceType::King && p.color == color => true,
        _ => false,
    })
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
    pub fn make(&self, board: &mut Board) {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                capture,
                promotion,
            } => {
                board.set(from, None);

                let piece = match promotion {
                    Some(x) => x,
                    None => piece,
                };

                board.set(to, Some(piece));
            }
            Move::Castling {
                from,
                to,
                from_rook,
                to_rook,
                color,
            } => {
                board.set(from, None);
                board.set(from_rook, None);
                board.set(to, Some(Piece::new(PieceType::King, color)));
                board.set(to_rook, Some(Piece::new(PieceType::Rook, color)));
            }
        }
    }

    pub fn unmake(&self, board: &mut Board) {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                capture,
                promotion,
            } => {
                board.set(from, Some(piece));
                board.set(to, capture);
            }
            Move::Castling {
                from,
                to,
                from_rook,
                to_rook,
                color,
            } => {
                board.set(from, Some(Piece::new(PieceType::King, color)));
                board.set(to, None);
                board.set(from_rook, Some(Piece::new(PieceType::Rook, color)));
                board.set(to_rook, None);
            }
        }
    }

    pub fn is_legal(&self, opponent_moves: &[Move]) -> bool {
        match *self {
            Move::Normal {
                from: _,
                to: _,
                piece,
                capture: _,
                promotion: _,
            } => !is_in_check(piece.color, opponent_moves),
            Move::Castling {
                from: _,
                to: _,
                from_rook: _,
                to_rook: _,
                color: _,
            } => false,
        }
    }
}

/// Generate pseudo legal moves
pub fn enumerate_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = Vec::with_capacity(1000);
    let mut has_king = false;
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_nocheck(file, rank);
            if let Some(piece) = board.get(sq) {
                if piece.color == color {
                    match piece.typ {
                        PieceType::King => {
                            has_king = true;
                            enumerate_king(board, color, sq, &mut moves);
                        }
                        PieceType::Queen => enumerate_queen(board, color, sq, &mut moves),
                        PieceType::Rook => enumerate_rook(board, color, sq, &mut moves),
                        PieceType::Bishop => enumerate_bishop(board, color, sq, &mut moves),
                        PieceType::Knight => enumerate_knight(board, color, sq, &mut moves),
                        PieceType::Pawn => enumerate_pawn(board, color, sq, &mut moves),
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
