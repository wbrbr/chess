use std::cmp::{max, min};

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

    let old_score = evaluate(&board);

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

    Some((best_move?, best_score-old_score))
}

pub fn negamax(board: &mut Board, color: Color, depth: u32) -> i32 {
    //println!("{}:{:?}", depth, color);
    if depth == 0 {
        evaluate(board)
    } else {
        // TODO: remove the branches

        let moves = enumerate_moves(board, color);
        let mut best_score = match color {
            Color::White => i32::MIN,
            Color::Black => i32::MAX,
        };

        for m in moves.iter() {
            m.make(board);
            let score = negamax(board, color.opposite(), depth - 1);
            best_score = match color {
                Color::White => max(best_score, score),
                Color::Black => min(best_score, score),
            };
            m.unmake(board);
        }

        best_score
    }
}
