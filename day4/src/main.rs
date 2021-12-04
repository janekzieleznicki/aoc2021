use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use crate::bingo_board::BingoBoard;

mod bingo_board;

fn main() {
    {
        let input = fs::read_to_string("day4/input_data.dat");
        let mut game: BingoGame = input.unwrap().as_str().parse().unwrap();
        println!("Winning score: {}", game.winning_score());
    }
    {
        let input = fs::read_to_string("day4/input_data.dat");
        let mut game: BingoGame = input.unwrap().as_str().parse().unwrap();
        println!("Losing score: {}", game.losing_score());
    }
}

#[derive(Debug)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    numbers: VecDeque<u8>,
}

impl BingoGame {
    pub fn winning_score(&mut self) -> u64 {
        for num in &self.numbers {
            for bingo_board in self.boards.as_mut_slice() {
                match bingo_board.mark(*num) {
                    None => {}
                    Some(res) => return res
                }
            }
        }
        0
    }
    pub fn losing_score(&mut self) -> u64 {
        let mut losing_num = 0;
        while self.boards.len() > 1 {
            losing_num = self.numbers.pop_front().unwrap();
            dbg!("num: {}, board_count: {}",losing_num,self.boards.len());
            let updated_boards = self.boards.iter_mut()
                .filter_map(|board|
                    match board.mark(losing_num) {
                        None => Some(board.clone()),
                        Some(_) => None
                    })
                .collect();
            self.boards = updated_boards;
        }
        dbg!("Last num: {} | \n{:?}",losing_num,&self.boards);
        self.winning_score()
    }
}

impl FromStr for BingoGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("\n\n") {
            Some((numbers, boards)) => Ok(BingoGame {
                numbers: numbers.split(",").into_iter()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect(),
                boards: boards.split("\n\n").into_iter()
                    .map(|boards_str| boards_str.parse().unwrap())
                    .collect(),
            }),
            None => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::BingoGame;

    #[test]
    fn winning_from_test_data() {
        let input = fs::read_to_string("test_data.dat");
        let mut game: BingoGame = input.unwrap().as_str().parse().unwrap();
        println!("{:?}", game);
        assert_eq!(game.winning_score(), 4512);
    }

    #[test]
    fn losing_from_test_data() {
        let input = fs::read_to_string("test_data.dat");
        let mut game: BingoGame = input.unwrap().as_str().parse().unwrap();
        println!("{:?}", game);
        assert_eq!(game.losing_score(), 1924);
    }
}


