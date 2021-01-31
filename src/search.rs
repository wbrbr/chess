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

    for m in moves {
        m.make(&mut board);
        let sc_opt = minmax(&mut board, color.opposite(), 1, depth, m);
        if let Some(sc) = sc_opt {
            if sc * color.to_int() > best_score {
                best_move = Some(m);
                best_score = sc * color.to_int();
            }
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

fn minmax(
    board: &mut Board,
    color: Color,
    depth: u32,
    max_depth: u32,
    prev_move: Move,
) -> Option<i32> {
    if depth == max_depth {
        Some(evaluate(board, depth))
    } else {
        // TODO: remove the branches

        let moves = enumerate_moves(board, color);
        if !prev_move.is_legal(&moves) {
            return None;
        }

        let mut best_score = match color {
            Color::White => -1000000,
            Color::Black => 1000000,
        };

        for m in moves.iter() {
            m.make(board);

            // if None is returned it means that m is illegal so we ignore it (don't update the best score)
            if let Some(score) = minmax(board, color.opposite(), depth + 1, max_depth, *m) {
                match color {
                    Color::White => {
                        if score > best_score {
                            best_score = score;
                        }
                    }
                    Color::Black => {
                        if score < best_score {
                            best_score = score;
                        }
                    }
                };
            }
            m.unmake(board);
        }

        Some(best_score)
    }
}
