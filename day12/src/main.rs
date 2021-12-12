#![feature(test)]

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::str::FromStr;
use ansi_term::Style;

static TEST_DATA: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

fn main() {
    let str = fs::read_to_string("day12/input_data.dat").unwrap();
    let system: CaveSystem = str.parse().unwrap();
    {
        let paths = find_paths(Cave::Start, Cave::End, &system);
        // println!("{:?}", paths);
        println!("Part1 answer: {}", paths.len());
    }
    {
        let paths = find_paths_part2(Cave::Start, Cave::End, &system);
        // println!("{:?}", paths);
        println!("Part2 answer: {}", paths.len());
    }
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small {
        name: String
    },
    Big {
        name: String
    },
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f, "{}", "start"),
            Cave::End => write!(f, "{}", "end"),
            Cave::Small { name: n } => write!(f, "{}", n),
            Cave::Big { name: n } => write!(f, "{}", n),
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f, "{}", Style::new().bold().paint("start")),
            Cave::End => write!(f, "{}", Style::new().bold().paint("end")),
            Cave::Small { name: n } => write!(f, "{}", n),
            Cave::Big { name: n } => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Path(Cave, Cave);


#[derive(Debug, Clone)]
struct CaveSystem {
    paths: HashSet<Path>,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            s if s.to_uppercase().eq(s) => Ok(Cave::Big { name: s.to_string() }),
            s if !s.to_uppercase().eq(s) => Ok(Cave::Small { name: s.to_string() }),
            _ => Err(())
        }
    }
}

impl FromStr for CaveSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paths = HashSet::new();
        for line in s.lines() {
            if let Some((start, end)) = line.split_once('-') {
                let start = start.parse::<Cave>()?;
                let end = end.parse::<Cave>()?;
                paths.insert(Path(start.clone(), end.clone()));
                paths.insert(Path(end, start));
            }
        }
        Ok(Self { paths })
    }
}

impl CaveSystem {
    pub fn is_cave_single(&self, cave: &Cave) -> bool {
        // chcek if it has exactly two paths and reversible
        self.paths.iter()
            .filter(|Path(start, _)| start == cave)
            .count() == 1
    }
}

fn find_paths_impl(finish: Cave, cave_system: &CaveSystem, start_path: Vec<Cave>) -> Vec<Vec<Cave>> {
    let mut valid_paths = Vec::new();

    for Path(start, end) in &cave_system.paths {
        if start == start_path.get(start_path.len() - 1).unwrap() {
            match end {
                Cave::Small { name: _ } if start_path.iter().any(|visited_path| visited_path == end) => continue,
                _ if end == &finish => {
                    let mut new_path = start_path.clone();
                    new_path.push(end.clone());
                    valid_paths.push(new_path)
                }
                Cave::Start => continue,
                _ => {
                    let mut new_path = start_path.clone();
                    new_path.push(end.clone());
                    valid_paths.extend(find_paths_impl(finish.clone(), cave_system, new_path))
                }
            }
        }
    }
    valid_paths
}

fn find_paths(begin: Cave, finish: Cave, cave_system: &CaveSystem) -> Vec<Vec<Cave>> {
    find_paths_impl(finish, cave_system, vec![begin])
}

fn is_path_valid(candidate_path: &Vec<Cave>) -> bool {
    let mut counts: HashMap<Cave, usize> = HashMap::new();
    for cave in candidate_path {
        match cave {
            Cave::Small { name: _ } => {
                let mut count = counts.entry(cave.clone()).or_insert(0);
                *count += 1;
                if *count > 2 { return false; }
            }
            _ => continue
        }
    }
    counts.into_values().filter(|cnt| *cnt >= 2).count() <= 1
}

fn find_paths_part2_impl(finish: Cave, cave_system: &CaveSystem, start_path: Vec<Cave>) -> Vec<Vec<Cave>> {
    let mut valid_paths = Vec::new();

    for Path(start, end) in &cave_system.paths {
        if start == start_path.get(start_path.len() - 1).unwrap() {
            match end {
                Cave::Small { name: _ } if start_path.iter().filter(|visited_cave| **visited_cave == *end).count() >= 2 => continue,
                Cave::Start => continue,
                _ if *end == finish => {
                    let mut new_path = start_path.clone();
                    new_path.push(end.clone());
                    valid_paths.push(new_path)
                }
                _ => {
                    let mut candidate_path = start_path.clone();
                    candidate_path.push(end.clone());
                    if is_path_valid(&candidate_path) {
                        valid_paths.extend(find_paths_part2_impl(finish.clone(), cave_system, candidate_path))
                    }
                }
            }
        }
    }
    valid_paths
}

