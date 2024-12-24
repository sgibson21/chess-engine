use crate::fen::{to_fen, from_fen, fen_to_asci_board};
use crate::pieces::{Pieces, PieceAsci};
use crate::bitboard::{BitBoard, to_asci_board, asci_board_indicies};
use crate::evaluator::evaluate;

const HEADER: &str =       "64↓             48↓             32↓             16↓             1↓";
const RANKS_HEADER: &str = " 8↓      7↓      6↓      5↓      4↓      3↓      2↓      1↓       ";

fn print_board(board: BitBoard) {
    println!("BitBoard:\t{:#066b}", board.0);
}

pub fn play() {

    // let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";

    let sqa1: usize = SquareLabels::A1 as usize;
    let sqh8: usize = SquareLabels::H8 as usize;
    println!("Square  A1: {:#?}", sqa1);
    println!("Square  H8: {:#?}", sqh8);

    let mut board = BitBoard(0);
    println!("New Board:");
    print_board(board);
    board.set(0,0);
    board.set(7,7);

    println!("After setting (0,0) and (7,7):");
    print_board(board);

    println!("\nBuilding position from FEN:\n\t{}", starting_fen);
    let position = from_fen(starting_fen);

    println!("\t{}", HEADER);
    println!("\t{}", RANKS_HEADER);
    println!("Pawns:\t{:#066b}", position.pieces[Pieces::PAWN].0);
    println!("Bishops\t{:#066b}", position.pieces[Pieces::BISHOP].0);
    println!("Knights\t{:#066b}", position.pieces[Pieces::KNIGHT].0);
    println!("Rooks\t{:#066b}", position.pieces[Pieces::ROOK].0);
    println!("Queens\t{:#066b}", position.pieces[Pieces::QUEEN].0);
    println!("Kings\t{:#066b}", position.pieces[Pieces::KING].0);

    let bb_asci = to_asci_board(position.pieces[Pieces::PAWN], PieceAsci::PAWN.black);
    println!("\nPawn ASCI Board:\n{}", bb_asci);
    println!("\nNumbered ASCI Board:\n{}", asci_board_indicies());

    println!("Evaluation: {}", evaluate(starting_fen, 1));

    let built_fen = to_fen(position.clone());
    println!("\nFEN built from position:\n\t{}", built_fen);

    let asci_board = fen_to_asci_board(built_fen);
    println!("\nASCI Board:\n{}", asci_board);

    position.get_white_pawns().print();
    position.get_white_bishops().print();
    position.get_white_knights().print();
    position.get_white_rooks().print();
    position.get_white_queens().print();
    position.get_white_kings().print();

    position.get_black_pawns().print();
    position.get_black_bishops().print();
    position.get_black_knights().print();
    position.get_black_rooks().print();
    position.get_black_queens().print();
    position.get_black_kings().print();


}

/// Labels for every [`Square`] on the board.
#[repr(usize)]
#[rustfmt::skip]
#[derive(Debug)]
pub enum SquareLabels {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
