// ======================
// Board Navigator Module
// ======================

// modules
mod board_navigator;
mod board_scout;
mod coord;

// exports from module
pub use board_navigator::get_piece_movements;
pub use coord::Coord;

use board_scout::is_attacked;
