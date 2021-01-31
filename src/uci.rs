use std::str::SplitAsciiWhitespace;

use crate::square::Square;
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

    let promotion = match chars.next() {
        Some('q') => Some(PieceType::Queen),
        Some('r') => Some(PieceType::Rook),
        Some('b') => Some(PieceType::Bishop),
        Some('n') => Some(PieceType::Knight),
        Some(_) => return None,
        None => None,
    };

    Some(Move::new(from, to, promotion));
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
