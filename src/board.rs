use crate::square::Square;

pub const FILE_A: u8 = 0;
pub const FILE_B: u8 = 1;
pub const FILE_C: u8 = 2;
pub const FILE_D: u8 = 3;
pub const FILE_E: u8 = 4;
pub const FILE_F: u8 = 5;
pub const FILE_G: u8 = 6;
pub const FILE_H: u8 = 7;

pub const RANK_1: u8 = 0;
pub const RANK_2: u8 = 1;
pub const RANK_3: u8 = 2;
pub const RANK_4: u8 = 3;
pub const RANK_5: u8 = 4;
pub const RANK_6: u8 = 5;
pub const RANK_7: u8 = 6;
pub const RANK_8: u8 = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub typ: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(typ: PieceType, color: Color) -> Self {
        Piece {
            typ: typ,
            color: color,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    board: [Option<Piece>; 64],
}

impl Board {
    pub fn starting_board() -> Self {
        let mut board = [None; 64];

        board[0] = Some(Piece::new(PieceType::Rook, Color::White));
        board[1] = Some(Piece::new(PieceType::Knight, Color::White));
        board[2] = Some(Piece::new(PieceType::Bishop, Color::White));
        board[3] = Some(Piece::new(PieceType::Queen, Color::White));
        board[4] = Some(Piece::new(PieceType::King, Color::White));
        board[5] = Some(Piece::new(PieceType::Bishop, Color::White));
        board[6] = Some(Piece::new(PieceType::Knight, Color::White));
        board[7] = Some(Piece::new(PieceType::Rook, Color::White));

        board[56] = Some(Piece::new(PieceType::Rook, Color::Black));
        board[57] = Some(Piece::new(PieceType::Knight, Color::Black));
        board[58] = Some(Piece::new(PieceType::Bishop, Color::Black));
        board[59] = Some(Piece::new(PieceType::Queen, Color::Black));
        board[60] = Some(Piece::new(PieceType::King, Color::Black));
        board[61] = Some(Piece::new(PieceType::Bishop, Color::Black));
        board[62] = Some(Piece::new(PieceType::Knight, Color::Black));
        board[63] = Some(Piece::new(PieceType::Rook, Color::Black));

        for file in 0..8 {
            board[8 + file] = Some(Piece::new(PieceType::Pawn, Color::White));
            board[48 + file] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        Board { board: board }
    }

    /// Should I check here ?
    pub fn get(&self, square: Square) -> Option<Piece> {
        self.board[square.index()]
    }

    pub fn set(&mut self, square: Square, piece: Option<Piece>) {
        self.board[square.index()] = piece;
    }

    pub fn to_fen(&self) -> String {
        let mut res = String::new();
        for rank in (0..8).rev() {
            let mut empty_counter = 0u8;
            for file in 0..8 {
                if let Some(piece) = self.get(Square::new_nocheck(file, rank)) {
                    if empty_counter > 0 {
                        // this is inefficient since we know that 1 <= empty_counter <= 7
                        res += &empty_counter.to_string();
                    }

                    empty_counter = 0;

                    res.push(match (piece.typ, piece.color) {
                        (PieceType::King, Color::White) => 'K',
                        (PieceType::King, Color::Black) => 'k',
                        (PieceType::Queen, Color::White) => 'Q',
                        (PieceType::Queen, Color::Black) => 'q',
                        (PieceType::Rook, Color::White) => 'R',
                        (PieceType::Rook, Color::Black) => 'r',
                        (PieceType::Bishop, Color::White) => 'B',
                        (PieceType::Bishop, Color::Black) => 'b',
                        (PieceType::Knight, Color::White) => 'N',
                        (PieceType::Knight, Color::Black) => 'n',
                        (PieceType::Pawn, Color::White) => 'P',
                        (PieceType::Pawn, Color::Black) => 'p',
                    });
                } else {
                    empty_counter += 1;
                }
            }
            if empty_counter > 0 {
                res += &empty_counter.to_string();
            }

            if rank > 0 {
                res += "/";
            }
        }
        res
    }

    pub fn contains_ally(&self, square: Square, color: Color) -> bool {
        match self.get(square) {
            Some(p) if p.color == color => true,
            _ => false,
        }
    }
}

#[test]
fn test_start_fen() {
    let board = Board::starting_board();
    assert_eq!(
        &board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
}

#[test]
fn test_e4_fen() {
    let mut board = Board::starting_board();
    board.set(Square::new_nocheck(FILE_E, RANK_2), None);
    board.set(
        Square::new_nocheck(FILE_E, RANK_4),
        Some(Piece::new(PieceType::Pawn, Color::White)),
    );
    assert_eq!(
        &board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
    );
}

#[test]
fn test_start() {
    let board = Board::starting_board();

    let sq = Square::new_nocheck(FILE_E, RANK_2);
    assert_eq!(board.get(sq), Some(Piece { typ: PieceType::Pawn, color: Color::White}));

    let sq = Square::new_nocheck(FILE_A, RANK_7);
    assert_eq!(sq.index(), 48);
    println!("{:?}", board.board);
    assert_eq!(board.get(sq), Some(Piece { typ: PieceType::Pawn, color: Color::Black}));
}