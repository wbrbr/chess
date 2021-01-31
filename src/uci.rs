use std::str::SplitAsciiWhitespace;

use crate::{board::{FILE_A, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, RANK_1, RANK_8}, square::Square};
use crate::{
    board::{Board, Color, Piece, PieceType},
    game::Game,
    moves::Move,
};

#[derive(Clone, Debug)]
pub enum Command {
    Uci,
    Debug(bool),
    IsReady,
    //SetOption()
    NewGame,
    Position(Game),
    Go(String), // TODO: something better
    Stop,
    PonderHit,
    Quit,
}

fn parse_move(board: &Board, str: &str) -> Option<Move> {
    let mut chars = str.chars();

    let from_file = chars.next()?;
    let from_rank = chars.next()?;
    let from = Square::from_chars(from_file, from_rank)?;

    let to_file = chars.next()?;
    let to_rank = chars.next()?;
    let to = Square::from_chars(to_file, to_rank)?;

    if board.get(from)?.typ == PieceType::King {
        match (from, to) {
            (Square(FILE_E, RANK_1), Square(FILE_G, RANK_1)) => {
                return Some(Move::Castling {
                    from: from,
                    to: to,
                    from_rook: Square::new_nocheck(FILE_H, RANK_1),
                    to_rook: Square::new_nocheck(FILE_F, RANK_1),
                    color: Color::White,
                })
            },
            (Square(FILE_E, RANK_1), Square(FILE_C, RANK_1)) => {
                return Some(Move::Castling {
                    from: from,
                    to: to,
                    from_rook: Square::new_nocheck(FILE_A, RANK_1),
                    to_rook: Square::new_nocheck(FILE_D, RANK_1),
                    color: Color::White,
                })
            },
            (Square(FILE_E, RANK_8), Square(FILE_C, RANK_8)) => {
                return Some(Move::Castling {
                    from: from,
                    to: to,
                    from_rook: Square::new_nocheck(FILE_A, RANK_8),
                    to_rook: Square::new_nocheck(FILE_D, RANK_8),
                    color: Color::Black,
                })
            },
            (Square(FILE_E, RANK_8), Square(FILE_G, RANK_8)) => {
                return Some(Move::Castling {
                    from: from,
                    to: to,
                    from_rook: Square::new_nocheck(FILE_H, RANK_8),
                    to_rook: Square::new_nocheck(FILE_F, RANK_8),
                    color: Color::Black,
                })
            }
            _ => {},
        }
    }

    let promotion = match chars.next() {
        Some('q') => Some(PieceType::Queen),
        Some('r') => Some(PieceType::Rook),
        Some('b') => Some(PieceType::Bishop),
        Some('n') => Some(PieceType::Knight),
        Some(_) => return None,
        None => None,
    };

    Some(Move::new(board, from, to, promotion))
}

fn parse_position(split: &mut SplitAsciiWhitespace) -> Option<Command> {
    let mut game = match split.next()? {
        "fen" => Game::from_fen(split)?,
        "startpos" => Game::new(),
        _ => return None,
    };

    if let Some(tok) = split.next() {
        if tok != "moves" {
            return None;
        }

        for m in split {
            let mov = parse_move(&game.board, m)?;
            mov.make(&mut game.board);
            game.player = game.player.opposite();
        }
    }

    Some(Command::Position(game))
}

pub fn parse_command(cmd: &str) -> Option<Command> {
    let mut split = cmd.split_ascii_whitespace();
    match split.next()? {
        "uci" => Some(Command::Uci),
        "isready" => Some(Command::IsReady),
        "ucinewgame" => Some(Command::NewGame),
        "go" => Some(Command::Go("".to_owned())),
        "quit" => Some(Command::Quit),
        "position" => parse_position(&mut split),
        _ => None,
    }
}

#[test]
fn parse_position1() {
    parse_command("position startpos moves e2e4").unwrap();
}

enum Response {
    IdName(String),
    IdAuthor(String),
    UciOk,
    ReadyOk,
    BestMove(Move),
}


#[test]
fn test_parse_castling() {
    let cmd = "position fen 8/8/8/8/8/8/8/4K2R b - - 11 34 moves e1g1";
    match parse_command(cmd) {
        Some(Command::Position(g)) => {
            assert_eq!(g.board.to_fen(), "8/8/8/8/8/8/8/5RK1");
        },
        _ => unreachable!()
    }

    let cmd = "position fen 8/8/8/8/8/8/8/R3K3 b - - 11 34 moves e1c1";
    match parse_command(cmd) {
        Some(Command::Position(g)) => {
            assert_eq!(g.board.to_fen(), "8/8/8/8/8/8/8/2KR4");
        },
        _ => unreachable!()
    }

    let cmd = "position fen 4k2r/8/8/8/8/8/8/8 b - - 11 34 moves e8g8";
    match parse_command(cmd) {
        Some(Command::Position(g)) => {
            assert_eq!(g.board.to_fen(), "5rk1/8/8/8/8/8/8/8");
        },
        _ => unreachable!()
    }

    let cmd = "position fen r3k3/8/8/8/8/8/8/8 b - - 11 34 moves e8c8";
    match parse_command(cmd) {
        Some(Command::Position(g)) => {
            assert_eq!(g.board.to_fen(), "2kr4/8/8/8/8/8/8/8");
        },
        _ => unreachable!()
    }
}