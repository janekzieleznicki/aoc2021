#![feature(int_abs_diff)]
#![feature(test)]

use std::fs;
use std::str::Lines;
use ndarray::{Array1, Array2};
use fs::read_to_string;
use std::collections::HashSet;
use itertools::{Itertools};

fn main() {
    let str = read_to_string("day9/input_data.dat").unwrap();
    let arr = parse(str.lines());
    println!("Part 1: {}", part_1(&arr));
    println!("Part 2: {}", part_2(&arr));
}

pub fn parse(lines: Lines) -> Array2<u8> {
    let vec = lines.map(|line|
        line.chars().into_iter().map(|num| num.to_string().parse().unwrap()).collect()
    ).collect::<Vec<Vec<u8>>>();
    let mut arr = Array2::default([vec.len(), vec[0].len(), ]);
    arr.rows_mut().into_iter()
        .zip(vec)
        .for_each(|(mut row, parsed)| row.assign(&Array1::from_iter(parsed.into_iter())));
    arr
}

fn is_min(previous: Option<&u8>, x: &u8, next: Option<&u8>) -> bool {
    match (previous, next) {
        (Some(p), Some(n)) => p > x && x < n,
        (None, Some(n)) => x < n,
        (Some(p), None) => p > x,
        _ => { panic!() }
    }
}

fn explore_from_point(arr: &Array2<u8>, row_start: usize, col_start: usize) -> HashSet<(usize, usize)> {
    arr.indexed_iter()
        // .filter(|((row,_),_)| row_start.abs_diff(*row) <= 1)
        .filter(|((row, col), _)| col_start.abs_diff(*col) <= 1 && row_start.abs_diff(*row) <= 1)
        .filter(|((row, col), _)|
            (col_start.abs_diff(*col) == 0 && row_start.abs_diff(*row) == 0) ||
                col_start.abs_diff(*col) != row_start.abs_diff(*row)
        )
        .filter(|(_, &val)| val != 9)
        .map(|(idx, _)| idx)
        .collect()
}

fn basin_area(arr: &Array2<u8>, row: usize, col: usize) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    points.insert((row, col));
    let mut prev_len = 0;
    while points.len() > prev_len {
        prev_len = points.len();
        points = points.iter()
            .map(|(row, col)| explore_from_point(&arr, *row, *col))
            .flatten()
            .collect();
        println!("points: {:?}", points);
    }
    points.len()
}

pub fn part_1(arr: &Array2<u8>) -> usize {
    arr.indexed_iter()
        .filter(|((x, y), val)| {
            let prev = if x > &0 { arr.get([x - 1, *y]) } else { None };
            is_min(prev, val, arr.get([x + 1, *y]))
        }
        )
        .filter(|((x, y), val)| {
            let prev = if y > &0 { arr.get([*x, y - 1]) } else { None };
            is_min(prev, val, arr.get([*x, y + 1]))
        }
        ).map(|(_, height)| (height + 1) as usize).sum()
}

pub fn part_2(arr: &Array2<u8>) -> usize {
    arr.indexed_iter()
        .filter(|((x, y), val)| {
            let prev = if x > &0 { arr.get([x - 1, *y]) } else { None };
            is_min(prev, val, arr.get([x + 1, *y]))
        }
        )
        .filter(|((x, y), val)| {
            let prev = if y > &0 { arr.get([*x, y - 1]) } else { None };
            is_min(prev, val, arr.get([*x, y + 1]))
        }
        ).map(|((x, y), _)| basin_area(&arr, x, y))
        .sorted()
        .rev()
        .take(3)
        .fold(1,|acc, val|acc*val)
}

#[cfg(test)]
mod tests {
    extern crate test;
    use std::fs;
    use test::Bencher;
    use crate::{basin_area, parse, part_1, part_2};

    static TEST_DATA: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn with_test_data() {
        let arr = parse(TEST_DATA.lines());
        println!("{:#?}", arr);
        assert_eq!(part_1(&arr), 15);
        assert_eq!(part_2(&arr), 1134)
    }

    #[test]
    fn basin_area_data() {
        let arr = parse(TEST_DATA.lines());
        println!("{:#?}", arr);
        assert_eq!(basin_area(&arr,0,0),3);
        assert_eq!(basin_area(&arr, 0, 8), 9);
        assert_eq!(basin_area(&arr, 3, 3), 14);
    }

    #[bench]
    fn day9_part1(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let arr = parse(str.lines());
        b.iter(|| {
            assert_eq!(part_1(&arr), 439);
        })
    }
    #[bench]
    fn day9_part2(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let arr = parse(str.lines());
        b.iter(|| {
            assert_eq!(part_2(&arr), 900900);
        })
    }
}