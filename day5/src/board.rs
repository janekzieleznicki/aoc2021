use crate::line::Line;

#[derive(Debug, Clone)]
pub struct Board {
    fields: ndarray::Array2<u16>,
}

impl Board {
    pub fn marked_at_least(&self, p0: u16) -> usize {
        self.fields.iter()
            .filter(|&&marks| marks >= p0)
            .count()
    }
    pub fn mark(&mut self, line: Line) {
        assert!(line.is_horizontal_vertical_or_diagonal(), "Only horizontal, vertical or diagonal lines are supported for now");
        line.point_vec().into_iter().for_each(|point| self.fields[[point.x as usize, point.y as usize]] += 1);
    }
    pub fn with_dim(dim: usize) -> Board {
        Board {
            fields: ndarray::Array2::from_elem((dim, dim), 0),
        }
    }
}

#[cfg(test)]
mod board_tests {
    use crate::board::Board;

    #[test]
    fn mark_test() {
        let mut board = Board::with_dim(10);
        board.mark("1,1 -> 1,3".parse().unwrap());
        board.mark("9,7 -> 7,7".parse().unwrap());
        println!("{:?}", board);
        assert_eq!(board.fields[[1, 1]], 1);
        assert_eq!(board.fields[[1, 2]], 1);
        assert_eq!(board.fields[[1, 3]], 1);
        assert_eq!(board.fields[[9, 7]], 1);
        assert_eq!(board.fields[[8, 7]], 1);
        assert_eq!(board.fields[[7, 7]], 1);
    }

    #[test]
    fn part2_mark_test() {
        let mut board = Board::with_dim(10);
        board.mark("1,1 -> 3,3".parse().unwrap());
        board.mark("9,7 -> 7,9".parse().unwrap());
        println!("{:?}", board);
        assert_eq!(board.fields[[1, 1]], 1);
        assert_eq!(board.fields[[2, 2]], 1);
        assert_eq!(board.fields[[3, 3]], 1);
        assert_eq!(board.fields[[9, 7]], 1);
        assert_eq!(board.fields[[8, 8]], 1);
        assert_eq!(board.fields[[7, 9]], 1);
    }
}
