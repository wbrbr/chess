use crate::board::{Board, Color};
use crate::fen::board_from_fen;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub board: Board,
    pub player: Color,
}

impl Game {
    pub fn from_fen(str: &str) -> Option<Self> {
        let mut split = str.split(' ');
        let board_str = split.next()?;
        let board = board_from_fen(&mut board_str.chars().peekable())?;

        let player_str = split.next()?;

        let player = match player_str {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        Some(Game {
            board: board,
            player: player
        })
    }

    pub fn new() -> Self {
        Game {
            board: Board::starting_board(),
            player: Color::White,
        }
    }
}

#[test]
fn test_parse_game_start() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game {
        board: Board::starting_board(),
        player: Color::White,
    };
    assert_eq!(game, Game::from_fen(fen).unwrap());
}