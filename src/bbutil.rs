use bitboard::bitboard_to_string;

mod bitboard;

pub fn main() {
    let n = std::env::args().skip(1).next().expect("no arg").parse().expect("not an int");

    println!("{}", &bitboard_to_string(n));
}