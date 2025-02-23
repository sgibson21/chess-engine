use crate::{pieces::Pieces, position::Position, utils::coord_from_index};
use super::Coord;

#[derive(Clone)]
struct AttackVector<'a> {
    directions: &'a[&'a(i32, i32)],
    piece_types: &'a[usize], // vec of Pieces
    ranged: bool,
}

/**
 * directions to check in a straight line for an attacking bishop, queen - nw, ne, se, sw
 */
const DIAGONAL_DIRECTIONS: &[&(i32, i32)] = &[
    &( -1,  1 ),
    &(  1,  1 ),
    &(  1, -1 ),
    &( -1, -1 ),
];

/**
 * directions to check in a straight line for an attacking rook, queen - n, e, s, w
 */
const STRAIGHT_DIRECTIONS: &[&(i32, i32)] = &[
    &(  0,  1 ),
    &(  1,  0 ),
    &(  0, -1 ),
    &( -1,  0 ),
];

const ALL_DIRECTIONS: &[&(i32, i32)] = &[
    &( -1,  1 ),
    &(  1,  1 ),
    &(  1, -1 ),
    &( -1, -1 ),
    &(  0,  1 ),
    &(  1,  0 ),
    &(  0, -1 ),
    &( -1,  0 ),
];

/**
 * Ranged piece types and the directions in which they can move
 */
const ATTACK_VECTORS: [AttackVector; 3] = [
    AttackVector {
        directions: DIAGONAL_DIRECTIONS,
        piece_types: &[Pieces::BISHOP, Pieces::QUEEN],
        ranged: true
    },
    AttackVector {
        directions: STRAIGHT_DIRECTIONS,
        piece_types: &[Pieces::ROOK, Pieces::QUEEN],
        ranged: true
    },
    AttackVector {
        directions: ALL_DIRECTIONS,
        piece_types: &[Pieces::KING],
        ranged: false
    }
];

pub fn is_attacked(position: &Position, coord: &Coord, direction: i32) -> bool {

    let pawn_attack_vector = AttackVector {
        directions: &[&(-1, direction), &(1, direction)],
        piece_types: &[Pieces::PAWN],
        ranged: false,
    };

    let mut attack_vectors: Vec<AttackVector> = ATTACK_VECTORS.to_vec();
    attack_vectors.push(pawn_attack_vector);

    let mut next_coord = coord.clone();
    // scout lines
    for attack_vector in attack_vectors {
        for dir in attack_vector.directions {

            if attack_vector.ranged {
                while let Some(c) = next_coord.to(dir.0, dir.1) {
                    next_coord = c;
                    let next_coord_index = next_coord.to_index();
    
                    if position.has_piece(next_coord_index) {
                        if has_attacking_piece(position, next_coord_index, direction, attack_vector.piece_types) {
                            return true;
                        }
    
                        // reset coord
                        next_coord = coord.clone();
                        break;
                    }
                }
            } else {
                if let Some(c) = next_coord.to(dir.0, dir.1) {
                    let next_coord_index = c.to_index();

                    if has_attacking_piece(position, next_coord_index, direction, attack_vector.piece_types) {
                        println!("piece on {} is attacking {}", coord_from_index(next_coord_index), coord);
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

fn has_attacking_piece(position: &Position, index: i32, direction: i32, piece_types: &[usize]) -> bool {
    if position.has_piece(index) {
        if is_opposing_piece(position, index, direction) && is_piece_of_type(position, index, piece_types) {
            return true;
        }
    }

    false
}

fn is_opposing_piece(position: &Position, index: i32, direction: i32) -> bool {
    if direction > 0 { position.is_black(index) } else { position.is_white(index) }
}

// checks if position has a piece of one of the given piece types at the given index
fn is_piece_of_type(position: &Position, index: i32, types: &[usize]) -> bool {
    for piece_type in types {
        let is_of_type = match piece_type {
            &Pieces::PAWN   => position.is_pawn(index),
            &Pieces::BISHOP => position.is_bishop(index),
            &Pieces::KNIGHT => position.is_knight(index),
            &Pieces::ROOK => position.is_rook(index),
            &Pieces::QUEEN => position.is_queen(index),
            &Pieces::KING => position.is_king(index),
            _ => false,
        };

        if is_of_type {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::Coord;
    use super::is_attacked;
    use crate::fen::{from_fen, fen_to_asci_board};

    #[test]
    fn is_attacked_by_pawn() {
        let fen = "rn2k3/p1p2pp1/1p1p4/5r2/1bP1n3/1PN2N2/P2B2pP/R2QK2R w KQkq -";
        let position = from_fen(fen);
        let coord = Coord('f',1);
    
        let expected = true;
        let actual = is_attacked(&position, &coord, 1);
    
        assert_eq!(expected, actual, "expected {} but got {}", expected, actual);
    }
    
    #[test]
    fn is_not_attacked_by_pawn_out_of_range() {
        let fen = "rn2k3/p1p2pp1/1p1p4/8/1bP1nr2/1PN2NPp/P6P/R1BQK2R w KQkq -";
        let position = from_fen(fen);
        let coord = Coord('f',1);
    
        let expected = false;
        let actual = is_attacked(&position, &coord, 1);
    
        assert_attacked(expected, actual, &coord, fen);
    }
    
    #[test]
    fn is_attacked_by_ranged_diagonal_ne() {
        let fen = "rn1qkbnr/p1pp1ppp/1p6/4p3/2b1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -";
        let position = from_fen(fen);
        let coord = Coord('f',1);
    
        let expected = true;
        let actual = is_attacked(&position, &coord, 1);
    
        assert_attacked(expected, actual, &coord, fen);
    }
    
    #[test]
    fn is_attacked_by_ranged_straight_n() {
        let fen = "rn2k3/p1pp1pp1/1p6/4Nr1p/1bP1n3/2P5/PP4PP/RNBQK2R w KQkq -";
        let position = from_fen(fen);
        let coord = Coord('f',1);
    
        let expected = true;
        let actual = is_attacked(&position, &coord, 1);
    
        assert_attacked(expected, actual, &coord, fen);
    }
    
    #[test]
    fn is_not_attacked_by_ranged_straight_n_due_to_block() {
        let fen = "rn2k3/p1p2pp1/1p1p4/5r1p/1bP1n3/2P2N2/PP4PP/RNBQK2R w KQkq -";
        let position = from_fen(fen);
        let coord = Coord('f',1);
    
        let expected = false;
        let actual = is_attacked(&position, &coord, 1);
    
        assert_attacked(expected, actual, &coord, fen);
    }
    
    fn assert_attacked(expected: bool, actual: bool, coord: &Coord, fen: &str) {
        assert_eq!(
            expected,
            actual,
            "Testing {} is attacked on:\n\n{}\n\nexpected {} but got {}",
            coord,
            fen_to_asci_board(String::from(fen.split_once(" ").unwrap().0)),
            expected,
            actual
        );
    }
}
