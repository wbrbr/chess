use std::str::SplitAsciiWhitespace;

use crate::square::Square;
use crate::{
    board::{
        Board, Color, Piece, PieceType
    },
    game::Game,
};

#[derive(Clone, Copy, Debug)]
struct Move {
    from: Square,
    to: Square,
    promotion: Option<PieceType>,
}

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

fn parse_move(str: &str) -> Option<Move> {
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

    Some(Move {
        from: from,
        to: to,
        promotion: promotion,
    })
}

fn exec_move(board: &mut Board, mov: Move) {
    let orig_piece = board.get(mov.from).expect("received invalid move");
    board.set(mov.from, None);

    let final_piece = match mov.promotion {
        Some(typ) => Piece::new(typ, orig_piece.color),
        None => orig_piece,
    };

    board.set(mov.to, Some(final_piece));
}

fn parse_position_fen(split: &mut SplitAsciiWhitespace) -> Option<Game> {
    // it's kind of stupid to make a join here that will be split again later but whatever
    let str = split.fold(String::new(), |acc, x| {
        let mut res = acc;
        if !res.is_empty() {
            res.push(' ');
        }
        res += x;
        res
    });

    Game::from_fen(&str)
}

fn parse_position(split: &mut SplitAsciiWhitespace) -> Option<Command> {
    let mut game = match split.next()? {
        "fen" => parse_position_fen(split)?,
        "startpos" => Game::new(),
        _ => return None,
    };

    if let Some(tok) = split.next() {
        if tok != "moves" {
            return None;
        }

        for m in split {
            exec_move(&mut game.board, parse_move(m)?);
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
