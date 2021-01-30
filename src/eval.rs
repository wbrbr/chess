use crate::{board::{Board, Color, PieceType}, moves::{enumerate_legal_moves, enumerate_moves}};

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

pub fn evaluate(board: &mut Board) -> i32 {

    // TODO: check
    if enumerate_legal_moves(board, Color::White).len() == 0 {
        return i32::MIN;
    } else if enumerate_legal_moves(board, Color::Black).len() == 0 {
        return i32::MAX;
    }

    board.iter().filter_map(|x| *x).fold(0, |acc, p| {
        acc + value(p.typ) * p.color.to_int()
    })
}