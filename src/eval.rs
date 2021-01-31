use crate::{board::{Board, PieceType}};

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

pub fn evaluate(board: &Board, depth: u32) -> i32 {

    // TODO: check
    board.iter().filter_map(|x| *x).fold(0, |acc, p| {
        acc + value(p.typ, depth) * p.color.to_int()
    })
}