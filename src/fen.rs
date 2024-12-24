use crate::pieces::{Pieces, Side};
use crate::position::{StatelessPosition, Position, Castling};
use crate::bitboard::BitBoard;
use crate::utils::get_uni_char;
use crate::board_navigator::Coord;

use std::collections::HashMap;

pub struct FenPieceSymbol {
    pub white: char,
    pub black: char,
}

pub struct FenSymbols;
impl FenSymbols {
    pub const PAWN: FenPieceSymbol = FenPieceSymbol {
        white: 'P',
        black: 'p',
    };
    pub const BISHOP: FenPieceSymbol = FenPieceSymbol {
        white: 'B',
        black: 'b',
    };
    pub const KNIGHT: FenPieceSymbol = FenPieceSymbol {
        white: 'N',
        black: 'n',
    };
    pub const ROOK: FenPieceSymbol = FenPieceSymbol {
        white: 'R',
        black: 'r',
    };
    pub const QUEEN: FenPieceSymbol = FenPieceSymbol {
        white: 'Q',
        black: 'q',
    };
    pub const KING: FenPieceSymbol = FenPieceSymbol {
        white: 'K',
        black: 'k',
    };
}

pub fn to_fen(position: Position) -> String {

    let pieces_to_fen_symbols: HashMap<usize, FenPieceSymbol> = HashMap::from([
        (Pieces::PAWN, FenSymbols::PAWN),
        (Pieces::BISHOP, FenSymbols::BISHOP),
        (Pieces::KNIGHT, FenSymbols::KNIGHT),
        (Pieces::ROOK, FenSymbols::ROOK),
        (Pieces::QUEEN, FenSymbols::QUEEN),
        (Pieces::KING, FenSymbols::KING),
    ]);
    

    let mut fen: String = "".to_owned();
    let mut blank_count: u32 = 0;

    for n in 0..64 {

        let is_white = position.is_white(n);
        let mut has_piece: bool = false;

        for (piece, fen_symbols) in &pieces_to_fen_symbols {
            if position.pieces[*piece].0 >> n & 1 == 1 {
                if blank_count > 0 {
                    fen.push(char::from_digit(blank_count, 10).unwrap());
                    blank_count = 0;
                }

                fen.push(
                    match is_white {
                        true => fen_symbols.white,
                        false => fen_symbols.black,
                    }
                );

                has_piece = true;
            }
        }

        if !has_piece {
            blank_count += 1;
        }

        if n < 64 && n % 8 == 7 {
            if blank_count > 0 {
                fen.push(char::from_digit(blank_count, 10).unwrap());
            }
            fen.push('/');
            blank_count = 0;
        }

    }

    fen
}

pub fn from_fen(fen: &str) -> Position {
    let [placement, active_color, castling, en_passant_target/*, _half_move_clock, _full_moves*/]: [&str; 4/*6*/] = fen.split(' ')
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();
    build_position(placement, active_color, castling, en_passant_target, 0, 1) // TODO get _half_move_clock and _full_moves working in client app
}

pub fn fen_to_asci_board(fen: String) -> String {

    // fen symbol to piece asci tuple (white, black)
    let piece_asci: HashMap<char, u32> = HashMap::from([
        (FenSymbols::PAWN.white, 0x2659),
        (FenSymbols::PAWN.black, 0x265F),
        (FenSymbols::BISHOP.white, 0x2657),
        (FenSymbols::BISHOP.black, 0x265D),
        (FenSymbols::KNIGHT.white, 0x2658),
        (FenSymbols::KNIGHT.black, 0x265E),
        (FenSymbols::ROOK.white, 0x2656),
        (FenSymbols::ROOK.black, 0x265C),
        (FenSymbols::QUEEN.white, 0x2655),
        (FenSymbols::QUEEN.black, 0x265B),
        (FenSymbols::KING.white, 0x2654),
        (FenSymbols::KING.black, 0x265A),
    ]);

    let mut output: String = "".to_owned();

    for char in fen.chars() {

        if char == '/' {
            output.push('\n');
        }

        if let Some(asci_code) = piece_asci.get(&char) {
            let asci = get_uni_char(*asci_code);
            output.push_str(&format!("[{asci}]"))
        }

        if let '1'..='8' = char {
            if let Some(i) = char.to_digit(10) {
                for _j in 0..i {
                    output.push_str("[ ]");
                }
            }
        }
    }

    output
}

/**
 * castling (eg: KQkq)
 * en_passant_target (eg: c3)
 */
fn build_position(fen: &str, active_colour: &str, castling: &str, en_passant_target: &str, half_move_clock: usize, full_moves: usize) -> Position {
    let stateless_position = build_stateless_position(fen);

    let castling = Castling {
        K: castling.contains('K'),
        Q: castling.contains('Q'),
        k: castling.contains('k'),
        q: castling.contains('q'),
    };

    Position::new(
        stateless_position,
        match active_colour {
            "w" => Side::White,
            "b" => Side::Black,
            _ => panic!("Unknown active colour"),
        },
        castling,
        get_en_passant_target_coord(en_passant_target),
        half_move_clock,
        full_moves,
    )
}

fn get_en_passant_target_coord(en_passant_target: &str) -> Option<Coord> {

    if en_passant_target.eq("-") {
        return None;
    }
    let mut en_passant_target_chars = en_passant_target.chars();
    let file = en_passant_target_chars.next().unwrap();
    let rank_char = en_passant_target_chars.next().unwrap();

    println!("en_passant_target: '{}'", en_passant_target);
    println!("File: {}, Rank: {}", file, rank_char);

    const RADIX: u32 = 10;
    let rank = rank_char.to_digit(RADIX).unwrap() as u8;

    Some(Coord(file, rank))
}

fn build_stateless_position(fen: &str) -> StatelessPosition {
    let mut white_pieces = BitBoard(0);
    let mut black_pieces = BitBoard(0);

    let mut pawns = BitBoard(0);
    let mut rooks = BitBoard(0);
    let mut knights = BitBoard(0);
    let mut bishops = BitBoard(0);
    let mut queens = BitBoard(0);
    let mut kings = BitBoard(0);

    let mut count = 0;

    fn set_piece(x: u8, y: u8, piece_board: &mut BitBoard, side_board: &mut BitBoard) {
        piece_board.set(x, y);
        side_board.set(x, y);
    }

    for c in fen.chars() {

        let x = count % 8;
        let y = count / 8;
        let mut increment: u8 = 1;

        match c {
            'p' => set_piece(x, y, &mut pawns, &mut black_pieces),
            'r' => set_piece(x, y, &mut rooks, &mut black_pieces),
            'n' => set_piece(x, y, &mut knights, &mut black_pieces),
            'b' => set_piece(x, y, &mut bishops, &mut black_pieces),
            'q' => set_piece(x, y, &mut queens, &mut black_pieces),
            'k' => set_piece(x, y, &mut kings, &mut black_pieces),
            'P' => set_piece(x, y, &mut pawns, &mut white_pieces),
            'R' => set_piece(x, y, &mut rooks, &mut white_pieces),
            'N' => set_piece(x, y, &mut knights, &mut white_pieces),
            'B' => set_piece(x, y, &mut bishops, &mut white_pieces),
            'Q' => set_piece(x, y, &mut queens, &mut white_pieces),
            'K' => set_piece(x, y, &mut kings, &mut white_pieces),
            '1'..='8' => if let Some(i) = c.to_digit(10) { increment = i as u8 },
            '/' => increment = 0_u8,
            _ => (),
        };

        count += increment;

    }

    StatelessPosition::new(white_pieces, black_pieces, pawns, bishops, knights, rooks, queens, kings)

}
