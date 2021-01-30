use std::cmp::max;

use crate::{
    board::{Board, Color},
    eval::evaluate,
    moves::enumerate_moves,
    moves::Move,
};

pub fn best_move(board: &Board, color: Color, depth: u32) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }

    let mut board = board.clone();

    let moves = enumerate_moves(&mut board, color);

    let mut best_move = None;
    let mut best_score = i32::MIN;
    for m in moves.iter() {

        m.make(&mut board);
        let sc = negamax(&mut board, color, depth-1) * color.to_int();
        if sc > best_score {
            best_move = Some(*m);
            best_score = sc;
        }
        m.unmake(&mut board);
    }

    Some((best_move?, best_score))
}

pub fn negamax(board: &mut Board, color: Color, depth: u32) -> i32 {
    //println!("{}:{:?}", depth, color);
    if depth == 0 {
        evaluate(board)
    } else {
        let moves = enumerate_moves(board, color);
        let mut max_score = std::i32::MIN;
        for m in moves.iter() {
            m.make(board);
            let score = -negamax(board, color.opposite(), depth - 1);
            // println!("{}\n", score);
            max_score = max(max_score, score);
            m.unmake(board);
        }

        max_score
    }
}
