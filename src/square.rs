use crate::board::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8};

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

    pub fn offset(&self, dir: (i8, i8)) -> Option<Self> {
        let Square(file, rank) = *self;
        let (off_file, off_rank) = dir;
        let new_file = file as i8 + off_file;
        let new_rank = rank as i8 + off_rank;

        if new_file < 0 || new_file > 7 || new_rank < 0 || new_rank > 7 {
            None
        } else {
            Some(Square::new_nocheck(new_file as u8, new_rank as u8))
        }
    }

    pub fn file(&self) -> u8 {
        self.0
    }

    pub fn rank(&self) -> u8 {
        self.1
    }

    pub fn to_string(&self) -> String {
        let Square(file, rank) = *self;
        let file_c = match file {
            FILE_A => 'a',
            FILE_B => 'b',
            FILE_C => 'c',
            FILE_D => 'd',
            FILE_E => 'e',
            FILE_F => 'f',
            FILE_G => 'g',
            FILE_H => 'h',
            _ => unreachable!(),
        };

        let rank_c = match rank {
            RANK_1 => '1',
            RANK_2 => '2',
            RANK_3 => '3',
            RANK_4 => '4',
            RANK_5 => '5',
            RANK_6 => '6',
            RANK_7 => '7',
            RANK_8 => '8',
            _ => unreachable!(),
        };

        let mut res = String::with_capacity(2);
        res.push(file_c);
        res.push(rank_c);
        res
    }
}

#[test]
fn test_to_string() {
    let sq = Square::new_nocheck(FILE_A, RANK_7);
    assert_eq!(&sq.to_string(), "a7");

    let sq = Square::new_nocheck(FILE_A, RANK_8);
    assert_eq!(&sq.to_string(), "a8");
}