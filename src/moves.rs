use crate::{
    bitboard::{self, bitboard_iter, shift_north_east, shift_north_west, shift_south_east, shift_south_west},
    board::{Color, Piece, PieceType},
    game::{Game, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE},
    square::square_string,
};

// TODO: don't use Board::set because it is slow

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Normal {
        from: u8,
        to: u8,
        piece: Piece,
        promotion: Option<Piece>,
    },
    Castling {
        from: u8,
        to: u8,
        from_rook: u8,
        to_rook: u8,
        color: Color,
    },
}

impl Move {
    pub fn new(from: u8, to: u8, piece: Piece, promotion: Option<PieceType>) -> Self {
        Move::Normal {
            from,
            to,
            piece,
            promotion: promotion.map(|typ| Piece::new(typ, piece.color)),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Move::Normal {
                from,
                to,
                piece: _,
                promotion,
            } => {
                let mut res: String = square_string(from) + &square_string(to);

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
                from_rook: _,
                to_rook: _,
                color: _,
            } => square_string(from) + &square_string(to),
        }
    }

    // TODO: take game and update castling rights
    pub fn make(&self, game: &mut Game) {
        match *self {
            Move::Normal {
                from,
                to,
                piece,
                promotion,
            } => {
                match piece {
                    Piece {
                        typ: PieceType::King,
                        color: Color::White,
                    } => game.castling_rights &= !(WHITE_QUEENSIDE | WHITE_KINGSIDE),
                    Piece {
                        typ: PieceType::King,
                        color: Color::Black,
                    } => game.castling_rights &= !(BLACK_QUEENSIDE | BLACK_KINGSIDE),
                    Piece {
                        typ: PieceType::Rook,
                        color: Color::Black,
                    } => {
                        if from == bitboard::A8 {
                            game.castling_rights &= !BLACK_QUEENSIDE;
                        } else if from == bitboard::H8 {
                            game.castling_rights &= !BLACK_KINGSIDE;
                        }
                    }
                    Piece {
                        typ: PieceType::Rook,
                        color: Color::White,
                    } => match from {
                        bitboard::A1 => game.castling_rights &= !WHITE_QUEENSIDE,
                        bitboard::H1 => game.castling_rights &= !WHITE_KINGSIDE,
                        _ => {}
                    },
                    _ => {}
                }
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
                match color {
                    Color::White => game.castling_rights &= !(WHITE_QUEENSIDE | WHITE_KINGSIDE),
                    Color::Black => game.castling_rights &= !(BLACK_QUEENSIDE | BLACK_KINGSIDE),
                }
                game.board.set(from, None);
                game.board.set(from_rook, None);
                game.board.set(to, Some(Piece::new(PieceType::King, color)));
                game.board
                    .set(to_rook, Some(Piece::new(PieceType::Rook, color)));
            }
        }

        game.player = game.player.opposite();
    }
}

fn enumerate_white_pawns(game: &Game, moves: &mut Vec<Move>) {
    // TODO: promotions
    let empty = !(game.board.white_pieces | game.board.black_pieces);
    let res = (game.board.white_pawns << 8) & empty;
    for to in bitboard_iter(res) {
        moves.push(Move::new(
            to - 8,
            to,
            Piece::new(PieceType::Pawn, Color::White),
            None,
        ));
    }

    let attacks_nw = shift_north_west(game.board.white_pawns) & game.board.black_pieces;
    let attacks_ne = shift_north_east(game.board.white_pawns) & game.board.black_pieces;

    for to in bitboard_iter(attacks_nw) {
        moves.push(Move::new(
            to - 7,
            to,
            Piece::new(PieceType::Pawn, Color::White),
            None
        ));
    }

    for to in bitboard_iter(attacks_ne){
        moves.push(Move::new(
            to-9,
            to,
            Piece::new(PieceType::Pawn, Color::White),
            None
        ))
    }
}

fn enumerate_black_pawns(game: &Game, moves: &mut Vec<Move>) {
    let empty = !(game.board.white_pieces | game.board.black_pieces);
    let res = (game.board.black_pawns >> 8) & empty;

    for to in bitboard_iter(res) {
        moves.push(Move::new(
            to + 8,
            to,
            Piece::new(PieceType::Pawn, Color::Black),
            None,
        ));
    }

    let attacks_sw = shift_south_west(game.board.black_pawns) & game.board.white_pieces;
    let attacks_se = shift_south_east(game.board.black_pawns) & game.board.white_pieces;

    for to in bitboard_iter(attacks_sw) {
        moves.push(Move::new(
            to + 9,
            to,
            Piece::new(PieceType::Pawn, Color::Black),
            None
        ));
    }

    for to in bitboard_iter(attacks_se){
        moves.push(Move::new(
            to + 7,
            to,
            Piece::new(PieceType::Pawn, Color::Black),
            None
        ))
    }
}

/// Generate pseudo legal moves
pub fn enumerate_moves(game: &Game) -> Vec<Move> {
    let mut moves = Vec::with_capacity(1000);
    match game.player {
        Color::White => enumerate_white_pawns(game, &mut moves),
        Color::Black => enumerate_black_pawns(game, &mut moves),
    }

    moves
}

