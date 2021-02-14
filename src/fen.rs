use std::iter::Peekable;

use crate::{
    board::{Board, Color, Piece, PieceType},
    square::Square,
};

fn piece(c: char) -> Option<Piece> {
    match c {
        'K' => Some(Piece::new(PieceType::King, Color::White)),
        'k' => Some(Piece::new(PieceType::King, Color::Black)),
        'Q' => Some(Piece::new(PieceType::Queen, Color::White)),
        'q' => Some(Piece::new(PieceType::Queen, Color::Black)),
        'R' => Some(Piece::new(PieceType::Rook, Color::White)),
        'r' => Some(Piece::new(PieceType::Rook, Color::Black)),
        'N' => Some(Piece::new(PieceType::Knight, Color::White)),
        'n' => Some(Piece::new(PieceType::Knight, Color::Black)),
        'B' => Some(Piece::new(PieceType::Bishop, Color::White)),
        'b' => Some(Piece::new(PieceType::Bishop, Color::Black)),
        'P' => Some(Piece::new(PieceType::Pawn, Color::White)),
        'p' => Some(Piece::new(PieceType::Pawn, Color::Black)),
        _ => None,
    }
}

fn ranki<I>(chars: &mut Peekable<I>, rank: u8, board: &mut Board) -> Option<()>
where
    I: Iterator<Item = char>,
{
    let mut file = 0u8;
    loop {
        match chars.next() {
            None | Some('/') => break,
            Some(c) if c.is_ascii_digit() => {
                let digit = c.to_digit(10)?;
                if digit == 0 || digit > 8 {
                    return None;
                }
                file += digit as u8;
            }
            Some(c) => {
                if let Some(p) = piece(c) {
                    let sq = rank * 8 + file;
                    board.set(sq, Some(p));
                    file += 1;
                }
            }
        }
    }
    if file == 8 {
        Some(())
    } else {
        None
    }
}

pub fn board_from_fen<I>(chars: &mut Peekable<I>) -> Option<Board>
where
    I: Iterator<Item = char>,
{
    let mut rank = 7u8;

    let mut board = Board::empty();

    loop {
        ranki(chars, rank, &mut board)?;

        if rank == 0 {
            break;
        } else {
            rank -= 1;
        }
    }

    Some(board)
}

#[test]
fn test_parse_board_start() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    assert_eq!(Board::new(), board_from_fen(&mut fen.chars().peekable()).unwrap());
}