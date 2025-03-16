use crate::bitboard::BitBoard;
use crate::pieces::Side;
use crate::position::Position;
use crate::board_navigator::get_piece_movements;
use crate::fen::from_fen;

struct PieceValue;
impl PieceValue {
    pub const PAWN: f32 = 1.0;
    pub const BISHOP: f32 = 3.0;
    pub const KNIGHT: f32 = 3.0;
    pub const ROOK: f32 = 5.0;
    pub const QUEEN: f32 = 9.0;
    // KING has infinite value
}

/**
 * Adjust the piece value based on where it is located on the board
 * 
 * 8
 * 7
 * 6
 * 5
 * 4
 * 3
 * 2
 * 1
 * a     b     c     d     e     f     g     h
 * 
 */
const WHITE_PAWN_ADJ: [f32; 64] = [
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
    1.50, 1.50, 1.50, 1.50, 1.50, 1.50, 1.50, 1.50,
    1.10, 1.10, 1.20, 1.30, 1.30, 1.20, 1.10, 1.10,
    1.05, 1.05, 1.10, 1.25, 1.25, 1.10, 1.05, 1.05,
    1.00, 1.00, 1.00, 1.20, 1.20, 1.00, 1.00, 1.00,
    1.05, 0.95, 0.90, 1.00, 1.00, 0.90, 0.95, 1.05,
    1.05, 1.10, 1.10, 0.80, 0.80, 1.10, 1.10, 1.05,
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
];

const BLACK_PAWN_ADJ: [f32; 64] = [
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
    1.05, 1.10, 1.10, 0.80, 0.80, 1.10, 1.10, 1.05,
    1.05, 0.95, 0.90, 1.00, 1.00, 0.90, 0.95, 1.05,
    1.00, 1.00, 1.00, 1.20, 1.20, 1.00, 1.00, 1.00,
    1.05, 1.05, 1.10, 1.25, 1.25, 1.10, 1.05, 1.05,
    1.10, 1.10, 1.20, 1.30, 1.30, 1.20, 1.10, 1.10,
    1.50, 1.50, 1.50, 1.50, 1.50, 1.50, 1.50, 1.50,
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
];

const WHITE_BISHOP_ADJ: [f32; 64] = [
    0.80, 0.90, 0.90, 0.90, 0.90, 0.90, 0.90, 0.80,
    0.90, 1.05, 1.00, 1.00, 1.00, 1.00, 1.05, 0.90,
    0.90, 1.10, 1.10, 1.10, 1.10, 1.10, 1.10, 0.90,
    0.90, 1.00, 1.10, 1.10, 1.10, 1.10, 1.00, 0.90,
    0.90, 1.05, 1.05, 1.10, 1.10, 1.05, 1.05, 0.90,
    0.90, 1.00, 1.05, 1.10, 1.10, 1.05, 1.00, 0.90,
    0.90, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.80, 0.90, 0.90, 0.90, 0.90, 0.90, 0.90, 0.80,
];

const BLACK_BISHOP_ADJ: [f32; 64] = [
    0.80, 0.90, 0.90, 0.90, 0.90, 0.90, 0.90, 0.80,
    0.90, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.90, 1.00, 1.05, 1.10, 1.10, 1.05, 1.00, 0.90,
    0.90, 1.05, 1.05, 1.10, 1.10, 1.05, 1.05, 0.90,
    0.90, 1.00, 1.10, 1.10, 1.10, 1.10, 1.00, 0.90,
    0.90, 1.10, 1.10, 1.10, 1.10, 1.10, 1.10, 0.90,
    0.90, 1.05, 1.00, 1.00, 1.00, 1.00, 1.05, 0.90,
    0.80, 0.90, 0.90, 0.90, 0.90, 0.90, 0.90, 0.80,
];

