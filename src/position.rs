#![allow(non_snake_case)]

use crate::bitboard::BitBoard;
use crate::fen::{to_fen, fen_to_asci_board};
use crate::utils::{coord_from_index};
use crate::board_navigator::{Coord, Piece, CastlingSide};
use crate::pieces::Side;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Position {
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

    fn get_white_pieces_mut(&mut self) -> &mut BitBoard {
        &mut self.sides[SIDE_WHITE_INDEX]
    }

    fn get_black_pieces(&self) -> BitBoard {
        self.sides[SIDE_BLACK_INDEX]
    }

    fn get_black_pieces_mut(&mut self) -> &mut BitBoard {
        &mut self.sides[SIDE_BLACK_INDEX]
    }

    fn get_pawns(&self) -> BitBoard {
        self.pieces[PIECE_PAWN_INDEX]
    }

    fn get_pawns_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_PAWN_INDEX]
    }

    fn get_bishops(&self) -> BitBoard {
        self.pieces[PIECE_BISHOP_INDEX]
    }

    fn get_bishops_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_BISHOP_INDEX]
    }

    fn get_knights(&self) -> BitBoard {
        self.pieces[PIECE_KNIGHT_INDEX]
    }

    fn get_knights_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_KNIGHT_INDEX]
    }

    fn get_rooks(&self) -> BitBoard {
        self.pieces[PIECE_ROOK_INDEX]
    }

    fn get_rooks_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_ROOK_INDEX]
    }

    fn get_queens(&self) -> BitBoard {
        self.pieces[PIECE_QUEEN_INDEX]
    }

    fn get_queens_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_QUEEN_INDEX]
    }

    fn get_kings(&self) -> BitBoard {
        self.pieces[PIECE_KING_INDEX]
    }

    fn get_kings_mut(&mut self) -> &mut BitBoard {
        &mut self.pieces[PIECE_KING_INDEX]
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

    pub fn make_move(&mut self, from: &Coord, to: &Coord, castling: &Option<CastlingSide>) -> Result<i32, String> {
        let from_index = from.to_index();
        let to_index = to.to_index();
        let o_side = self.get_side(from_index);
        if let Some(side) = o_side {
            let o_piece = self.get_piece(from_index);
            if let Some(piece) = o_piece  {

                // TODO: if promoting, place the selected piece

                // if moving pawn up 2, set en_passant_target
                if piece == Piece::P {
                    let diff = from.rank_diff(to);
                    if diff == 2 {
                        let en_passant_target_rank = if from.1 < to.1 { from.1 + 1 } else { from.1 - 1 };
                        self.en_passant_target = Some(Coord(from.0, en_passant_target_rank));
                    }
                }

                // place the piece first because we need to know the side and type
                self.place_piece(to_index, side, piece);

                // if castling, move rook to other side
                self.castle(castling);

                // remove the piece moved after it was placed
                self.remove_piece(from_index);

                return Ok(to_index);
            }
        }

        Err(format!("No Piece found at {}", from))
    }

    fn get_side(&self, index: i32) -> Option<Side> {
        if self.has_piece(index) {
            if self.is_white(index) { return Some(Side::White); } else { return Some(Side::Black); }
        }
        None
    }

    fn get_piece(&self, index: i32) -> Option<Piece> {
        if self.is_pawn(index) {
            return Some(Piece::P);
        } else if self.is_bishop(index) {
            return Some(Piece::B);
        } else if self.is_knight(index) {
            return Some(Piece::N);
        } else if self.is_rook(index) {
            return Some(Piece::R);
        } else if self.is_queen(index) {
            return Some(Piece::Q);
        } else if self.is_king(index) {
            return Some(Piece::K);
        }
        None
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

    fn remove_piece(&mut self, index: i32) {
        if self.has_piece(index) {
            let side_bb = self.get_side_bitboard(index);
            side_bb.unset_index(index as u8);
            let piece_bb_opt = self.get_piece_bitboard(index);
            if let Some(piece_bb) = piece_bb_opt {
                piece_bb.unset_index(index as u8);
            }
        }
    }

    fn place_piece(&mut self, index: i32, side: Side, piece: Piece) {
        if self.has_piece(index) {
            // if there is a piece, remove it
            self.remove_piece(index);
        }

        let side_bitboard = match side {
            Side::White => self.get_white_pieces_mut(),
            Side::Black => self.get_black_pieces_mut(),
        };

        side_bitboard.set_index(index as u8);

        let piece_bitboard = match piece {
            Piece::P => self.get_pawns_mut(),
            Piece::N => self.get_knights_mut(),
            Piece::B => self.get_bishops_mut(),
            Piece::R => self.get_rooks_mut(),
            Piece::Q => self.get_queens_mut(),
            Piece::K => self.get_kings_mut(),
        };

        piece_bitboard.set_index(index as u8);
    }

    fn castle(&mut self, castling: &Option<CastlingSide>) {
        if let Some(castling_side) = castling {
            match castling_side {
                CastlingSide::WK => {
                    let result = self.make_move(&Coord('h', 1), &Coord('f', 1), &None);
                    if result.is_ok() {
                        // castling revokes rights to castle again
                        self.castling.K = false;
                        self.castling.Q = false;
                    }
                },
                CastlingSide::WQ => {
                    let result = self.make_move(&Coord('a', 1), &Coord('d', 1), &None);
                    if result.is_ok() {
                        // castling revokes rights to castle again
                        self.castling.Q = false;
                        self.castling.K = false;
                    }
                }
                CastlingSide::BK => {
                    let result = self.make_move(&Coord('h', 8), &Coord('f', 8), &None);
                    if result.is_ok() {
                        // castling revokes rights to castle again
                        self.castling.k = false;
                        self.castling.q = false;
                    }
                }
                CastlingSide::BQ => {
                    let result = self.make_move(&Coord('a', 8), &Coord('d', 8), &None);
                    if result.is_ok() {
                        // castling revokes rights to castle again
                        self.castling.q = false;
                        self.castling.k = false;
                    }
                }
            };
        }
    }

    fn get_side_bitboard(&mut self, index: i32) -> &mut BitBoard {
        if self.is_white(index) { self.get_white_pieces_mut() } else { self.get_black_pieces_mut() }
    }

    fn get_piece_bitboard(&mut self, index: i32) -> Option<&mut BitBoard> {
        if self.is_pawn(index) {
            return Some(self.get_pawns_mut());
        } else if self.is_bishop(index) {
            return Some(self.get_bishops_mut());
        } else if self.is_knight(index) {
            return Some(self.get_knights_mut());
        } else if self.is_rook(index) {
            return Some(self.get_rooks_mut());
        } else if self.is_queen(index) {
            return Some(self.get_queens_mut());
        } else if self.is_king(index) {
            return Some(self.get_kings_mut());
        }
        None
    }
}

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

#[cfg(test)]
mod tests {
    use super::{Side, Piece, Coord};
    use crate::{board_navigator::CastlingSide, fen::{fen_to_asci_board, from_fen}};

    #[test]
    fn remove_piece() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let coord = Coord('e',2);
        let removed_index = coord.to_index();

        position.remove_piece(removed_index);
        position.print();
        assert_eq!(position.has_piece(removed_index), false);
    }

    #[test]
    fn place_piece() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let coord = Coord('e',4);
        let placed_index = coord.to_index();
        let side = Side::White;
        let piece = Piece::Q;

        position.place_piece(placed_index, side, piece);
        position.print();
        assert_eq!(position.has_piece(placed_index), true);
    }

    #[test]
    fn make_move_success() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let from_coord = Coord('e',2);
        let to_coord = Coord('e',4);

        position.print();
    
        let result = position.make_move(&from_coord, &to_coord, &None);
    
        position.print();
        
        assert!(result.is_ok());
    }

    #[test]
    fn make_move_fail() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let from_coord = Coord('e',3);
        let to_coord = Coord('e',4);
    
        let result = position.make_move(&from_coord, &to_coord, &None);

        assert!(result.is_err());
        assert_eq!(result, Err(String::from("No Piece found at e3")));
    }

    #[test]
    fn make_move_should_set_en_passant_target_as_white() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let from_coord = Coord('e',2);
        let to_coord = Coord('e',4);

        let result = position.make_move(&from_coord, &to_coord, &None);
        
        assert!(result.is_ok());
        assert!(position.en_passant_target.is_some());
        let en_passant_target = position.en_passant_target.unwrap();
        assert_eq!(en_passant_target.0, 'e');
        assert_eq!(en_passant_target.1, 3);
    }

    #[test]
    fn make_move_should_set_en_passant_target_as_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let mut position = from_fen(fen);
        let from_coord = Coord('e',7);
        let to_coord = Coord('e',5);

        let result = position.make_move(&from_coord, &to_coord, &None);
        
        assert!(result.is_ok());
        assert!(position.en_passant_target.is_some());
        let en_passant_target = position.en_passant_target.unwrap();
        assert_eq!(en_passant_target.0, 'e');
        assert_eq!(en_passant_target.1, 6);
    }

    #[test]
    fn make_move_castling_wk() {
        let fen = "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -";
        let mut position = from_fen(fen);
        let from = Coord('e',1);
        let to = Coord('g',1);

        let result = position.make_move(&from, &to, &Some(CastlingSide::WK));

        position.print();
        
        assert!(result.is_ok());
        assert!(position.is_white(Coord('f',1).to_index()));
        assert!(position.is_rook(Coord('f',1).to_index()));
        assert_eq!(position.castling.K, false);
        assert_eq!(position.castling.Q, false);
    }

    #[test]
    fn make_move_castling_bk() {
        let fen = "r3k2r/pppq1ppp/2np1n2/2b1p1B1/2B1P1b1/2NP1N2/PPPQ1PPP/2KR3R b kq -";
        let mut position = from_fen(fen);
        let from = Coord('e',8);
        let to = Coord('g',8);

        let result = position.make_move(&from, &to, &Some(CastlingSide::BK));

        position.print();
        
        assert!(result.is_ok());
        assert!(position.is_black(Coord('f',8).to_index()));
        assert!(position.is_rook(Coord('f',8).to_index()));
        assert_eq!(position.castling.k, false);
        assert_eq!(position.castling.q, false);
    }

    #[test]
    fn make_move_castling_wq() {
        let fen = "r3k2r/pppq1ppp/2np1n2/2b1p1B1/2B1P1b1/2NP1N2/PPPQ1PPP/R3K2R w KQkq -";
        let mut position = from_fen(fen);
        let from = Coord('e',1);
        let to = Coord('c',1);

        let result = position.make_move(&from, &to, &Some(CastlingSide::WQ));

        position.print();
        
        assert!(result.is_ok());
        assert!(position.is_white(Coord('d',1).to_index()));
        assert!(position.is_rook(Coord('d',1).to_index()));
        assert_eq!(position.castling.Q, false);
        assert_eq!(position.castling.K, false);
    }

    #[test]
    fn make_move_castling_bq() {
        let fen = "r3k2r/pppq1ppp/2np1n2/2b1p1B1/2B1P1b1/2NP1N2/PPPQ1PPP/2KR3R b kq -";
        let mut position = from_fen(fen);
        let from = Coord('e',8);
        let to = Coord('c',8);

        position.print();

        let result = position.make_move(&from, &to, &Some(CastlingSide::BQ));

        position.print();
        
        assert!(result.is_ok());
        assert!(position.is_black(Coord('d',8).to_index()));
        assert!(position.is_rook(Coord('d',8).to_index()));
        assert_eq!(position.castling.q, false);
        assert_eq!(position.castling.k, false);
    }
}

