use crate::board::{Board, PieceType};

fn value(typ: PieceType) -> i32 {
    match typ {
        PieceType::King => 0,
        PieceType::Queen => 1000,
        PieceType::Rook => 525,
        PieceType::Bishop => 350,
        PieceType::Knight => 350,
        PieceType::Pawn => 100,
    }
}

pub fn evaluate(board: &Board) -> i32 {

    // TODO: check
    
    board.iter().filter_map(|x| *x).fold(0, |acc, p| {
        acc + value(p.typ) * p.color.to_int()
    })
}