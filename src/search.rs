use std::cmp::{max, min};

use crate::{board::{Color, Piece, PieceType}, eval::evaluate, game::Game, moves::Move, moves::enumerate_moves, square::Square};

pub fn best_move(game: &Game, depth: u32) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }

    let mut game = game.clone();

    let moves = enumerate_moves(&mut game);

    let (mut best_score, best_m) = minmax(&mut game, 0, depth, &moves);
    best_score *= game.player.to_int(); 

    Some((best_m?, best_score))
}

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

fn square_is_threatened(square: Square, moves: &Vec<Move>) -> bool {
    moves.iter().any(|m| match m {
        Move::Normal {
            to: sq, ..
        } if *sq == square => true,
        _ => false,
    })
}

fn minmax(game: &mut Game, depth: u32, max_depth: u32, moves: &Vec<Move>) -> (i32, Option<Move>) {
    if depth == max_depth {
        (evaluate(&game.board, depth), None)
    } else {
        // TODO: remove the branches

        let mut best_score = match game.player {
            Color::White => -1000000,
            Color::Black => 1000000,
        };
        let mut best_m = None;

        'lop: for m in moves.iter() {
            m.make(game);

            let opp_moves = enumerate_moves(game);

            // if the opponent has a (pseudo-legal) king capture, it means that
            // the current move is illegal so we continue
            if contains_king_capture(&opp_moves) {
                m.unmake(game);
                continue;
            }

            if let Move::Castling { from, to, ..} = m {
                assert_eq!(from.rank(), to.rank());
                let min_file = min(from.file(), to.file());
                let max_file = max(from.file(), to.file());

                for f in min_file..max_file {
                    if square_is_threatened(Square::new_nocheck(f, from.rank()), &opp_moves) {
                        m.unmake(game);
                        continue 'lop;
                    }
                }
            }

            let (score, _) = minmax(game, depth + 1, max_depth, &opp_moves);

            // we are now game.player.opposite() because m.make(game) changed the current player
            match game.player.opposite() {
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
            m.unmake(game);
        }

        // we have no legal move
        // this is either a checkmate or a stalemate
        if best_m.is_none() {
            game.player = game.player.opposite();
            let opp_moves = enumerate_moves(game);
            game.player = game.player.opposite();
            if contains_king_capture(&opp_moves) {
                // do nothing, the initial best_score is good
            } else {
                best_score = 0;
            }
        }
        (best_score, best_m.copied())
    }
}

pub fn perft(depth: u32) -> u32 {
    let mut game = Game::new();

    let moves = enumerate_moves(&game);

    perft_rec(&mut game, 0, depth, &moves)
}

fn perft_rec(game: &mut Game, depth: u32, max_depth: u32, moves: &Vec<Move>) -> u32 {
    if depth == max_depth {
        1
    } else {
        let mut n = 0;

        'lop: for m in moves {
            m.make(game);

            let opp_moves = enumerate_moves(game);

            if contains_king_capture(&opp_moves) {
                m.unmake(game);
                continue;
            }

            if let Move::Castling { from, to, ..} = m {
                assert_eq!(from.rank(), to.rank());
                let min_file = min(from.file(), to.file());
                let max_file = max(from.file(), to.file());

                for f in min_file..max_file {
                    if square_is_threatened(Square::new_nocheck(f, from.rank()), &opp_moves) {
                        m.unmake(game);
                        continue 'lop;
                    }
                }
            }

            let x = perft_rec(game, depth+1, max_depth, &opp_moves);
            if depth == 0 {
                println!("{}: {}", m.to_string(), x);
            }

            n += x;

            m.unmake(game);
        }
        n
    }
}