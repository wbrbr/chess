use std::cmp::max;

use crate::{
    board::{Board, Color},
    eval::evaluate,
    moves::enumerate_moves,
    moves::Move,
};

pub fn best_move(board: &Board, color: Color, depth: u32) -> Option<Move> {
    let mut board = board.clone();

    let moves: Vec<Move> = enumerate_moves(&mut board, color)
        .into_iter()
        .filter(|m| m.is_legal(&mut board)).collect();

    moves.into_iter().max_by_key(|m| {
        m.make(&mut board);
        let ret = negamax(&mut board, color, depth) * color.to_int();
        m.unmake(&mut board);
        ret
    })
}

pub fn negamax(board: &mut Board, color: Color, depth: u32) -> i32 {
    if depth == 0 {
        evaluate(board)
    } else {
        let moves = enumerate_moves(board, color);
        let mut max_score = std::i32::MIN;
        for m in moves.iter() {
            m.make(board);
            let score = -negamax(board, color.opposite(), depth - 1);
            max_score = max(max_score, score);
            m.unmake(board);
        }

        max_score
    }
}
