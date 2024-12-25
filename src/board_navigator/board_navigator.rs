use crate::position::{Position, Castling};
use crate::pieces::Side;
use crate::utils::{coord_from_index, file_of_index, is_rank};
use super::Coord;

 /**
  * A second, separate way to do proper move notation { piece: Piece, toCoord, capture: boolean, castling: Q | K, promotionPiece: Piece, disambiguation: file | rank | { file, rank } }
  */
  #[derive(Debug)]
pub enum Piece {
    N,
    B,
    R,
    Q,
    K,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::N => write!(f, "Knight"),
            Piece::B => write!(f, "Bishop"),
            Piece::R => write!(f, "Rook"),
            Piece::Q => write!(f, "Queen"),
            Piece::K => write!(f, "King"),
        }
    }
}

/// Describes castling King or Queen side for a given move
#[derive(Debug)]
enum CastlingSide {
    K,
    Q,
}

/**
 * An efficient way to denote a move that is not "proper move notation"
 */
#[derive(Debug)]
struct Move {
    from: Coord,
    to: Coord,
    capture: bool,
    en_passant: bool, // just a bool, as the Position holds the en_passant_target
    castling: Option<Castling>, // optional
    promotion: Option<Piece>, // optional
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let delim = if self.capture { "X" } else { "->" };
        let promo = match &self.promotion {
            Some(p) => format!("= {p}"),
            None            => String::from(""),
        };
        write!(f, "{} {} {} {}", self.from, delim, self.to, promo)
    }
}

pub fn get_piece_movements(position: &Position) -> /*Vec<Move>*/ () {

    // todo get piece on coord


    // todo get direction from colour of piece
    let direction = match position.active_colour {
        Side::White => 1,
        Side::Black => -1,
    };

    let mut movements = vec![];

    for i in 0..64 {

        // dont get the movements if it's not the right colour:
        if (direction > 0 && !position.is_white(i)) || (direction < 0 && position.is_white(i)) {
            // Not our piece
            continue;
        }

        if position.is_pawn(i) {
            movements.append(&mut get_pawn_movements(position, direction, i));
        }
        else if position.is_bishop(i) {
            movements.append(&mut get_bishop_movements(position, i));
        }
        else if position.is_knight(i) {
            movements.append(&mut get_knight_movements(position, i));
        }
        else if position.is_rook(i) {
            movements.append(&mut get_rook_movements(position, i));
        }
        else if position.is_queen(i) {
            movements.append(&mut get_queen_movements(position, i));
        }
        else if position.is_king(i) {
            movements.append(&mut get_king_movements(position, direction, i));
        }
    }

    println!("Movements for {}:", if direction > 0 { "white" } else { "black" });
    for m in movements.iter() {
        println!("{}", m);
    }

    position.print();

}

fn get_pawn_movements(position: &Position, direction: i32, index: i32) -> Vec<Move> {
    let mut movements = vec![];

    movements.append(&mut get_pawn_movements_forward(position, direction, coord_from_index(index)));
    movements.append(&mut get_pawn_captures(position, direction, coord_from_index(index)));

    // TODO: check moves don't put our king in check

    movements
}

fn get_bishop_movements(position: &Position, index: i32) -> Vec<Move> {
        let mut movements = vec![];

        for direction in [
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ] {
            movements.append(
                &mut get_moves_in_direction_until_blocked(position, coord_from_index(index), direction)
            );
        }

        movements
}

fn get_knight_movements(position: &Position, index: i32) -> Vec<Move> {
    let mut movements = vec![];
    let coord = coord_from_index(index);

    let move_coords = vec![
        (2,1),
        (1,2),
        (-1,2),
        (-2,1),
        (-2,-1),
        (-1,-2),
        (1,-2),
        (2,-1)
    ];

    for (x,y) in move_coords {
        match coord.to(x, y) {
            Some(to) => {
                // if there's a piece that is not ours, capture available
                let capture = position.has_piece(to.to_index()) && (
                    (position.is_white(coord.to_index()) && position.is_black(to.to_index())) ||
                    (position.is_black(coord.to_index()) && position.is_white(to.to_index()))
                );

                movements.push(
                    Move {
                        from: coord.clone(),
                        to: to.clone(),
                        capture,
                        en_passant: false,
                        castling: None,
                        promotion: None,
                    }
                );
            },
            None => continue,
        }
    }

    movements
}

fn get_rook_movements(position: &Position, index: i32) -> Vec<Move> {
    let mut movements = vec![];

    for direction in [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ] {
        movements.append(
            &mut get_moves_in_direction_until_blocked(position, coord_from_index(index), direction)
        );
    }

    movements
}

fn get_queen_movements(position: &Position, index: i32) -> Vec<Move> {
    let mut movements = vec![];

    movements.append(
        &mut get_rook_movements(position, index)
    );

    movements.append(
        &mut get_bishop_movements(position, index)
    );

    movements
}

fn get_king_movements(position: &Position, direction: i32, index: i32) -> Vec<Move> {
    let mut movements = vec![];

    let directions = [
        (0,1),
        (1,1),
        (1,0),
        (1,-1),
        (0,-1),
        (-1,-1),
        (-1,0),
        (-1,1),
    ];

    for dir in directions {
        if let Some(m) = get_move_in_direction(position, &coord_from_index(index), dir) {
            movements.push(m);
        }
    }

    // castling
    if direction > 0 && file_of_index(index) == 'a' {
        if position.castling.K {
            // TODO: if route to castling is not in line of sight of opposing piece
        }
        if position.castling.Q {
            // TODO: if route to castling is not in line of sight of opposing piece
        }
    } else if direction < 0 && file_of_index(index) == 'h' {
        if position.castling.k {
            // TODO: if route to castling is not in line of sight of opposing piece
        }
        if position.castling.q {
            // TODO: if route to castling is not in line of sight of opposing piece
        }
    }

    movements
}

