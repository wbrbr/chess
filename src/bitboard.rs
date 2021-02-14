pub type Bitboard = u64;

pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;
pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;
pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;
pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

fn bitscan_forward(n: u64) -> u8 {
    assert_ne!(n, 0);
    n.trailing_zeros() as u8
}

pub struct BitboardIter {
    bitboard: Bitboard,
}

impl Iterator for BitboardIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard == 0 {
            None
        } else {
            let index = bitscan_forward(self.bitboard);
            self.bitboard &= self.bitboard - 1;
            Some(index)
        }
    }
}

pub fn bitboard_iter(bitboard: Bitboard) -> BitboardIter {
    BitboardIter {
        bitboard
    }
}

pub fn bitboard_to_string(bitboard: Bitboard) -> String {
    let mut str = String::new();

    for rank in (0..8u8).rev() {
        for file in 0..8u8 {
            let index = rank * 8 + file;
            let x = 1 << index;
            if bitboard & x != 0 {
                str += "0";
            } else {
                str += ".";
            }
        }
        str += "\n";
    }

    str
}

#[test]
fn test_bitboard_iter() {
    let mut iter = bitboard_iter(7);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

const NOT_A_FILE: Bitboard = 0xfefefefefefefefe;
const NOT_H_FILE: Bitboard = 0x7f7f7f7f7f7f7f7f;

pub fn shift_east(board: Bitboard) -> Bitboard {
    (board << 1) & NOT_A_FILE
}

pub fn shift_west(board: Bitboard) -> Bitboard {
    (board >> 1) & NOT_H_FILE
}

pub fn shift_north(board: Bitboard) -> Bitboard {
    board << 8
}

pub fn shift_south(board: Bitboard) -> Bitboard {
    board >> 8
}

pub fn shift_north_west(board: Bitboard) -> Bitboard {
    (board << 7) & NOT_H_FILE
}

pub fn shift_north_east(board: Bitboard) -> Bitboard {
    (board << 9) & NOT_A_FILE
}

pub fn shift_south_west(board: Bitboard) -> Bitboard {
    (board >> 9) & NOT_H_FILE
}

pub fn shift_south_east(board: Bitboard) -> Bitboard {
    (board >> 7) & NOT_A_FILE
}