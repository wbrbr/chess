const FILE_A: usize = 0;
const FILE_B: usize = 1;
const FILE_C: usize = 2;
const FILE_D: usize = 3;
const FILE_E: usize = 4;
const FILE_F: usize = 5;
const FILE_G: usize = 6;
const FILE_H: usize = 7;

const RANK_1: usize = 0;
const RANK_2: usize = 1;
const RANK_3: usize = 2;
const RANK_4: usize = 3;
const RANK_5: usize = 4;
const RANK_6: usize = 5;
const RANK_7: usize = 6;
const RANK_8: usize = 7;

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    typ: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(typ: PieceType, color: Color) -> Self {
        Piece {
            typ: typ,
            color: color,
        }
    }
}
pub struct Board {
    board: [Option<Piece>; 64],
}

pub struct Game {
    board: Board,
    next_move: Color,
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
    pub fn get(&self, file: usize, rank: usize) -> Option<Piece> {
        self.board[8 * rank + file]
    }

    pub fn set(&mut self, file: usize, rank: usize, piece: Option<Piece>) {
        self.board[8 * rank + file] = piece;
    }

    pub fn to_fen(&self) -> String {
        let mut res = String::new();
        for rank in (0..8).rev() {
            let mut empty_counter = 0u8;
            for file in 0..8 {
                if let Some(piece) = self.get(file, rank) {
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
    board.set(FILE_E, RANK_2, None);
    board.set(FILE_E, RANK_4, Some(Piece::new(PieceType::Pawn, Color::White)));
    assert_eq!(
        &board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
    );
}

fn main() {
    let board = Board::starting_board();
    println!("{}", board.to_fen());
}
