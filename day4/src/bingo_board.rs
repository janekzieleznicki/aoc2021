use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct BingoBoard {
    board_numbers: ndarray::Array2<u8>,
    board_markers: ndarray::Array2<bool>,
}

impl BingoBoard {
    /// Marks a number on board and if it has won return result
    pub fn mark(&mut self, num: u8) -> Option<u64> {
        let mut marked_indices = [0, 0];
        self.board_numbers.indexed_iter()
            .find(|((row, col), &entry)|
                if num == entry {
                    marked_indices = [*row, *col];
                    self.board_markers[marked_indices] = true;
                    return true;
                } else {
                    return false;
                });
        let row_full = self.board_markers.row(marked_indices[0])
            .iter().all(|elem| *elem);
        let column_full = self.board_markers.column(marked_indices[1])
            .iter().all(|elem| *elem);
        if row_full || column_full {
            Some(self.calculate_result(num))
        } else {
            None
        }
    }

    pub fn calculate_result(&mut self, num: u8) -> u64 {
        self.unmarked_sum() * num as u64
    }
    fn unmarked_sum(&self) -> u64 {
        self.board_markers.indexed_iter()
            .filter(|((_, _), mark)| !**mark)
            .map(|((row, col), _)| self.board_numbers[[row, col]] as u64)
            .sum()
    }
}

impl Default for BingoBoard {
    fn default() -> Self {
        BingoBoard {
            board_numbers: ndarray::Array2::from_elem((5, 5), 0),
            board_markers: ndarray::Array2::from_elem((5, 5), false),
        }
    }
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = BingoBoard::default();
        s.lines().enumerate()
            .for_each(|(row_num, row_str)| {
                row_str.split_whitespace().into_iter().enumerate()
                    .for_each(|(col_num, col_val)| { board.board_numbers[[row_num, col_num]] = col_val.parse().unwrap() })
            });
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::bingo_board::BingoBoard;

    static BOARD_GAMES: [&str; 3] = [
        r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"#,
        r#" 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#,
        r#"14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#];

    #[test]
    fn from_str() {
        let mut board: BingoBoard = BOARD_GAMES[0].parse().unwrap();
        assert_eq!(board.board_numbers, ndarray::arr2(&[
            [22, 13, 17, 11, 0, ],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7, ],
            [6, 10, 3, 18, 5, ],
            [1, 12, 20, 15, 19, ],
        ]));
        board.mark(22);
        board.mark(1);
        board.mark(9);
        assert_eq!(board.board_markers, ndarray::arr2(&[
            [true, false, false, false, false, ],
            [false, false, false, false, false, ],
            [false, true, false, false, false, ],
            [false, false, false, false, false, ],
            [true, false, false, false, false, ],
        ]));
        assert_eq!(board.mark(8), None);
        assert_eq!(board.mark(21), None);
        assert!(board.mark(6).is_some());
    }

    #[test]
    fn example_data() {
        let mut board1: BingoBoard = BOARD_GAMES[0].parse().unwrap();
        let mut board2: BingoBoard = BOARD_GAMES[1].parse().unwrap();
        let mut winning_board: BingoBoard = BOARD_GAMES[2].parse().unwrap();
        let inputs: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        inputs[..(inputs.len() - 1)].iter().for_each(|num| {
            println!("Marking {}",num);
            assert_eq!(board1.mark(*num), None);
            assert_eq!(board2.mark(*num), None);
            assert_eq!(winning_board.mark(*num), None);
        });
        assert_eq!(board1.mark(inputs[inputs.len()-1]), None);
        assert_eq!(board2.mark(inputs[inputs.len()-1]), None);
        assert_eq!(winning_board.mark(inputs[inputs.len()-1]), Some(4512));
    }
}