

#[derive(Clone)]
pub struct Coord (pub char, pub u8);

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Coord {

    // Helper functions to get index of move in given direction:
    // North, North East, East, South East, South, South West, West, North West

    pub fn n(&self, direction: i32, count: i32) -> Option<Coord> {
        let next_rank = self.1 as i32 + (count * direction);
        if (1..=8).contains(&next_rank) {
            Some(Coord(self.0, next_rank as u8))
        } else {
            None
        }
    }

    pub fn s(&self, direction: i32, count: i32) -> Option<Coord> {
        let next_rank = self.1 as i32 - (count * direction);
        if (1..=8).contains(&next_rank) {
            Some(Coord(self.0, next_rank as u8))
        } else {
            None
        }
    }

    pub fn e(&self, direction: i32, count: i32) -> Option<Coord> {
        let next_file = match self.file_as_i32() {
            Some(file) => file + (count * direction),
            None => return None,
        };

        i32_as_file(next_file).map(|f| Coord(f, self.1))
    }

    pub fn w(&self, direction: i32, count: i32) -> Option<Coord> {
        let next_file = match self.file_as_i32() {
            Some(file) => file - (count * direction),
            None => return None,
        };

        i32_as_file(next_file).map(|f| Coord(f, self.1))

    }

    pub fn ne(&self, direction: i32, count: i32) -> Option<Coord> {
        match self.n(direction, count) {
            Some(n) => n.e(direction, count),
            None => None,
        }
    }

    pub fn se(&self, direction: i32, count: i32) -> Option<Coord> {
        match self.s(direction, count) {
            Some(s) => s.e(direction, count),
            None => None,
        }
    }

    pub fn sw(&self, direction: i32, count: i32) -> Option<Coord> {
        match self.s(direction, count) {
            Some(s) => s.w(direction, count),
            None => None,
        }
    }

    pub fn nw(&self, direction: i32, count: i32) -> Option<Coord> {
        match self.n(direction, count) {
            Some(n) => n.w(direction, count),
            None => None,
        }
    }

    /// Returns an Option of a Coord by moving x squares North and y squares East
    /// Direction is in white's orientation, with A1 South West and H8 North East
    pub fn to(&self, x: i32, y: i32) -> Option<Coord> {
        match self.n(1, y) {
            Some(n) => n.e(1, x),
            None => None,
        }
    }

    pub fn rank_diff(&self, other: &Coord) -> i32 {
        (self.1 as i32 - other.1 as i32).abs()
    }

    //
    // 0  1  2  3  4  5  6  7
    // 8  9  10 11 12 13 14 15
    // 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31
    // 32 33 34 35 36 37 38 39
    // 40 41 42 43 44 45 46 47
    // 48 49 50 51 52 53 54 55
    // 56 57 58 59 60 61 62 63
    //
    pub fn to_index(&self) -> i32 {
        let index_of_a_file_and_rank = (8 - self.1) * 8;
        index_of_a_file_and_rank as i32 + (self.file_as_i32().unwrap() - 1)
    }

    fn file_as_i32(&self) -> Option<i32> {
        match self.0 {
            'a' => Some(1),
            'b' => Some(2),
            'c' => Some(3),
            'd' => Some(4),
            'e' => Some(5),
            'f' => Some(6),
            'g' => Some(7),
            'h' => Some(8),
            _   => None,
        }
    }
}

#[test]
fn to_index() {
    assert_eq!(Coord('e', 4).to_index(), 36);
}

fn i32_as_file(file: i32) -> Option<char> {
    match file {
        1 => Some('a'),
        2 => Some('b'),
        3 => Some('c'),
        4 => Some('d'),
        5 => Some('e'),
        6 => Some('f'),
        7 => Some('g'),
        8 => Some('h'),
        _ => None,
    }
}
