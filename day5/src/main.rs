use std::fs::File;
use std::io::{BufRead, BufReader};
use board::Board;
use crate::line::Line;

mod line;
mod point;
mod board;

fn main() {
    if let Ok(file) = File::open("day5/input_data.dat") {
        let reader = BufReader::new(file);
        let mut board = Board::with_dim(1000);
        reader.lines()
            .map(|str| str.unwrap().parse::<Line>().unwrap())
            .filter(|line|line.is_horizontal_or_vertical())
            .for_each(|line|board.mark(line));
        println!("Part1 marked points count: {}", board.marked_at_least(2))
    }
    if let Ok(file) = File::open("day5/input_data.dat") {
        let reader = BufReader::new(file);
        let mut board = Board::with_dim(1000);
        reader.lines()
            .map(|str| str.unwrap().parse::<Line>().unwrap())
            .filter(|line|line.is_horizontal_vertical_or_diagonal())
            .for_each(|line|board.mark(line));
        println!("Part2 marked points count: {}", board.marked_at_least(2))
    }
}

#[cfg(test)]
mod main_test{
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::board::Board;
    use crate::line::Line;

    #[test]
    fn with_test_data() {
        if let Ok(file) = File::open("test_data.dat") {
            let reader = BufReader::new(file);
            let mut board = Board::with_dim(10);
            reader.lines()
                .map(|str| str.unwrap().parse::<Line>().unwrap())
                .filter(|line|line.is_horizontal_or_vertical())
                .for_each(|line|board.mark(line));
            assert_eq!(board.marked_at_least(2),5);
        }
    }
    #[test]
    fn part2_with_test_data() {
        if let Ok(file) = File::open("test_data.dat") {
            let reader = BufReader::new(file);
            let mut board = Board::with_dim(10);
            reader.lines()
                .map(|str| str.unwrap().parse::<Line>().unwrap())
                .filter(|line|line.is_horizontal_vertical_or_diagonal())
                .for_each(|line|board.mark(line));
            assert_eq!(board.marked_at_least(2),12);
        }
    }
}
