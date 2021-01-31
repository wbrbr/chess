use crate::{board::{Board, Color, Piece, PieceType}, square::Square};

fn value(typ: PieceType, depth: u32) -> i32 {
    match typ {
        PieceType::King => 100000 - depth as i32,
        PieceType::Queen => 1000,
        PieceType::Rook => 525,
        PieceType::Bishop => 350,
        PieceType::Knight => 350,
        PieceType::Pawn => 100,
    }
}

#[rustfmt::skip]
const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,40,-50 ,
];

#[rustfmt::skip]
const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

pub fn piece_square_value(p: Piece, file: u8, rank: u8) -> i32 {
    let index = match p.color {
        Color::White => rank * 8 + file,
        Color::Black => (7 - rank) * 8 + file,
    } as usize;

    match p.typ {
        PieceType::Knight => KNIGHT_TABLE[index],
        PieceType::Bishop => BISHOP_TABLE[index],
        _ => 0
    }
}

pub fn evaluate(board: &Board, depth: u32) -> i32 {
    // TODO: check
    let mut val = 0;

    for rank in 0u8..8u8 {
        for file in 0u8..8u8 {
            let sq = Square::new_nocheck(file, rank);
            if let Some(p) = board.get(sq) {
                val += p.color.to_int() * (value(p.typ, depth) + piece_square_value(p, file, rank));
            }
        }
    }
    val
}
