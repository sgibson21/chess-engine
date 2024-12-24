use crate::board_navigator::Coord;

pub fn get_uni_char(i: u32) -> char {
    std::char::from_u32(i).unwrap_or('�')
}

pub fn is_rank(index: i32, rank: i32) -> bool {
    rank == rank_of_index(index)
}

pub fn coord_from_index(index: i32) -> Coord {
    let rank = rank_of_index(index);
    let file = file_of_index(index);
    Coord(file, rank.try_into().unwrap())
}

pub fn file_of_index(index: i32) -> char {
    match index % 8 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => 'X', // Should not be possible
    }
}

fn rank_of_index(index: i32) -> i32 {
    match index {
        0..=7 =>   8,
        8..=15 =>  7,
        16..=23 => 6,
        24..=31 => 5,
        32..=39 => 4,
        40..=47 => 3,
        48..=55 => 2,
        56..=63 => 1,
        _       => 0,
    }
}
