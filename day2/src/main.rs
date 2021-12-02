use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    if let Ok(file) = File::open("day2/part1-commands.dat") {
        let reader = BufReader::new(file);
        let mut position = Position::default();
        let mut submarine = Submarine::default();
        reader.lines()
            .map(|line| line.unwrap().parse::<Movement>().unwrap())
            .for_each(|mov| {
                position.move_by(mov.clone());
                submarine.move_by(mov);
            });

        println!("Final {:?} with result {}",position,position.result());
        println!("Final {:?} with result {}",submarine,submarine.position.result());
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Position {
    x: i32,
    depth: i32,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Submarine {
    position: Position,
    aim: i32
}
#[derive(Clone)]
enum Movement {
    Horizontal(i32),
    Vertical(i32),
}

impl Position {
    pub fn default() -> Position {
        Position {
            x: 0,
            depth: 0,
        }
    }
    pub fn move_by(&mut self, movement: Movement) -> &mut Self {
        match movement {
            Movement::Horizontal(inc) => self.x += inc,
            Movement::Vertical(inc) => self.depth += inc
        }
        self
    }
    pub fn result(&self) -> i32 {
        self.x * self.depth
    }
}
impl Submarine {
    pub fn default() -> Submarine {
        Submarine {
            position: Position::default(),
            aim: 0
        }
    }
    pub fn move_by(&mut self, movement: Movement) -> &mut Self {
        match movement {
            Movement::Vertical(inc) => self.aim +=inc,
            Movement::Horizontal(inc) => {
                self.position.x += inc;
                self.position.depth += inc*self.aim
            }
        }
        self
    }
}
impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split_once(' ');
        if let Some((cmd, inc)) = s.split_once(' ') {
            match cmd {
                "up" => return Ok(Movement::Vertical(-(inc.parse::<i32>().unwrap()))),
                "down" => return Ok(Movement::Vertical(inc.parse().unwrap())),
                "forward" => return Ok(Movement::Horizontal(inc.parse().unwrap())),
                &_ => return Err(())
            }
        }
        Err(())
    }
}



#[cfg(test)]
mod test {
    use crate::{Movement, Position, Submarine};

    #[test]
    fn default_input() {
        let str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        let mut position = Position::default();
        let mut submarine = Submarine::default();
        for line in str.lines() {
            println!("Line: {}", line);
            position.move_by(line.parse::<Movement>().unwrap());
            submarine.move_by(line.parse::<Movement>().unwrap());
        }
        assert_eq!(position.x, 15);
        assert_eq!(position.depth, 10);
        assert_eq!(position.result(), 150);

        assert_eq!(submarine.position.x,15);
        assert_eq!(submarine.position.depth,60);
        assert_eq!(submarine.position.result(),900);
    }
}