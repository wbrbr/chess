mod board;
mod moves;
mod square;
mod uci;
mod eval;
mod search;
mod game;

use std::{
    fs::File,
    io::{BufRead, Write},
};

use board::{Board, Color};
use eval::evaluate;
use moves::{Move, enumerate_moves};
use search::best_move;
use uci::{parse_command, Command};

/* pub struct Game {
    board: Board,
    next_move: Color,
} */

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut f = File::create("/home/wilhem/chess_log").unwrap();

    let mut game = None;

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
            Command::Position(g) => game = Some(g),
            Command::Go(_) => {
                let g = game.as_ref().expect("no position");
                let (m, score) = best_move(&g.board, g.player, 4).expect("no valid move");
                let str = format!("info score cp {}\nbestmove {}\n", score.to_string(), m.to_string());
                stdout.lock().write_all(str.as_bytes()).unwrap();
            }
            _ => {}
        }
    }
}
