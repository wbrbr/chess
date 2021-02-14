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
    // TODO: piece square value
    assert!(board.black_king != 0);
    assert!(board.white_king != 0);
    assert_eq!(board.white_pieces, board.white_king | board.white_queens | board.white_rooks | board.white_bishops | board.white_knights | board.white_pawns);
    assert_eq!(board.black_pieces, board.black_king | board.black_queens | board.black_rooks | board.black_bishops | board.black_knights | board.black_pawns);

    1000 * (board.white_queens.count_ones() as i32 - board.black_queens.count_ones() as i32)
    + 525 * (board.white_rooks.count_ones() as i32- board.black_rooks.count_ones() as i32)
    + 350 * (board.white_bishops.count_ones() as i32 - board.black_bishops.count_ones() as i32)
    + 350 * (board.white_knights.count_ones() as i32- board.black_knights.count_ones() as i32)
    + 100 * (board.white_pawns.count_ones() as i32 - board.black_pawns.count_ones() as i32)

    /* let mut val = 0;

    for rank in 0u8..8u8 {
        for file in 0u8..8u8 {
            let sq = Square::new_nocheck(file, rank);
            if let Some(p) = board.get(sq) {
                val += p.color.to_int() * (value(p.typ, depth) + piece_square_value(p, file, rank));
            }
        }
    }
    val */
}