const WHITE_KNIGHT_ADJ: [f32; 64] = [
    0.50, 0.60, 0.70, 0.70, 0.70, 0.70, 0.60, 0.50,
    0.60, 0.80, 1.00, 1.05, 1.05, 1.00, 0.80, 0.60,
    0.70, 1.05, 1.10, 1.15, 1.15, 1.10, 1.05, 0.70,
    0.70, 1.00, 1.15, 1.20, 1.20, 1.15, 1.00, 0.70,
    0.70, 1.05, 1.15, 1.20, 1.20, 1.15, 1.05, 0.70,
    0.70, 1.00, 1.10, 1.15, 1.15, 1.10, 1.00, 0.70,
    0.60, 0.80, 1.00, 1.00, 1.00, 1.00, 0.80, 0.60,
    0.50, 0.60, 0.70, 0.70, 0.70, 0.70, 0.60, 0.50,
 ];

 const BLACK_KNIGHT_ADJ: [f32; 64] = [
    0.50, 0.60, 0.70, 0.70, 0.70, 0.70, 0.60, 0.50,
    0.60, 0.80, 1.00, 1.00, 1.00, 1.00, 0.80, 0.60,
    0.70, 1.00, 1.10, 1.15, 1.15, 1.10, 1.00, 0.70,
    0.70, 1.05, 1.15, 1.20, 1.20, 1.15, 1.05, 0.70,
    0.70, 1.00, 1.15, 1.20, 1.20, 1.15, 1.00, 0.70,
    0.70, 1.05, 1.10, 1.15, 1.15, 1.10, 1.05, 0.70,
    0.60, 0.80, 1.00, 1.05, 1.05, 1.00, 0.80, 0.60,
    0.50, 0.60, 0.70, 0.70, 0.70, 0.70, 0.60, 0.50,
 ];

 const WHITE_ROOK_ADJ: [f32; 64] = [
    1.00, 1.00, 1.05, 1.10, 1.10, 1.05, 1.00, 1.00,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    1.05, 1.10, 1.10, 1.10, 1.10, 1.10, 1.10, 1.05,
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
 ];

 const BLACK_ROOK_ADJ: [f32; 64] = [
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
    1.05, 1.10, 1.10, 1.10, 1.10, 1.10, 1.10, 1.05,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    0.95, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.95,
    1.00, 1.00, 1.05, 1.10, 1.10, 1.05, 1.00, 1.00,    
 ];

 const WHITE_QUEEN_ADJ: [f32; 64] = [
    0.80, 0.90, 0.90, 0.95, 0.95, 0.90, 0.90, 0.80,
    0.90, 1.00, 1.05, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.90, 1.05, 1.05, 1.05, 1.05, 1.05, 1.00, 0.90,
    1.00, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.95,
    0.95, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.95,
    0.90, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.90,
    0.90, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.80, 0.90, 0.90, 0.95, 0.95, 0.90, 0.90, 0.80,
 ];

 const BLACK_QUEEN_ADJ: [f32; 64] = [
    0.80, 0.90, 0.90, 0.95, 0.95, 0.90, 0.90, 0.80,
    0.90, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.90, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.90,
    0.95, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.95,
    1.00, 1.00, 1.05, 1.05, 1.05, 1.05, 1.00, 0.95,
    0.90, 1.05, 1.05, 1.05, 1.05, 1.05, 1.00, 0.90,
    0.90, 1.00, 1.05, 1.00, 1.00, 1.00, 1.00, 0.90,
    0.80, 0.90, 0.90, 0.95, 0.95, 0.90, 0.90, 0.80,
 ];

struct WhitePieceAdjustments;
impl WhitePieceAdjustments {
    const PAWN: [f32; 64] = WHITE_PAWN_ADJ;
    const BISHOP: [f32; 64] = WHITE_BISHOP_ADJ;
    const KNIGHT: [f32; 64] = WHITE_KNIGHT_ADJ;
    const ROOK: [f32; 64] = WHITE_ROOK_ADJ;
    const QUEEN: [f32; 64] = WHITE_QUEEN_ADJ;
}

