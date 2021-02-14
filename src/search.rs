use crate::{board::{Color, Piece, PieceType}, eval::evaluate, game::Game, moves::Move, moves::enumerate_moves, square::Square};

pub fn best_move(game: &Game, depth: u32) -> Option<(Move, i32)> {
    if depth == 0 {
        return None;
    }

    let mut game = game.clone();

    let (mut best_score, best_m) = minmax(&mut game, 0, depth);
    best_score *= game.player.to_int(); 

    Some((best_m?, best_score))
}

pub fn perft(depth: u32) -> u32 {
    println!("not implemented");
    0
}


fn minmax(game: &Game, depth: u32, max_depth: u32) -> (i32, Option<Move>) {
    if depth == max_depth {
        (evaluate(&game.board, depth), None)
    } else {
        // TODO: remove the branches

        let mut best_score = match game.player {
            Color::White => -1000000,
            Color::Black => 1000000,
        };
        let mut best_m = None;

        let moves = enumerate_moves(game);

        for m in moves.iter() {
            let mut new_game = game.clone();
            m.make(&mut new_game);

            let (score, _) = minmax(&new_game, depth + 1, max_depth);

            match game.player {
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
        }

        (best_score, best_m.copied())
    }
}

/* 
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
} */