fn find_paths_part2(begin: Cave, finish: Cave, cave_system: &CaveSystem) -> Vec<Vec<Cave>> {
    find_paths_part2_impl(finish, cave_system, vec![begin])
}

#[cfg(test)]
mod tests {
    use crate::{Cave, CaveSystem, find_paths, find_paths_part2, is_path_valid, TEST_DATA};

    #[test]
    fn parse_cave() {
        assert_eq!("start".parse::<Cave>().unwrap(), Cave::Start);
        assert_eq!("end".parse::<Cave>().unwrap(), Cave::End);
        assert_eq!("A".parse::<Cave>().unwrap(), Cave::Big { name: 'A'.to_string() });
        assert_eq!("b".parse::<Cave>().unwrap(), Cave::Small { name: 'b'.to_string() });
    }

    #[test]
    fn valid_paths() {
        let paths = r#"start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end"#;
        for path_str in paths.lines() {
            let path = path_str.split(',').into_iter().map(|cave| cave.parse::<Cave>().unwrap()).collect();
            println!("Is valid: {} | path: {:?}", is_path_valid(&path), path);
            assert!(is_path_valid(&path));
        }
    }

    #[test]
    fn invalid_paths() {
        let paths = r#"start,A,b,A,b,A,c,A,c,end
start,A,b,b,A,c,c,A,b,A,end
start,A,b,A,c,A,c,A,b,end
start,A,b,A,c,b,A,c,A,c,A,end
"#;
        for path_str in paths.lines() {
            let path = path_str.split(',').into_iter().map(|cave| cave.parse::<Cave>().unwrap()).collect();
            println!("Is valid: {} | path: {:?}", is_path_valid(&path), path);
            assert!(!is_path_valid(&path));
        }
    }

    #[test]
    fn parse_system() {
        let system: CaveSystem = TEST_DATA.parse().unwrap();
        assert!(!system.is_cave_single(&Cave::Small { name: 'b'.to_string() }));
        assert!(system.is_cave_single(&Cave::Small { name: 'c'.to_string() }));
        assert!(system.is_cave_single(&Cave::Small { name: 'd'.to_string() }));
        {
            let paths = find_paths(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 10);
        }
        {
            let paths = find_paths_part2(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 36);
        }
    }
}

mod bench {
    /*
    test bench::part1_input_data ... bench:   8,591,284 ns/iter (+/- 262,300)
    test bench::part1_test_data  ... bench:       3,618 ns/iter (+/- 198)
    test bench::part2_input_data ... bench: 1,127,565,216 ns/iter (+/- 14,505,783)
    test bench::part2_test_data  ... bench:      34,699 ns/iter (+/- 3,515)
    */
    extern crate test;

    use std::fs;
    use test::Bencher;
    use crate::{Cave, CaveSystem, find_paths, find_paths_part2, TEST_DATA};


    #[bench]
    fn part1_test_data(b: &mut Bencher) {
        let system: CaveSystem = TEST_DATA.parse().unwrap();
        b.iter(|| {
            let paths = find_paths(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 10);
        })
    }

    #[bench]
    fn part2_test_data(b: &mut Bencher) {
        let system: CaveSystem = TEST_DATA.parse().unwrap();
        b.iter(|| {
            let paths = find_paths_part2(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 36);
        })
    }

    #[bench]
    fn part1_input_data(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let system: CaveSystem = str.parse().unwrap();
        b.iter(|| {
            let paths = find_paths(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 3779);
        })
    }

    #[bench]
    fn part2_input_data(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let system: CaveSystem = str.parse().unwrap();
        b.iter(|| {
            let paths = find_paths_part2(Cave::Start, Cave::End, &system);
            assert_eq!(paths.len(), 96988);
        })
    }
}