/// returns a vec of moves in a given direction until blocked by own piece, blocked by piece that can be captured or reaches end of board
fn get_moves_in_direction_until_blocked(position: &Position, coord: Coord, direction: (i32, i32)) -> Vec<Move> {
    let mut movements = vec![];
    let mut current_coord = coord.clone();

    while let Some(next_coord) = current_coord.to(direction.0, direction.1) {
        let (move_option, is_blocked) = get_move_to(position, &coord, &next_coord);
        
        if let Some(m) = move_option {
            movements.push(m);
        }

        if is_blocked {
            break;
        }

        current_coord = next_coord;
    }

    movements
}

fn get_move_in_direction(position: &Position, from: &Coord, direction: (i32, i32)) -> Option<Move> {

    if let Some(to) = from.to(direction.0, direction.1) {
        return get_move_to(position, from, &to).0;
    }

    None
}

fn get_move_to(position: &Position, from: &Coord, to: &Coord) -> (Option<Move>, bool) {
    let next_index = to.to_index();
    let is_blocked = position.has_piece(next_index);
    let has_capturable_piece = is_blocked && (position.is_white(from.to_index()) && position.is_black(next_index)) ||
        (position.is_black(from.to_index()) && position.is_white(next_index));

    if !is_blocked || has_capturable_piece {
        (
            Some(
                Move {
                    from: from.clone(),
                    to: to.clone(),
                    capture: has_capturable_piece,
                    en_passant: false,
                    castling: None,
                    promotion: None,
                }
            ),
            is_blocked
        )
    } else {
        (None, is_blocked)
    }
}

fn get_pawn_captures(position: &Position, direction: i32, coord: Coord) -> Vec<Move> {
    let mut movements = vec![];

    let is_white_promotion_available = direction > 0 && is_rank(coord.to_index(), 7);
    let is_black_promotion_available = direction < 0 && is_rank(coord.to_index(), 2);

    let nw_coord = coord.nw(direction, 1);
    let ne_coord = coord.ne(direction, 1);


    for coord_of_capture_op in [nw_coord, ne_coord] {
        if coord_of_capture_op.is_none() {
            continue;
        }

        let coord_of_capture = coord_of_capture_op.unwrap();
        let index_of_capture = coord_of_capture.to_index();

        // check for captures diagonally
        if position.has_piece(index_of_capture) && (
            (direction > 0 && position.is_black(index_of_capture)) || (direction < 0 && position.is_white(index_of_capture))
        ) {
            // promotion by capture
            if is_white_promotion_available || is_black_promotion_available {
                let mut promotion_moves = get_pawn_promotion_movements(coord.clone(), coord_of_capture, true);
                movements.append(&mut promotion_moves);
            } else {
                movements.push(
                    Move {
                        from: coord.clone(),
                        to: coord_of_capture,
                        capture: true,
                        en_passant: false,
                        castling: None,
                        promotion: None,
                    }
                );
            }
        } else if position.is_en_passant_target(index_of_capture) {
            movements.push(
                Move {
                    from: coord.clone(),
                    to: coord_of_capture,
                    capture: true,
                    en_passant: true,
                    castling: None,
                    promotion: None,
                }
            );
        }
    }
    movements
}

fn get_pawn_movements_forward(position: &Position, direction: i32, coord: Coord) -> Vec<Move> {
    let mut movements = vec![];
    let index = coord.to_index();

    let is_white_promotion_available = direction > 0 && is_rank(index, 7);
    let is_black_promotion_available = direction < 0 && is_rank(index, 2);

    let coord_of_forward_move = coord.n(direction, 1);

    if coord_of_forward_move.is_none() {
        return movements;
    }

    let index_of_forward_move = coord_of_forward_move.as_ref().unwrap().to_index();

    // if the pawn is blocked, we can't go forward 1 or 2 but we might still be able to capture
    if !position.has_piece(index_of_forward_move) {
        // forward promotion
        if is_white_promotion_available || is_black_promotion_available {
            let mut promotion_moves = get_pawn_promotion_movements(coord.clone(), coord_of_forward_move.unwrap().clone(), false);
            movements.append(&mut promotion_moves);
        } else {
            movements.push(
                Move {
                    from: coord.clone(),
                    to: coord_of_forward_move.unwrap().clone(),
                    capture: false,
                    en_passant: false,
                    castling: None,
                    promotion: None,
                }
            );
        }

        // pawn's 1st move can be 2 squares
        let is_white_first_move = direction > 0 && is_rank(index, 2);
        let is_black_first_move = direction < 0 && is_rank(index, 7);

        if is_white_first_move || is_black_first_move {
            if let Some(coord_of_forward_2) = coord.n(direction, 2) {
                if !position.has_piece(coord_of_forward_2.to_index()) {
                    movements.push(
                        Move {
                            from: coord,
                            to: coord_of_forward_2,
                            capture: false,
                            en_passant: false,
                            castling: None,
                            promotion: None,
                        }
                    );
                }
            }
        }
    }
    movements
}

fn get_pawn_promotion_movements(from: Coord, to: Coord, capture: bool) -> Vec<Move> {
    let mut movements = vec![];

    for p in [
        Piece::N,
        Piece::B,
        Piece::R,
        Piece::Q,
        Piece::K,
    ] {
        movements.push(
            Move {
                from: from.clone(),
                to: to.clone(),
                capture,
                en_passant: false,
                castling: None,
                promotion: Some(p),
            }
        );
    }

    movements
}
