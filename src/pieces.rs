
#[derive(Debug, Clone)]
pub enum Side {
    White,
    Black,
}

pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

pub struct PieceAsciSymbol {
    pub white: u32,
    pub black: u32,
}

pub struct PieceAsci;
impl PieceAsci {
    pub const PAWN: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2659,
        black: 0x265F,
    };
    pub const BISHOP: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2657,
        black: 0x265D,
    };
    pub const KNIGHT: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2658,
        black: 0x265E,
    };
    pub const ROOK: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2656,
        black: 0x265C,
    };
    pub const QUEEN: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2655,
        black: 0x265B,
    };
    pub const KING: PieceAsciSymbol = PieceAsciSymbol {
        white: 0x2654,
        black: 0x265A,
    };
}
