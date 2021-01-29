use crate::board::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H};

#[derive(Clone, Copy, Debug)]
pub struct Square(u8, u8);

impl Square {
    pub fn new_nocheck(file: u8, rank: u8) -> Square {
        Square(file, rank)
    }

    pub fn from_chars(file: char, rank: char) -> Option<Self> {
        let filei = match file {
            'a' => FILE_A,
            'b' => FILE_B,
            'c' => FILE_C,
            'd' => FILE_D,
            'e' => FILE_E,
            'f' => FILE_F,
            'g' => FILE_G,
            'h' => FILE_H,
            _ => return None,
        };

        let ranki = match rank {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        Some(Square(filei, ranki))
    }

    pub fn index(&self) -> usize {
        let Square(file, rank) = *self;
        8 * rank as usize + file as usize
    }
}