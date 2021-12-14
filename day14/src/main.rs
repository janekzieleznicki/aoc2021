#![feature(slice_index_methods)]
#![feature(test)]

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::slice::SliceIndex;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn main() {
    let str = fs::read_to_string("day14/input_data.dat").unwrap();
    println!("part1 answer: {}", part2_btree_map(&str,10));
    println!("part2 answer: {}", part2_btree_map(&str,40));
}

fn polymerize(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut polymer: Vec<char> = Vec::with_capacity(template.len() * 2);
    for (left, right) in template.iter().tuple_windows() {
        polymer.push(*left);
        match rules.get(&(*left, *right)) {
            None => {}
            Some(c) => { polymer.push(*c) }
        }
    }
    polymer.push(*template.get(template.len() - 1).unwrap());
    polymer
}

fn calc_part1_res(template: Vec<char>) -> usize {
    let counts = template.into_iter().counts();
    let vals = counts.values().into_iter().sorted().collect_vec();
    **vals.last().unwrap() - **vals.first().unwrap()
}

fn part1(str: &str) -> usize {
    let (template, rules) = str.split_once("\n\n").unwrap();
    let mut template = template.chars().collect::<Vec<char>>();
    let rules = rules.lines().into_iter().map(|line|
        line.split_once(" -> ").unwrap())
        .map(|(pair, insert)| ((pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()), insert.chars().nth(0).unwrap()))
        .collect::<HashMap<(char, char), char>>();

    for _ in 0..10 {
        let counts = template.iter().counts();
        template = polymerize(&template, &rules);
    }
    calc_part1_res(template)
}

fn part2_hashmap(str: &str, steps: usize) -> usize
{
    let (template, rules) = str.split_once("\n\n").unwrap();
    let rules = rules.lines().into_iter().map(|line|
        line.split_once(" -> ").unwrap())
        .map(|(pair, insert)|
            (
                (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                insert.chars().nth(0).unwrap()
            )
        )
        .collect::<HashMap<_,_>>();

    let frequencies = template.chars().tuple_windows().map(|(a, b)| (a, b)).counts();
    let x = (0..steps).fold(frequencies, |frequencies, _| {
        let mut new_frequencies = frequencies.clone();
        for pair @ ((a, b), f) in frequencies {
            if let Some(insertion) = rules.get(&pair.0) {
                // add left pair
                *new_frequencies.entry((a, *insertion)).or_insert(0) += f;
                // add right pair
                *new_frequencies.entry((*insertion, b)).or_insert(0) += f;
                // remove original pair since it got split
                *new_frequencies.entry((a, b)).or_insert(0) -= f;
            }
        }
        new_frequencies
    });
    let mut letter_frequencies = HashMap::new();

    for ((a, b), f) in x {
        letter_frequencies.entry(a).or_insert((0, 0)).0 += f;
        letter_frequencies.entry(b).or_insert((0, 0)).1 += f;
    }

    if let MinMax(min, max) = letter_frequencies.into_iter().map(|(_, (l, r))| l.max(r)).minmax() {
        max - min
    } else {
        unreachable!()
    }
}
fn part2_btree_map(str: &str, steps: usize) -> usize
{
    let (template, rules) = str.split_once("\n\n").unwrap();
    let rules = rules.lines().into_iter().map(|line|
        line.split_once(" -> ").unwrap())
        .map(|(pair, insert)|
            (
                (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                insert.chars().nth(0).unwrap()
            )
        )
        .collect::<BTreeMap<_,_>>();

    let frequencies = template.chars().tuple_windows().map(|(a, b)| (a, b)).counts();
    let x = (0..steps).fold(frequencies, |frequencies, _| {
        let mut new_frequencies = frequencies.clone();
        for pair @ ((a, b), f) in frequencies {
            if let Some(insertion) = rules.get(&pair.0) {
                // add left pair
                *new_frequencies.entry((a, *insertion)).or_insert(0) += f;
                // add right pair
                *new_frequencies.entry((*insertion, b)).or_insert(0) += f;
                // remove original pair since it got split
                *new_frequencies.entry((a, b)).or_insert(0) -= f;
            }
        }
        new_frequencies
    });
    let mut letter_frequencies = BTreeMap::new();

    for ((a, b), f) in x {
        letter_frequencies.entry(a).or_insert((0, 0)).0 += f;
        letter_frequencies.entry(b).or_insert((0, 0)).1 += f;
    }

    if let MinMax(min, max) = letter_frequencies.into_iter().map(|(_, (l, r))| l.max(r)).minmax() {
        max - min
    } else {
        unreachable!()
    }
}
#[cfg(test)]
mod polymer_tests {
    use std::collections::HashMap;
    use std::fs;
    use crate::{part1, part2_hashmap};

    #[test]
    fn with_test_data() {
        let str = fs::read_to_string("test_data.dat").unwrap();
        assert_eq!(part1(&str), 1588);
        assert_eq!(part2_hashmap(&str, 10), 1588);
        assert_eq!(part2_hashmap(&str, 40), 2188189693529);
    }
}

mod bench {
    extern crate test;

    use std::fs;
    use test::Bencher;
    use crate::{part1, part2_btree_map, part2_hashmap};

    #[bench]
    fn naive_part1(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        b.iter(|| {
            part1(&str);
        })
    }
    #[bench]
    fn hashmap_part1(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        b.iter(|| {
            part2_hashmap(&str, 10);
        })
    }
    #[bench]
    fn hashmap(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        b.iter(|| {
            part2_hashmap(&str, 60);
        })
    }

    #[bench]
    fn btreemap(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        b.iter(|| {
            part2_btree_map(&str, 60);
        })
    }
}
