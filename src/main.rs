mod board;
mod uci;
mod square;

use std::{
    fs::File,
    io::{BufRead, Write},
};

use uci::{parse_command, Command};

/* pub struct Game {
    board: Board,
    next_move: Color,
} */

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut f = File::create("/home/wilhem/chess_log").unwrap();
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
            Command::Position(_) => {},
            Command::Go(_) => stdout.lock().write_all("bestmove e7e5\n".as_bytes()).unwrap(),
            _ => {}
        }
    }
}