/*
fn enumerate_castlings(game: &Game, moves: &mut Vec<Move>) {
    if game.castling_rights | WHITE_QUEENSIDE != 0
        && game
            .board
            .get(Square::new_nocheck(FILE_B, RANK_1))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_C, RANK_1))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_D, RANK_1))
            .is_none()
    {
        moves.push(Move::Castling {
            from: Square::new_nocheck(FILE_E, RANK_1),
            to: Square::new_nocheck(FILE_C, RANK_1),
            from_rook: Square::new_nocheck(FILE_A, RANK_1),
            to_rook: Square::new_nocheck(FILE_D, RANK_1),
            color: Color::White,
            castling_rights: game.castling_rights,
        });
    }

    if game.castling_rights | WHITE_KINGSIDE != 0
        && game
            .board
            .get(Square::new_nocheck(FILE_F, RANK_1))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_G, RANK_1))
            .is_none()
    {
        moves.push(Move::Castling {
            from: Square::new_nocheck(FILE_E, RANK_1),
            to: Square::new_nocheck(FILE_G, RANK_1),
            from_rook: Square::new_nocheck(FILE_H, RANK_1),
            to_rook: Square::new_nocheck(FILE_F, RANK_1),
            color: Color::White,
            castling_rights: game.castling_rights,
        });
    }

    if game.castling_rights | BLACK_QUEENSIDE != 0
        && game
            .board
            .get(Square::new_nocheck(FILE_B, RANK_8))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_C, RANK_8))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_D, RANK_8))
            .is_none()
    {
        moves.push(Move::Castling {
            from: Square::new_nocheck(FILE_E, RANK_8),
            to: Square::new_nocheck(FILE_C, RANK_8),
            from_rook: Square::new_nocheck(FILE_A, RANK_8),
            to_rook: Square::new_nocheck(FILE_D, RANK_8),
            color: Color::Black,
            castling_rights: game.castling_rights,
        });
    }

    if game.castling_rights | BLACK_KINGSIDE != 0
        && game
            .board
            .get(Square::new_nocheck(FILE_F, RANK_8))
            .is_none()
        && game
            .board
            .get(Square::new_nocheck(FILE_G, RANK_8))
            .is_none()
    {
        moves.push(Move::Castling {
            from: Square::new_nocheck(FILE_E, RANK_1),
            to: Square::new_nocheck(FILE_G, RANK_1),
            from_rook: Square::new_nocheck(FILE_H, RANK_8),
            to_rook: Square::new_nocheck(FILE_F, RANK_8),
            color: Color::Black,
            castling_rights: game.castling_rights,
        });
    }
}

fn enumerate_promotions(
    game: &Game,
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
        moves.push(Move::new(game, from, to, Some(*typ)));
    }
}

fn enumerate_pawn(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
    let off_rank = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let simple = (0, off_rank);

    if let Some(simple_sq) = from.offset(simple) {
        if game.board.get(simple_sq).is_none() {
            if simple_sq.rank() == RANK_1 && color == Color::Black
                || simple_sq.rank() == RANK_8 && color == Color::White
            {
                enumerate_promotions(game, from, simple_sq, moves);
            } else {
                moves.push(Move::new(game, from, simple_sq, None));
            }

            if from.rank() == RANK_2 {
                let double_sq = Square::new_nocheck(from.file(), RANK_4);
                if game.board.get(double_sq).is_none() {
                    moves.push(Move::new(game, from, double_sq, None));
                }
            }
        }
    }

    let captures_off = [(-1, off_rank), (1, off_rank)];
    for off in captures_off.iter() {
        if let Some(sq) = from.offset(*off) {
            match game.board.get(sq) {
                Some(p) if p.color != color => {
                    if (sq.rank() == RANK_1 && color == Color::Black)
                        || (sq.rank() == RANK_8 && color == Color::White)
                    {
                        enumerate_promotions(game, from, sq, moves);
                    } else {
                        moves.push(Move::new(game, from, sq, None))
                    }
                }
                _ => {}
            }
        }
    }
}

fn enumerate_king(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(sq) = from.offset((i, j)) {
                if !game.board.contains_ally(sq, color) {
                    moves.push(Move::new(game, from, sq, None));
                }
            }
        }
    }
}

fn enumerate_straight_line(
    game: &Game,
    color: Color,
    from: Square,
    moves: &mut Vec<Move>,
    dir: (i8, i8),
) {
    let mut i = 1i8;
    while let Some(sq) = from.offset((i * dir.0, i * dir.1)) {
        match game.board.get(sq) {
            Some(p) if p.color == color => break,
            Some(p) if p.color != color => {
                moves.push(Move::new(game, from, sq, None));
                break;
            }
            None => moves.push(Move::new(game, from, sq, None)),
            Some(_) => unreachable!(),
        }
        i += 1;
    }
}

fn enumerate_queen(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            enumerate_straight_line(game, color, from, moves, (i, j));
        }
    }
}

fn enumerate_rook(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
    enumerate_straight_line(game, color, from, moves, (-1, 0));
    enumerate_straight_line(game, color, from, moves, (1, 0));
    enumerate_straight_line(game, color, from, moves, (0, -1));
    enumerate_straight_line(game, color, from, moves, (0, 1));
}

fn enumerate_bishop(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
    enumerate_straight_line(game, color, from, moves, (-1, -1));
    enumerate_straight_line(game, color, from, moves, (-1, 1));
    enumerate_straight_line(game, color, from, moves, (1, -1));
    enumerate_straight_line(game, color, from, moves, (1, 1));
}

fn enumerate_knight(game: &Game, color: Color, from: Square, moves: &mut Vec<Move>) {
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
            if !game.board.contains_ally(sq, color) {
                moves.push(Move::new(game, from, sq, None));
            }
        }
    }
}
*/
