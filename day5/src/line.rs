use std::str::FromStr;
use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once(" -> ").unwrap();
        Ok(Line {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        })
    }
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }
    pub fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }
    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    pub fn is_diagonal(&self) -> bool {
        let inc  = self.start - self.end;
        inc.x.abs() == inc.y.abs()
    }
    pub fn is_horizontal_vertical_or_diagonal(&self) -> bool {
        self.is_horizontal_or_vertical() || self.is_diagonal()
    }
    pub fn point_vec(&self) -> Vec<Point> {
        if self.is_horizontal_vertical_or_diagonal() {
            // println!("Start: {:?} | End: {:?}",self.start, self.end);
            let inc = (self.end - self.start).normalize();
            // println!("Inc: {:?}",inc);
            let mut points = vec![self.start.clone()];
            while points.last().unwrap() != &self.end {
                points.push(*points.last().unwrap() + inc);
                // dbg!(&points);
            }
            points
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod line_tests {
    use crate::point::Point;
    use crate::line::Line;

    #[test]
    fn parse_line_str() {
        let first_str = "0,9 -> 5,9";
        let second_str = "8,0 -> 0,8";
        assert_eq!(first_str.parse::<Line>().unwrap(), Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        });
        assert_eq!(second_str.parse::<Line>().unwrap(), Line {
            start: Point { x: 8, y: 0 },
            end: Point { x: 0, y: 8 },
        });
        assert_eq!("9,7 -> 7,7".parse::<Line>().unwrap(), Line{
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 7 },
        })
    }
    #[test]
    fn normalize_test() {
        let line = "9,7 -> 7,7".parse::<Line>().unwrap();
        line.point_vec();
    }
}