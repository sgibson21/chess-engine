use std::ops::{BitAnd, BitOr};
use crate::utils::get_uni_char;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitBoard {

    /// Set a bit at location (x,y)
    ///
    /// Origin is in the top-left, starting at 0, high values of `x` move to the right, high values
    /// of `y` move downward.
    ///
    pub fn set(&mut self, x: u8, y: u8) {

        // 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001 = (0,0)
        // 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 = (7,7)

        // how far from the right should the bit at position 1 be shifted?
        let bit_pos = (y * 8) + x;
        // println!("bit_pos: {}", bit_pos);

        // 64 bit int with single flipped bit in respective position
        let bit_to_or = 1 << bit_pos;
        // println!("\t\t{}", HEADER);
        // println!("bit_to_or:\t{:#066b}", bit_to_or);

        // new board with bit set according to coordinates
        let new_board = self.0 | bit_to_or;
        // println!("Setting:\t{:#066b}", new_board);
        // BitBoard(new_board)
        self.0 = new_board;
        // println!("Board:\t\t{:#066b} {}", (*self).0, (*self).0);
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn print(self) {
        println!("Board:\t{:#066b}", (self).0)
    }
}

// eg: to_asci_board(board, PieceAsci::QUEEN:white)
pub fn to_asci_board(board: BitBoard, asci_piece: u32) -> String {
    let mut output: String = "".to_owned();

    for n in 0..64 {

        let is_present = board.0 >> n & 1 == 1;

        if is_present {
            let asci = get_uni_char(asci_piece);
            output.push_str(&format!("[{asci}]"))
        }
        else {
            output.push_str("[ ]");
        }

        // if end of rank
        if n < 63 && n % 8 == 7 {
            output.push('\n');
        }
    }

    output
}

pub fn asci_board_indicies() -> String {
    let mut output: String = "".to_owned();

    for n in 0..64 {

        output.push_str(&format!("[\t{n}\t]"));

        // if end of rank
        if n < 63 && n % 8 == 7 {
            output.push('\n');
        }
    }

    output
}
