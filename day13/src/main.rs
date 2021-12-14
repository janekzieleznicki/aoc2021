use std::collections::VecDeque;
use std::{error, fs};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use ndarray::{Array2, Axis, Dim};
use regex::Regex;
use custom_error::custom_error;
use crate::ParsingError::{InstructionParsingErro, OrigamiParsingErro};
use ansi_term::Colour::Black;
use ansi_term::Colour::White;



fn main() {
    let str = fs::read_to_string("day13/input_data.dat");
    let mut origami: Origami = str.unwrap().parse().unwrap();
    origami.points.get((1310, 892)).unwrap();
    origami.fold();
    // println!("Part1: {}", origami.visible_dots());
    while !origami.instructions.is_empty() {
        origami.fold();
    }
    println!("part2: {}", origami.visible_dots());
    println!("{}", origami);
}

custom_error! {ParsingError
    InstructionParsingErro = "",
    OrigamiParsingErro = "",
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Up {
        idx: usize
    },
    Left {
        idx: usize
    },
}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^fold along (?P<axis>\w)=(?P<val>\d+)")?;
        let caps = re.captures(s).unwrap();

        let res = match &caps["axis"] {
            "x" => {
                Self::Left { idx: caps["val"].parse::<usize>()? }
            }
            "y" => {
                Self::Up { idx: caps["val"].parse::<usize>()? }
            }
            _ => return Err(Box::new(InstructionParsingErro))
        };
        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct Origami {
    points: Array2<bool>,
    instructions: VecDeque<Instruction>,
}

impl FromStr for Origami {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once("\n\n").ok_or(OrigamiParsingErro)?;

        let points = parts.0.lines().into_iter().map(|pt|
            {
                let (x, y) = pt.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            }).collect::<Vec<(usize, usize)>>();
        let mut arr = Array2::from_elem(Dim([
            *points.iter().map(|(x, _)| x).max().unwrap() + 1,
            *points.iter().map(|(_, y)| y).max().unwrap() + 1
        ]), false);
        for pt in points.into_iter() {
            *arr.get_mut(pt).unwrap() = true;
        }
        let insts = parts.1.lines().into_iter().map(|ins| ins.parse().unwrap()).collect::<VecDeque<Instruction>>();
        Ok(Self {
            points: arr,
            instructions: insts,
        })
    }
}

impl Display for Origami {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for col in self.points.columns() {
            for pt in col {
                match pt {
                    true => write!(f, "{}", White.paint("█"))?,
                    false => write!(f, "{}", Black.paint("█"))?,
                };
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

impl Origami {
    pub fn fold(&mut self) {
        let ins = self.instructions.pop_front().unwrap();
        match ins {
            Instruction::Up { idx } => {
                println!("Fold UP | Dim: {:?} | idx: {}", self.points.dim(), idx);
                for mut row in self.points.rows_mut().into_iter() {
                    let down = row.iter().skip(idx+1).copied().collect::<Vec<bool>>().into_iter();
                    row.iter_mut()
                        .take(idx )
                        .rev()
                        .zip(down)
                        .into_iter()
                        .for_each(|(top, down)| *top = *top | down)
                }
                self.points = self.points.view().split_at(Axis(1), idx).0.into_owned()
            }
            Instruction::Left { idx } => {
                println!("Fold LEFT | Dim: {:?} | idx: {}", self.points.dim(), idx);
                for mut col in self.points.columns_mut().into_iter() {
                    let right = col.iter().skip(idx+1).copied().collect::<Vec<bool>>().into_iter();
                    col.iter_mut()
                        .take(idx)
                        .rev()
                        .zip(right)
                        .into_iter()
                        .for_each(|(top, down)| *top = *top | down)
                }
                self.points = self.points.view().split_at(Axis(0), idx).0.into_owned()
            }
        }
    }
    pub fn visible_dots(&self) -> usize {
        self.points.iter().filter(|p| **p).count()
    }
}

#[cfg(test)]
mod origami_test {
    use crate::{Instruction, Origami};
    static TEST_DATA: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
    #[test]
    fn parse_instruction() {
        assert_eq!("fold along y=7".parse::<Instruction>().unwrap(), Instruction::Up { idx: 7 });
        assert_eq!("fold along x=5".parse::<Instruction>().unwrap(), Instruction::Left { idx: 5 });
    }

    #[test]
    fn parse_origami() {
        let mut origami: Origami = TEST_DATA.parse().unwrap();
        assert_eq!(origami.instructions.len(), 2);
        assert_eq!(origami.points.dim(), (11, 15));
        println!("{}", origami);
        origami.fold();
        println!("{}", origami);
        assert_eq!(origami.visible_dots(), 17);
        origami.fold();
        println!("{}", origami);
    }
}