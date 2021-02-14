use crate::bitboard::{A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8, Bitboard};

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

impl Color {
    pub fn opposite(&self) -> Self {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn to_int(&self) -> i32 {
        match *self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            Color::White => "White",
            Color::Black => "Black",
        }
    }
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

    pub fn to_string(&self) -> char {
        match (self.typ, self.color) {
            (PieceType::King, Color::White) => 'K',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::King, Color::Black) => 'k',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Pawn, Color::Black) => 'p',
        }
    }
}

/* #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    pub board: [Option<Piece>; 64],
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
    assert_eq!(
        board.get(sq),
        Some(Piece {
            typ: PieceType::Pawn,
            color: Color::White
        })
    );

    let sq = Square::new_nocheck(FILE_A, RANK_7);
    assert_eq!(sq.index(), 48);
    println!("{:?}", board.board);
    assert_eq!(
        board.get(sq),
        Some(Piece {
            typ: PieceType::Pawn,
            color: Color::Black
        })
    );
} */


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub white_king: Bitboard,
    pub white_queens: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_knights: Bitboard,
    pub white_pawns: Bitboard,
    pub white_pieces: Bitboard,

    pub black_king: Bitboard,
    pub black_queens: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_knights: Bitboard,
    pub black_pawns: Bitboard,
    pub black_pieces: Bitboard,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            white_king: 0,
            white_queens: 0,
            white_rooks: 0,
            white_bishops: 0,
            white_knights: 0,
            white_pawns: 0,
            white_pieces: 0,
            black_king: 0,
            black_queens: 0,
            black_rooks: 0,
            black_bishops: 0,
            black_knights: 0,
            black_pawns: 0,
            black_pieces: 0,
        }
    }
    pub fn new() -> Self {
        const WHITE_KING: Bitboard = 1 << E1;
        const WHITE_QUEENS: Bitboard = 1 << D1;
        const WHITE_ROOKS: Bitboard = (1 << A1) |  (1 << H1);
        const WHITE_KNIGHTS: Bitboard = (1 << B1) | (1 << G1);
        const WHITE_BISHOPS: Bitboard = (1 << C1) | (1 << F1);
        const WHITE_PAWNS: Bitboard = (1 << A2) | (1 << B2) | (1 << C2) | (1 << D2) | (1 << E2) | (1 << F2) | (1 << G2) | (1  << H2); 

        const BLACK_KING: Bitboard = 1 << E8;
        const BLACK_QUEENS: Bitboard = 1 << D8;
        const BLACK_ROOKS: Bitboard = (1 << A8) | (1 << H8);
        const BLACK_KNIGHTS: Bitboard = (1 << B8) | (1 << G8);
        const BLACK_BISHOPS: Bitboard = (1 << C8) | (1 << F8);
        const BLACK_PAWNS: Bitboard = (1 << A7) | (1 << B7) | (1 << C7) | (1 << D7) | (1 << E7) | (1 << F7) | (1 << G7) | (1 << H7);

        Board {
            white_king: WHITE_KING,
            white_queens: WHITE_QUEENS,
            white_rooks: WHITE_ROOKS,
            white_bishops: WHITE_BISHOPS,
            white_knights: WHITE_KNIGHTS,
            white_pawns: WHITE_PAWNS,
            white_pieces: WHITE_KING | WHITE_QUEENS | WHITE_ROOKS | WHITE_BISHOPS | WHITE_KNIGHTS | WHITE_PAWNS,

            black_king: BLACK_KING,
            black_queens: BLACK_QUEENS,
            black_rooks: BLACK_ROOKS,
            black_bishops: BLACK_BISHOPS,
            black_knights: BLACK_KNIGHTS,
            black_pawns: BLACK_PAWNS,
            black_pieces: BLACK_KING | BLACK_QUEENS | BLACK_ROOKS | BLACK_BISHOPS | BLACK_KNIGHTS | BLACK_PAWNS,
        }
    }

    pub fn set(&mut self, square: u8, piece: Option<Piece>) {
        assert!(square < 64);

        let i = 1 << square;

        match piece {
            Some(Piece { typ: PieceType::King, color: Color::White }) => self.white_king |= i,
            Some(Piece { typ: PieceType::Queen, color: Color::White}) => self.white_queens |= i,
            Some(Piece { typ: PieceType::Rook, color: Color::White}) => self.white_rooks |= i,
            Some(Piece { typ: PieceType::Bishop, color: Color::White}) => self.white_bishops |= i,
            Some(Piece { typ: PieceType::Knight, color: Color::White}) => self.white_knights |= i,
            Some(Piece { typ: PieceType::Pawn, color: Color::White}) => self.white_pawns |= i,
            Some(Piece { typ: PieceType::King, color: Color::Black }) => self.black_king |= i,
            Some(Piece { typ: PieceType::Queen, color: Color::Black }) => self.black_queens |= i,
            Some(Piece { typ: PieceType::Rook, color: Color::Black }) => self.black_rooks |= i,
            Some(Piece { typ: PieceType::Bishop, color: Color::Black }) => self.black_bishops |= i,
            Some(Piece { typ: PieceType::Bishop, color: Color::Black }) => self.black_knights |= i,
            Some(Piece { typ: PieceType::Pawn, color: Color::Black }) => self.black_pawns |= i,
            None => {
                let mask = !i;
                self.white_king &= mask;
                self.white_queens &= mask;
                self.white_rooks &= mask;
                self.white_bishops &= mask;
                self.white_knights &= mask;
                self.white_pawns &= mask;
                self.black_king &= mask;
                self.black_queens &= mask;
                self.black_rooks &= mask;
                self.black_bishops &= mask;
                self.black_knights &= mask;
                self.black_pawns &= mask;
            }
            _ => unreachable!(),
        }

        self.update_sets();
    }

    pub fn get(&self, square: u8) -> Option<Piece> {
        assert!(square < 64);

        let i = 1 << square;

        if self.white_pieces & i != 0 {
            if self.white_king & i != 0 {
                Some(Piece::new(PieceType::King, Color::White))
            } else if self.white_queens & i != 0 {
                Some(Piece::new(PieceType::Queen, Color::White))
            } else if self.white_rooks & i != 0 {
                Some(Piece::new(PieceType::Rook, Color::White))
            } else if self.white_bishops & i != 0 {
                Some(Piece::new(PieceType::Bishop, Color::White))
            } else if self.white_knights & i != 0 {
                Some(Piece::new(PieceType::Knight, Color::White))
            } else if self.white_pawns & i != 0 {
                Some(Piece::new(PieceType::Pawn, Color::White))
            } else {
                unreachable!()
            }
        } else if self.black_pieces & i != 0 {
            if self.black_king & i != 0 {
                Some(Piece::new(PieceType::King, Color::Black))
            } else if self.black_queens & i != 0 {
                Some(Piece::new(PieceType::Queen, Color::Black))
            } else if self.black_rooks & i != 0 {
                Some(Piece::new(PieceType::Rook, Color::Black))
            } else if self.black_bishops & i != 0 {
                Some(Piece::new(PieceType::Bishop, Color::Black))
            } else if self.black_knights & i != 0 {
                Some(Piece::new(PieceType::Knight, Color::Black))
            } else if self.black_pawns & i != 0 {
                Some(Piece::new(PieceType::Pawn, Color::Black))
            } else {
                unreachable!()
            }

        } else {
            None
        }
    }

    fn update_sets(&mut self) {
        self.white_pieces = self.white_king | self.white_queens | self.white_rooks | self.white_bishops | self.white_knights | self.white_pawns;
        self.black_pieces = self.black_king | self.black_queens | self.black_rooks | self.black_bishops | self.black_knights | self.black_pawns;
    }

    pub fn to_fen(&self) -> String {
        let mut res = String::new();
        for rank in (0..8).rev() {
            let mut empty_counter = 0u8;
            for file in 0..8 {
                let i = 1 << (rank * 8 + file);

                if (self.black_pieces | self.white_pieces) & i != 0 {
                    if empty_counter > 0 {
                        // this is inefficient since we know that 1 <= empty_counter <= 7
                        res += &empty_counter.to_string();
                    }

                    empty_counter = 0;

                    if self.white_king & i != 0 {
                        res.push('K');
                    } else if self.white_queens & i != 0 {
                        res.push('Q');
                    } else if self.white_rooks & i != 0 { 
                        res.push('R');
                    } else if self.white_bishops & i != 0 {
                        res.push('B');
                    } else if self.white_knights & i != 0 {
                        res.push('N');
                    } else if self.white_pawns & i != 0 {
                        res.push('P');
                    } else if self.black_king & i != 0 {
                        res.push('k');
                    } else if self.black_queens & i != 0 {
                        res.push('q');
                    } else if self.black_rooks & i != 0 {
                        res.push('r');
                    } else if self.black_bishops & i != 0 {
                        res.push('b');
                    } else if self.black_knights & i != 0 {
                        res.push('n');
                    } else if self.black_pawns & i != 0 {
                        res.push('p');
                    } else {
                        unreachable!();
                    }
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

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        for rank in (0..7u8).rev() {
            for file in 0..7u8 {
                let i = rank * 8 + file;
                res.push(match self.get(i) {
                    Some(p) => p.to_string(),
                    None => '.',
                });
            }
            res.push('\n');
        }

        res
    }
}

#[test]
fn test_new_start_fen() {
    let board = Board::new();
    assert_eq!(
        &board.to_fen(),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
}