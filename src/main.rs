mod board;
mod moves;
mod square;
mod uci;
mod eval;

use std::{
    fs::File,
    io::{BufRead, Write},
};

use board::{Board, Color};
use eval::evaluate;
use moves::{Move, enumerate_moves};
use rand::Rng;
use uci::{parse_command, Command};

/* pub struct Game {
    board: Board,
    next_move: Color,
} */

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut f = File::create("/home/wilhem/chess_log").unwrap();

    let mut board = Board::starting_board();

    let mut rng = rand::thread_rng();

    for l in stdin.lock().lines().map(|l| l.unwrap()) {
        f.write_all(l.as_bytes()).unwrap();
        f.write_all("\n".as_bytes()).unwrap();
        let cmd = parse_command(&l);
        write!(&mut f, "{:?}\n", cmd).unwrap();
        match cmd.unwrap() {
            Command::Uci => stdout
                .lock()
                .write_all("id name chess\nid author Wilhem Barbier\nuciok\n".as_bytes())
                .unwrap(),
            Command::IsReady => stdout.lock().write_all("readyok\n".as_bytes()).unwrap(),
            Command::Quit => return,
            Command::Position(b) => board = b,
            Command::Go(_) => {
                let moves: Vec<Move> = enumerate_moves(&board, Color::Black).into_iter().filter(|m| m.is_legal(&mut board)).collect();
                let m = moves.iter().max_by_key(|m| {
                    m.make(&mut board);
                    let ret = -1 * evaluate(&board);
                    m.unmake(&mut board);
                    ret
                }).expect("no valid moves");
                let str = format!("bestmove {}\n", m.to_string());
                stdout.lock().write_all(str.as_bytes()).unwrap();
            }
            _ => {}
        }
    }
}
