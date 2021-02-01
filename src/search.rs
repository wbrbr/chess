use crate::{board::{Board, Color, Piece, PieceType}, eval::evaluate, moves::Move, moves::enumerate_moves};

pub fn best_move(board: &Board, color: Color, depth: u32) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }

    let mut board = board.clone();

    let moves = enumerate_moves(&mut board, color);

    let (mut best_score, best_m) = minmax(&mut board, color, 0, depth, &moves);
    best_score *= color.to_int(); 

    Some((best_m?, best_score))
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

fn contains_king_capture(moves: &Vec<Move>) -> bool {
    moves.iter().any(|m| match m {
        Move::Normal {
            capture:
                Some(Piece {
                    typ: PieceType::King,
                    ..
                }),
            ..
        } => true,
        _ => false,
    })
}

fn minmax(board: &mut Board, color: Color, depth: u32, max_depth: u32, moves: &Vec<Move>) -> (i32, Option<Move>) {
    if depth == max_depth {
        (evaluate(board, depth), None)
    } else {
        // TODO: remove the branches

        let mut best_score = match color {
            Color::White => -1000000,
            Color::Black => 1000000,
        };
        let mut best_m = None;

        for m in moves.iter() {
            m.make(board);

            let opp_moves = enumerate_moves(board, color.opposite());

            // if the opponent has a (pseudo-legal) king capture, it means that
            // the current move is illegal so we continue
            if contains_king_capture(&opp_moves) {
                m.unmake(board);
                continue;
            }

            let (score, _) = minmax(board, color.opposite(), depth + 1, max_depth, &opp_moves);
            match color {
                Color::White => {
                    if score > best_score {
                        best_score = score;
                        best_m = Some(m);
                    }
                }
                Color::Black => {
                    if score < best_score {
                        best_score = score;
                        best_m = Some(m);
                    }
                }
            };
            m.unmake(board);
        }

        // we have no legal move
        // this is either a checkmate or a stalemate
        if !best_m.is_none() {
            let opp_moves = enumerate_moves(board, color.opposite());
            if contains_king_capture(&opp_moves) {
                // do nothing, the initial best_score is good
            } else {
                best_score = 0;
            }
        }
        (best_score, best_m.copied())
    }
}
