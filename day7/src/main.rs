#![feature(test)]

use std::error::Error;
use std::fs;
use std::ops::Div;
use std::vec::IntoIter;

fn main() {
    {
        let str = fs::read_to_string("day7/input_data.dat").unwrap();
        let horizontal_pos: Vec<i64> = str.split(',')
            .into_iter()
            .filter_map(|pos_str| pos_str.parse().ok())
            .collect();
        let fuel = horizontal_pos.iter()
            .map(|crab_pos| {
                horizontal_pos.iter().fold(0, |acc, pos| acc + (pos - crab_pos).abs())
            })
            .min()
            .unwrap();
        println!("Part1 Minimum fuel: {}", fuel);
    }
    {
        let str = fs::read_to_string("day7/input_data.dat").unwrap();
        let horizontal_pos: Vec<i64> = str.split(',')
            .into_iter()
            .filter_map(|pos_str| pos_str.parse().ok())
            .collect();
        let fuel = (*horizontal_pos.iter().min().unwrap()..=*horizontal_pos.iter().max().unwrap()).into_iter()
            .map(|crab_pos| {
                horizontal_pos.iter().fold(0, |acc, pos| acc + calc_cost((pos - crab_pos).abs()))
            }).min().unwrap();
        println!("Part2 Minimum fuel: {}", fuel);


    }
}

pub fn calc_cost(movement: i64) -> i64 {
    (movement * (movement + 1)).div(2)
}

pub fn calc_cost_slow(movement: i64) -> i64 {
    (0..=movement).into_iter().sum()
}

pub fn total_fuel_used(start_pos: &[i64], end_pos: i64) -> i64 {
    start_pos.iter().fold(0, |acc, pos| acc + calc_cost_slow((pos - end_pos).abs()))
}

pub fn get_least_eval_in_range<T: IntoIterator<Item=i64>, F: Fn(i64) -> i64>(iterable: T, fun: F) -> Result<i64, ()> {
    let mut iter = iterable.into_iter();
    let previous = fun(iter.next().ok_or(())?);
    for target in iter {
        if previous < fun(target) {
            return Ok(previous);
        }
    }
    Err(())
}

///Assumes that
pub fn get_minimum_used_fuel(start_pos: &[i64]) -> Result<i64, ()> {
    let mut avg_pos = (start_pos.iter().sum::<i64>() as f64 / start_pos.len() as f64).round() as i64;
    // let mut avg_pos = start_pos[4];
    if total_fuel_used(start_pos, avg_pos - 1) > total_fuel_used(start_pos, avg_pos)
        && total_fuel_used(start_pos, avg_pos + 1) > total_fuel_used(start_pos, avg_pos){
        Ok(total_fuel_used(start_pos, avg_pos))
    }
    else if total_fuel_used(start_pos, avg_pos - 1) > total_fuel_used(start_pos, avg_pos) {
        //go right
        return get_least_eval_in_range(
            (avg_pos..=*start_pos.iter().max().unwrap()),
            |target_pos| total_fuel_used(start_pos, target_pos));
    } else if total_fuel_used(start_pos, avg_pos + 1) > total_fuel_used(start_pos, avg_pos) {
        return get_least_eval_in_range(
            (0..=avg_pos).rev(),
            |target_pos| total_fuel_used(start_pos, target_pos));
    } else {
        Ok(total_fuel_used(start_pos, avg_pos))
    }
}

#[cfg(test)]
mod day7test {
    extern crate test;

    use std::fs;
    use test::Bencher;
    use crate::{calc_cost, calc_cost_slow, get_minimum_used_fuel};

    #[test]
    fn part1() {
        let horizontal_pos: Vec<i16> = "16,1,2,0,4,2,7,1,2,14".split(',')
            .into_iter().filter_map(|pos_str| pos_str.parse().ok()).collect();
        let fuel: i16 = horizontal_pos.iter()
            .map(|crab_pos| {
                horizontal_pos.iter().fold(0, |acc, pos| acc + (pos - crab_pos).abs())
            }).min().unwrap();
        assert_eq!(fuel, 37);
    }

    #[test]
    fn cost() {
        assert_eq!(calc_cost(0), 0);
        assert_eq!(calc_cost(1), 1);
        assert_eq!(calc_cost(2), 3);
        assert_eq!(calc_cost(16 - 5), 66);
        assert_eq!(calc_cost(5 - 1), 10);
        assert_eq!(calc_cost(5 - 2), 6);
        assert_eq!(calc_cost(5), 15);
    }

    #[test]
    fn part2() {
        let horizontal_pos: Vec<i64> = "16,1,2,0,4,2,7,1,2,14".split(',')
            .into_iter().filter_map(|pos_str| pos_str.parse().ok()).collect();
        println!("{:?}", horizontal_pos);
        let fuel = (*horizontal_pos.iter().min().unwrap()..=*horizontal_pos.iter().max().unwrap()).into_iter()
            .map(|crab_pos| {
                horizontal_pos.iter().fold(0, |acc, pos| acc + calc_cost((pos - crab_pos).abs()))
            }).min().unwrap();
        assert_eq!(fuel, 168);
        assert_eq!(get_minimum_used_fuel(horizontal_pos.as_slice()).unwrap(), 168);
    }

    #[bench]
    fn calc_efficient(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let horizontal_pos: Vec<i64> = str.split(',')
            .into_iter()
            .filter_map(|pos_str| pos_str.parse().ok())
            .collect();
        b.iter(|| {
            let fuel = (*horizontal_pos.iter().min().unwrap()..=*horizontal_pos.iter().max().unwrap()).into_iter()
                .map(|crab_pos| {
                    horizontal_pos.iter().fold(0, |acc, pos| acc + calc_cost((pos - crab_pos).abs()))
                }).min().unwrap();
            assert_eq!(fuel, 93397632);
        })
    }

    #[bench]
    fn calc_inefficient(b: &mut Bencher) {
        let str = fs::read_to_string("input_data.dat").unwrap();
        let horizontal_pos: Vec<i64> = str.split(',')
            .into_iter()
            .filter_map(|pos_str| pos_str.parse().ok())
            .collect();
        b.iter(|| {
            assert_eq!(get_minimum_used_fuel(horizontal_pos.as_slice()).unwrap(), 93397632);
        })
    }
}