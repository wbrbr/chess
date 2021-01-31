use crate::{board::{Board, Color, PieceType}};

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
    let mut white_lost = true;
    let mut black_lost = true;

    board.iter().filter_map(|x| match x {
        Some(p) if p.typ == PieceType::King => Some(*p),
        _ => None
    }).for_each(|p| match p.color {
        Color::White => white_lost = false,
        Color::Black => black_lost = false,
    });

    if white_lost {
        // println!("white lost");
        return i32::MIN;
    } else if black_lost {
        // println!("black lost: {}", board.to_fen());
        return i32::MAX;
    }

    board.iter().filter_map(|x| *x).fold(0, |acc, p| {
        acc + value(p.typ) * p.color.to_int()
    })
}