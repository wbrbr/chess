pub type Bitboard = u64;

pub const A1: u64 = 1 << 0;
pub const B1: u64 = 1 << 1;
pub const C1: u64 = 1 << 2;
pub const D1: u64 = 1 << 3;
pub const E1: u64 = 1 << 4;
pub const F1: u64 = 1 << 5;
pub const G1: u64 = 1 << 6;
pub const H1: u64 = 1 << 7;
pub const A2: u64 = 1 << 8;
pub const B2: u64 = 1 << 9;
pub const C2: u64 = 1 << 10;
pub const D2: u64 = 1 << 11;
pub const E2: u64 = 1 << 12;
pub const F2: u64 = 1 << 13;
pub const G2: u64 = 1 << 14;
pub const H2: u64 = 1 << 15;
pub const A7: u64 = 1 << 48;
pub const B7: u64 = 1 << 49;
pub const C7: u64 = 1 << 50;
pub const D7: u64 = 1 << 51;
pub const E7: u64 = 1 << 52;
pub const F7: u64 = 1 << 53;
pub const G7: u64 = 1 << 54;
pub const H7: u64 = 1 << 55;
pub const A8: u64 = 1 << 56;
pub const B8: u64 = 1 << 57;
pub const C8: u64 = 1 << 58;
pub const D8: u64 = 1 << 59;
pub const E8: u64 = 1 << 60;
pub const F8: u64 = 1 << 61;
pub const G8: u64 = 1 << 62;
pub const H8: u64 = 1 << 63;

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