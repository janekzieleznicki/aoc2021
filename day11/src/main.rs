#![feature(int_abs_diff)]

use std::fmt::{Debug, Formatter};
use ndarray::{Array1, Array2};
use ansi_term::Style;

static INPUT_DATA: &str = r#"5433566276
6376253438
8458636316
6253254525
7211137138
1411526532
5788761424
8677841514
1622331631
5876712227"#;

fn main() {
    {
        let mut octopi = parse(INPUT_DATA);
        let mut sum = 0;
        for _ in 0..100 {
            sum += tick(&mut octopi);
        }
        println!("After 100 steps: {}",sum);
    }
    {
        let octopi = parse(INPUT_DATA);
        println!("Synchronized after {} steps",get_step_synchronized(octopi));
    }
}
fn get_step_synchronized(mut octopi: Array2<Octopus>) -> usize{
    for i in 0.. {
        if octopi.len() == tick(&mut octopi){
            return i+1;
        }
    }
    panic!("what happended?")
}
struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    pub fn from_u8(e: u8) -> Self {
        Self {
            energy: e,
            flashed: false,
        }
    }
    pub fn tick(&mut self) {
        self.energy += 1;
    }
    pub fn flash(&mut self) -> bool {
        if self.energy > 9 && !self.flashed {
            self.flashed = true;
            true
        } else {
            false
        }
    }
    pub fn reset(&mut self) {
        if self.flashed {
            self.energy = 0;
            self.flashed = false;
        }
    }
}


impl Debug for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let style = if self.flashed { Style::new().bold() } else { Style::new() };
        write!(f, "{}", style.paint(format!("{}", self.energy)))
    }
}

fn parse(str: &str) -> Array2<Octopus> {
    let size = (str.len() as f64).sqrt() as usize;
    str.lines().into_iter()
        .map(|line|
            line.chars().into_iter()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .map(|num| Octopus::from_u8(num))
                .collect::<Array1<Octopus>>()
        )
        .flatten()
        .collect::<Array1<Octopus>>()
        .into_shape((size, size)).unwrap()
}

fn flash_neighbours(octopi: &mut Array2<Octopus>, idx: (usize, usize)) {
    octopi.indexed_iter_mut()
        .filter(|((row, col), _)| idx.0.abs_diff(*row) <= 1 && idx.1.abs_diff(*col) <= 1)
        .filter(|(i, _)| *i != idx)
        .for_each(|(_, octopus)| if !octopus.flashed { octopus.tick() })
}

fn get_flashing_octopi(octopi: &mut Array2<Octopus>) -> Vec<(usize, usize)> {
    octopi.indexed_iter_mut()
        .filter_map(|(idx, octopus)| if octopus.flash() { Some(idx) } else { None })
        .collect::<Vec<(usize, usize)>>()
}

/// return flash count
fn tick(octopi: &mut Array2<Octopus>) -> usize {
    octopi.iter_mut().for_each(|octopus| octopus.tick());
    // octopi.iter_mut().for_each(|octopus| { octopus.flash(); });
    let mut flashed_indexes = get_flashing_octopi(octopi);
    while !flashed_indexes.is_empty() {
        // println!("Flashing: {:?}", flashed_indexes);
        flashed_indexes.into_iter()
            .for_each(|idx| flash_neighbours(octopi, idx));
        flashed_indexes = get_flashing_octopi(octopi);
    }
    let flashing_octopi = octopi.indexed_iter_mut()
        .filter(|(_, octopus)|
            octopus.flashed
        ).count();
    // println!("Flashing: {}, Octopi: \n{:?}", flashing_octopi, octopi);
    octopi.iter_mut()
        .for_each(|octopus| octopus.reset());
    flashing_octopi
}


#[cfg(test)]
mod test {
    use crate::{get_step_synchronized, Octopus, parse, tick};

    static TEST_DATA: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn octopus_flash() {
        let mut octopus = Octopus::from_u8(8);
        assert!(!octopus.flash());
        octopus.tick();
        octopus.tick();
        assert!(octopus.flash());
        assert!(!octopus.flash());
        assert!(!octopus.flash());
        assert!(!octopus.flash());
    }

    #[test]
    fn test_data() {
        let mut octopi = parse(TEST_DATA);
        println!("{:?}", octopi);
        tick(&mut octopi);
        println!("{:?}", octopi);
        tick(&mut octopi);
        println!("{:?}", octopi);
    }

    #[test]
    fn test_data_res() {
        {
            let mut octopi = parse(TEST_DATA);
            let mut sum = 0;
            for _ in 0..10 {
                sum += tick(&mut octopi);
            }
            assert_eq!(sum, 204);
        }
        {
            let mut octopi = parse(TEST_DATA);
            let mut sum = 0;
            for _ in 0..100 {
                sum += tick(&mut octopi);
            }
            assert_eq!(sum, 1656);
        }
        {
            let octopi = parse(TEST_DATA);
            assert_eq!(get_step_synchronized(octopi),195);
        }
    }

    #[test]
    fn example_data() {
        let mut octopi = parse(r#"11111
19991
19191
19991
11111"#);
        println!("{:?}", octopi);
        assert_eq!(tick(&mut octopi), 9);
        println!("{:?}", octopi);
        assert_eq!(tick(&mut octopi), 0);
    }
}