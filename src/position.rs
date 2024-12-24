#![allow(non_snake_case)]

use crate::bitboard::BitBoard;
use crate::fen::{to_fen, fen_to_asci_board};
use crate::utils::{coord_from_index};
use crate::board_navigator::Coord;
use crate::pieces::Side;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Position  {
    /// Board for each side
    pub sides: [BitBoard; 2],
    // BitBoards for all pieces
    pub pieces: [BitBoard; 6],

    pub active_colour: Side,

    pub castling: Castling,

    pub en_passant_target: Option<Coord>,

    pub half_move_clock: usize,

    pub full_moves: usize,
}

const SIDE_WHITE_INDEX: usize = 0;
const SIDE_BLACK_INDEX: usize = 1;

const PIECE_PAWN_INDEX: usize = 0;
const PIECE_BISHOP_INDEX: usize = 1;
const PIECE_KNIGHT_INDEX: usize = 2;
const PIECE_ROOK_INDEX: usize = 3;
const PIECE_QUEEN_INDEX: usize = 4;
const PIECE_KING_INDEX: usize = 5;

impl Position {
    pub fn new(
        stateless_position: StatelessPosition,
        active_colour: Side,
        castling: Castling,
        en_passant_target: Option<Coord>,
        half_move_clock: usize,
        full_moves: usize,
    ) -> Position {
        Position {
            sides: stateless_position.sides,
            pieces: stateless_position.pieces,
            active_colour,
            castling,
            en_passant_target,
            half_move_clock,
            full_moves,
        }
    }

    fn get_white_pieces(&self) -> BitBoard {
        self.sides[SIDE_WHITE_INDEX]
    }

    fn get_black_pieces(&self) -> BitBoard {
        self.sides[SIDE_BLACK_INDEX]
    }

    fn get_pawns(&self) -> BitBoard {
        self.pieces[PIECE_PAWN_INDEX]
    }

    fn get_bishops(&self) -> BitBoard {
        self.pieces[PIECE_BISHOP_INDEX]
    }

    fn get_knights(&self) -> BitBoard {
        self.pieces[PIECE_KNIGHT_INDEX]
    }

    fn get_rooks(&self) -> BitBoard {
        self.pieces[PIECE_ROOK_INDEX]
    }

    fn get_queens(&self) -> BitBoard {
        self.pieces[PIECE_QUEEN_INDEX]
    }

    fn get_kings(&self) -> BitBoard {
        self.pieces[PIECE_KING_INDEX]
    }

    pub fn has_piece(&self, index: i32) -> bool {
        (self.get_white_pieces() | self.get_black_pieces()).0 >> index & 1 == 1
    }

    /**
     * Returns true if a white piece is on the square given by the index (0..64)
     */
    pub fn is_white(&self, index: i32) -> bool {
        self.get_white_pieces().0 >> index & 1 == 1
    }

    pub fn is_black(&self, index: i32) -> bool {
        self.get_black_pieces().0 >> index & 1 == 1
    }

    pub fn is_pawn(&self, index: i32) -> bool {
        self.get_pawns().0 >> index & 1 == 1
    }

    pub fn is_bishop(&self, index: i32) -> bool {
        self.get_bishops().0 >> index & 1 == 1
    }

    pub fn is_knight(&self, index: i32) -> bool {
        self.get_knights().0 >> index & 1 == 1
    }

    pub fn is_rook(&self, index: i32) -> bool {
        self.get_rooks().0 >> index & 1 == 1
    }

    pub fn is_queen(&self, index: i32) -> bool {
        self.get_queens().0 >> index & 1 == 1
    }

    pub fn is_king(&self, index: i32) -> bool {
        self.get_kings().0 >> index & 1 == 1
    }

    pub fn is_en_passant_target(&self, index: i32) -> bool {
        match &self.en_passant_target {
            Some(en_passant_coord) => {
                let coord_to_check = coord_from_index(index);
                coord_to_check.0 == en_passant_coord.0 && coord_to_check.1 == en_passant_coord.1
            },
            None => false
        }
    }

    pub fn get_white_pawns(&self) -> BitBoard {
        self.get_white_pieces() & self.get_pawns()
    }

    pub fn get_white_bishops(&self) -> BitBoard {
        self.get_white_pieces() & self.get_bishops()
    }

    pub fn get_white_knights(&self) -> BitBoard {
        self.get_white_pieces() & self.get_knights()
    }

    pub fn get_white_rooks(&self) -> BitBoard {
        self.get_white_pieces() & self.get_rooks()
    }

    pub fn get_white_queens(&self) -> BitBoard {
        self.get_white_pieces() & self.get_queens()
    }

    pub fn get_white_kings(&self) -> BitBoard {
        self.get_white_pieces() & self.get_kings()
    }

    pub fn get_black_pawns(&self) -> BitBoard {
        self.get_black_pieces() & self.get_pawns()
    }

    pub fn get_black_bishops(&self) -> BitBoard {
        self.get_black_pieces() & self.get_bishops()
    }

    pub fn get_black_knights(&self) -> BitBoard {
        self.get_black_pieces() & self.get_knights()
    }

    pub fn get_black_rooks(&self) -> BitBoard {
        self.get_black_pieces() & self.get_rooks()
    }

    pub fn get_black_queens(&self) -> BitBoard {
        self.get_black_pieces() & self.get_queens()
    }

    pub fn get_black_kings(&self) -> BitBoard {
        self.get_black_pieces() & self.get_kings()
    }

    pub fn print(&self) {
        let built_fen = to_fen(self.clone());
        println!("\nFEN built from position:\n\t{}", built_fen);

        let asci_board = fen_to_asci_board(built_fen);
        println!("\nASCI Board:\n{}", asci_board);
        println!("Active Colour: {:?}", self.active_colour);
        println!("Castling: {:?}", self.castling);
        println!("En Passant: {:?}", self.en_passant_target);
    }
}

// pub fn get_piece(position: &Position, coord: Coord) -> Option<Piece> {
//     None
// }


pub struct StatelessPosition {
    /// Board for each side
    sides: [BitBoard; 2],
    // BitBoards for all pieces
    pieces: [BitBoard; 6],
}

impl StatelessPosition {
    pub fn new(
        white_pieces: BitBoard,
        black_pieces: BitBoard,
        pawns: BitBoard,
        bishops: BitBoard,
        knights: BitBoard,
        rooks: BitBoard,
        queens: BitBoard,
        kings: BitBoard
    ) -> StatelessPosition {
        StatelessPosition {
            sides: [white_pieces, black_pieces],
            pieces: [
                pawns,
                bishops,
                knights,
                rooks,
                queens,
                kings,
            ]
        }
    }
}

#[derive(Clone)]
pub struct Castling {
    pub K: bool,
    pub Q: bool,
    pub k: bool,
    pub q: bool,
}

impl fmt::Debug for Castling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // KQkq
        let mut castling: String = "".to_owned();
        if self.K {
            castling.push('K');
        }
        if self.Q {
            castling.push('Q');
        }
        if self.k {
            castling.push('k');
        }
        if self.q {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }
        write!(f, "{}", castling)
    }
}
