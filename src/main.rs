mod board;
mod eval;
mod fen;
mod game;
mod moves;
mod search;
mod square;
mod uci;
mod bitboard;

use std::{
    fs::File,
    io::{BufRead, Write},
};

use search::{best_move, perft};
use uci::{parse_command, Command};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut f = File::create("/home/wilhem/chess_log").unwrap();

    let mut game = None;

    for l in stdin.lock().lines().map(|l| l.unwrap()) {
        f.write_all(l.as_bytes()).unwrap();
        f.write_all("\n".as_bytes()).unwrap();
        let cmd = parse_command(&l);
        write!(&mut f, "{:?}\n", cmd).unwrap();
        match cmd {
            Some(Command::Uci) => stdout
                .lock()
                .write_all("id name chess\nid author Wilhem Barbier\nuciok\n".as_bytes())
                .unwrap(),
            Some(Command::IsReady) => stdout.lock().write_all("readyok\n".as_bytes()).unwrap(),
            Some(Command::Quit) => return,
            Some(Command::Position(g)) => game = Some(g),
            Some(Command::Go(_)) => {
                let g = game.as_ref().expect("no position");
                println!("{}", g.board.to_string());
                let (m, score) = best_move(&g, 6).expect("no valid move");
                let str = format!(
                    "info score cp {}\nbestmove {}\n",
                    score.to_string(),
                    m.to_string()
                );
                stdout.lock().write_all(str.as_bytes()).unwrap();
            }
            Some(Command::Perft(depth)) => {
                println!("{}", perft(depth));
            }
            Some(Command::NewGame) => {}
            None => {}
        }
    }
}
