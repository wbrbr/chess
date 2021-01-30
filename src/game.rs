use crate::board::{Board, Color};

#[derive(Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub player: Color,
}