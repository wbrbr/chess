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
        let sc = minmax(&mut board, color.opposite(), 1, depth) * color.to_int();
        if sc > best_score {
            best_move = Some(*m);
            best_score = sc;
        }
        m.unmake(&mut board);
    }

    Some((best_move?, best_score))
}

/* struct Line {
    length: usize,
    data: [Option<Move>; 10],
}

impl Line {
    pub fn new() -> Self {
        Line {
            length: 0,
            data: [None; 10],
        }
    }

    pub fn to_string(&self) -> String {
        self.data.iter().fold(String::new(), |acc, x| match x {
            None => acc,
            Some(m) => acc + m.to_string().as_ref() + " ",
        })
    }
} */

fn minmax(board: &mut Board, color: Color, depth: u32, max_depth: u32) -> i32 {

    if depth == max_depth {
        evaluate(board, depth)
    } else {
        // TODO: remove the branches

        let moves = enumerate_moves(board, color);
        let mut best_score = match color {
            Color::White => -1000000,
            Color::Black => 1000000,
        };

        for m in moves.iter() {
            m.make(board);
            let score = minmax(board, color.opposite(), depth+1, max_depth);
            match color {
                Color::White => if score > best_score {
                    best_score = score;
                }
                Color::Black => if score < best_score {
                    best_score = score;
                }
            };
            m.unmake(board);
        }

        best_score
    }
}
