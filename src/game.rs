use crate::board::{Board, Color};
use crate::fen::board_from_fen;

pub type CastlingRights = u8;
pub const WHITE_QUEENSIDE: CastlingRights = 0b0001;
pub const BLACK_QUEENSIDE: CastlingRights = 0b0010;
pub const WHITE_KINGSIDE: CastlingRights = 0b0100;
pub const BLACK_KINGSIDE: CastlingRights = 0b1000;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub player: Color,
    pub castling_rights: CastlingRights,
}

impl Game {
    pub fn from_fen<'a, I: Iterator<Item=&'a str>>(split: &mut I) -> Option<Self> {
        let board_str = split.next()?;
        let board = board_from_fen(&mut board_str.chars().peekable())?;

        let player_str = split.next()?;

        let player = match player_str {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        let mut castling_rights: CastlingRights = 0;
        for c in split.next()?.chars() {
            match c {
                '-' => break,
                'K' => castling_rights |= WHITE_KINGSIDE,
                'Q' => castling_rights |= WHITE_QUEENSIDE,
                'k' => castling_rights |= BLACK_KINGSIDE,
                'q' => castling_rights |= BLACK_QUEENSIDE,
                _ => return None,
            }
        } 

        split.next()?; // en passant
        split.next()?; // halfmove counter
        split.next()?; // fullmove counter

        Some(Game {
            board: board,
            player: player,
            castling_rights: castling_rights,
        })
    }

    pub fn new() -> Self {
        Game {
            board: Board::new(),
            player: Color::White,
            castling_rights: 0b1111,
        }
    }
}

#[test]
fn test_parse_game_start() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game {
        board: Board::new(),
        player: Color::White,
        castling_rights: 0b1111,
    };
    assert_eq!(game, Game::from_fen(&mut fen.split_ascii_whitespace()).unwrap());
}

#[test]
fn test_parse_castling_rights() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
    assert_eq!(0, Game::from_fen(&mut fen.split_ascii_whitespace()).unwrap().castling_rights);
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kk - 0 1";
    assert_eq!(BLACK_KINGSIDE | WHITE_KINGSIDE, Game::from_fen(&mut fen.split_ascii_whitespace()).unwrap().castling_rights);
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    assert_eq!(WHITE_QUEENSIDE | BLACK_QUEENSIDE | BLACK_KINGSIDE | WHITE_KINGSIDE, Game::from_fen(&mut fen.split_ascii_whitespace()).unwrap().castling_rights);
}