struct BlackPieceAdjustments;
impl BlackPieceAdjustments {
    const PAWN: [f32; 64] = BLACK_PAWN_ADJ;
    const BISHOP: [f32; 64] = BLACK_BISHOP_ADJ;
    const KNIGHT: [f32; 64] = BLACK_KNIGHT_ADJ;
    const ROOK: [f32; 64] = BLACK_ROOK_ADJ;
    const QUEEN: [f32; 64] = BLACK_QUEEN_ADJ;
}

pub fn evaluate(fen: &str, depth: u8) -> f32 {
    let position = from_fen(fen);

    let eval = evaluate_position(&position);

    // todo get which player's turn it is so you know what colour pieces to get_piece_movement for
    // Side::WHITE or Side::BLACK

    // 1. get all possible moves of a position
    //  we already know the piece type as there is a BitBoard for each type
    //  loop through all piece types and get respective piece movement
    //      you need board state like _en_passant_target for possible en passant moves and _castling if it is available

    let movements = get_piece_movements(&position);

    for m in movements.iter() {
        println!("{}", m);
    }

    position.print();

    eval
}

fn evaluate_position(position: &Position) -> f32 {
    white_material(position) - black_material(position)
}

fn white_material(position: &Position) -> f32 {
    calc_material(position, Side::White)
}

fn black_material(position: &Position) -> f32 {
    calc_material(position, Side::Black)
}

fn calc_material(position: &Position, side: Side) -> f32 {

    let coloured_pawns = match side {
        Side::White => position.get_white_pawns(),
        Side::Black => position.get_black_pawns(),
    };
    let coloured_bishops = match side {
        Side::White => position.get_white_bishops(),
        Side::Black => position.get_black_bishops(),
    };
    let coloured_knights = match side {
        Side::White => position.get_white_knights(),
        Side::Black => position.get_black_knights(),
    };
    let coloured_rooks = match side {
        Side::White => position.get_white_rooks(),
        Side::Black => position.get_black_rooks(),
    };
    let coloured_queens = match side {
        Side::White => position.get_white_queens(),
        Side::Black => position.get_black_queens(),
    };

    let coloured_pawn_material = get_adjusted_material(
        coloured_pawns,
        PieceValue::PAWN,
        match side {
            Side::White => WhitePieceAdjustments::PAWN,
            Side::Black => BlackPieceAdjustments::PAWN,
        }
    );

    let coloured_bishop_count = get_adjusted_material(
        coloured_bishops,
        PieceValue::BISHOP,
        match side {
            Side::White => WhitePieceAdjustments::BISHOP,
            Side::Black => BlackPieceAdjustments::BISHOP,
        }
    );

    let coloured_knight_count = get_adjusted_material(
        coloured_knights,
        PieceValue::KNIGHT,
        match side {
            Side::White => WhitePieceAdjustments::KNIGHT,
            Side::Black => BlackPieceAdjustments::KNIGHT,
        }
    );

    let coloured_rook_count = get_adjusted_material(
        coloured_rooks,
        PieceValue::ROOK,
        match side {
            Side::White => WhitePieceAdjustments::ROOK,
            Side::Black => BlackPieceAdjustments::ROOK,
        }
    );

    let coloured_queen_count = get_adjusted_material(
        coloured_queens,
        PieceValue::QUEEN,
        match side {
            Side::White => WhitePieceAdjustments::QUEEN,
            Side::Black => BlackPieceAdjustments::QUEEN,
        }
    );

    let total = coloured_pawn_material +
        coloured_bishop_count +
        coloured_knight_count +
        coloured_rook_count +
        coloured_queen_count;

    println!("total: {}", total);
    
    total
}

fn get_adjusted_material(coloured_pieces: BitBoard, piece_value: f32, adjustments: [f32; 64]) -> f32 {
    let mut coloured_material_value = 0.0;

    for (i, adjustment) in adjustments.iter().enumerate() {
        if coloured_pieces.0 >> i & 1 == 1 {
            coloured_material_value += piece_value * adjustment;
            println!("piece worth {} on {} adjusted by {}:\t{}", piece_value, i, adjustment, coloured_material_value);
        }
    }
    coloured_material_value
}