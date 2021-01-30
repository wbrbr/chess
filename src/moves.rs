use crate::{
    board::{Board, Color, Piece, PieceType, RANK_2, RANK_4},
    square::{Square},
};

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub capture: Option<Piece>,
    // TODO: castling, promotion, en passant, ...
}


impl Move {
    pub fn new(board: &Board, from: Square, to: Square) -> Self {
        let piece = board.get(from).expect("the from square is empty");
        let capture = board.get(to);

        Move {
            from: from,
            to: to,
            piece: piece,
            capture: capture
        }
    }

    pub fn to_string(&self) -> String {
        self.from.to_string() + &self.to.to_string()
    }

    pub fn make(&self, board: &mut Board) {
        board.set(self.from, None);
        board.set(self.to, Some(self.piece));
    }

    pub fn unmake(&self, board: &mut Board) {
        board.set(self.from, Some(self.piece));
        board.set(self.to, self.capture);
    }
}

/// Generate pseudo legal moves
pub fn enumerate_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = Vec::with_capacity(1000);
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new_nocheck(file, rank);
            if let Some(piece) = board.get(sq) {
                if piece.color == color {
                    match piece.typ {
                        PieceType::King => enumerate_king(board, color, sq, &mut moves),
                        PieceType::Queen => enumerate_queen(board, color, sq, &mut moves),
                        PieceType::Rook => enumerate_rook(board, color, sq, &mut moves),
                        PieceType::Bishop => enumerate_bishop(board, color, sq, &mut moves),
                        PieceType::Knight => enumerate_knight(board, color, sq, &mut moves),
                        PieceType::Pawn => enumerate_pawn(board, color, sq, &mut moves),
                    }
                }
            }
        }
    }

    moves
}

/* pub fn enumerate_legal_moves(board: &mut Board, color: Color) -> Vec<Move> {
    enumerate_moves(board, color).into_iter().filter(|m| m.is_legal(board)).collect()
} */

fn enumerate_pawn(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let off_rank = match color {
        Color::White => 1,
        Color::Black => -1,
    };

    let simple = (0, off_rank);

    if let Some(simple_sq) = from.offset(simple) {
        if board.get(simple_sq).is_none() {
            moves.push(Move::new(board, from, simple_sq));

            if from.rank() == RANK_2 {
                let double_sq = Square::new_nocheck(from.file(), RANK_4);
                if board.get(double_sq).is_none() {
                    moves.push(Move::new(board, from, double_sq));
                }
            }
        }
    }

    let captures_off = [(-1, off_rank), (1, off_rank)];
    for off in captures_off.iter() {
        if let Some(sq) = from.offset(*off) {
            match board.get(sq) {
                Some(p) if p.color != color => moves.push(Move::new(board, from,sq)),
                _ => {}
            }
        }
    }
}

fn enumerate_king(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(sq) = from.offset((i, j)) {
                if !board.contains_ally(sq, color) {
                    moves.push(Move::new(board, from, sq));
                }
            }
        }
    }
}

fn enumerate_straight_line(
    board: &Board,
    color: Color,
    from: Square,
    moves: &mut Vec<Move>,
    dir: (i8, i8),
    piece: Piece,
) {
    let mut i = 1i8;
    while let Some(sq) = from.offset((i * dir.0, i * dir.1)) {
        match board.get(sq) {
            Some(p) if p.color == color => break,
            Some(p) if p.color != color => {
                moves.push(Move::new(board, from, sq));
                break;
            }
            None => moves.push(Move::new(board, from, sq)),
            Some(_) => unreachable!(),
        }
        i += 1;
    }
}

fn enumerate_queen(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            enumerate_straight_line(
                board,
                color,
                from,
                moves,
                (i, j),
                Piece::new(PieceType::Queen, color),
            );
        }
    }
}

fn enumerate_rook(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let piece = Piece::new(PieceType::Rook, color);
    enumerate_straight_line(board, color, from, moves, (-1, 0), piece);
    enumerate_straight_line(board, color, from, moves, (1, 0), piece);
    enumerate_straight_line(board, color, from, moves, (0, -1), piece);
    enumerate_straight_line(board, color, from, moves, (0, 1), piece);
}

fn enumerate_bishop(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let piece = Piece::new(PieceType::Bishop, color);
    enumerate_straight_line(board, color, from, moves, (-1, -1), piece);
    enumerate_straight_line(board, color, from, moves, (-1, 1), piece);
    enumerate_straight_line(board, color, from, moves, (1, -1), piece);
    enumerate_straight_line(board, color, from, moves, (1, 1), piece);
}

fn enumerate_knight(board: &Board, color: Color, from: Square, moves: &mut Vec<Move>) {
    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (-1, -2),
        (1, 2),
        (1, -2),
        (2, -1),
        (2, 1),
    ];

    for off in offsets.iter() {
        if let Some(sq) = from.offset(*off) {
            if !board.contains_ally(sq, color) {
                moves.push(Move::new(board, from, sq));
            }
        }
    }
}
