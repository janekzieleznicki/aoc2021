use std::ops::Shl;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    if let Ok(file) = File::open("day3/input.dat") {
        part_1(file);
    }
    if let Ok(file) = File::open("day3/input.dat") {
        part_2(file);
    }
}

fn part_1(file: File) {
    let reader = BufReader::new(file);
    let mut res = vec![0; "010011001001".len()];
    let len = reader.lines()
        .map(|line| str_to_numbers(line.unwrap().as_str()))
        .map(|nums| {
            add_elems_to_first(&mut res, nums);
        }
        )
        .count();
    println!("Input len: {}, Result vec: {:?}", len, res);
    let divided: Vec<u8> = res.into_iter()
        .map(|num| (num as f64) / (len as f64))
        .map(|num| num.round() as u8)
        .collect();
    let gamma = vecbit_to_integer(divided.as_slice());
    let mut epsilon_vecbit = divided.clone();
    vecbit_flip(epsilon_vecbit.as_mut_slice());
    let epsilon = vecbit_to_integer(epsilon_vecbit.as_slice());
    println!("After division {:?} | gamma: {} | epsilon: {} | multiple: {}", divided, gamma, epsilon, gamma * epsilon);
}

fn part_2(file: File) {
    let reader = BufReader::new(file);
    let input: Vec<Vec<u8>> = reader.lines()
        .map(|line| str_to_numbers(line.unwrap().as_str()))
        .collect();
    let o2_generator = get_o2_rating(&input);
    let co2_gscrubber = get_co2_rating(&input);
    println!("O2: {} | CO2: {} | multiplied {}", o2_generator, co2_gscrubber, o2_generator * co2_gscrubber);
}

fn get_most_common_bit_at_idx(readings: &Vec<Vec<u8>>, idx: usize) -> u8 {
    let len = readings.len();
    let ones_count = readings.iter().filter(|row| row[idx] == 1).count();
    (ones_count as f64 / len as f64).round() as u8
}

fn get_o2_rating(readings: &Vec<Vec<u8>>) -> u32 {
    let mut readings_copy = readings.clone();
    let mut idx = 0;
    while readings_copy.len() > 1 {
        let common_bit = get_most_common_bit_at_idx(&readings_copy, idx);
        readings_copy = readings_copy.into_iter().filter(
            |row| row[idx] == common_bit
        ).collect();
        idx += 1;
    }
    vecbit_to_integer(readings_copy[0].as_slice())
}

fn get_co2_rating(readings: &Vec<Vec<u8>>) -> u32 {
    let mut readings_copy = readings.clone();
    let mut idx = 0;
    while readings_copy.len() > 1 {
        let common_bit = get_most_common_bit_at_idx(&readings_copy, idx);
        readings_copy = readings_copy.into_iter().filter(
            |row| row[idx] != common_bit
        ).collect();
        idx += 1;
    }
    vecbit_to_integer(readings_copy[0].as_slice())
}

pub fn str_to_numbers<T: FromStr>(str: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug
{
    str.chars()
        .map(|char| char.to_string().parse::<T>().unwrap())
        .collect()
}

fn add_elems_to_first<T: std::ops::AddAssign + Copy>(vec_a: &mut Vec<T>, vec_b: Vec<T>) {
    vec_a.iter_mut().enumerate()
        .for_each(|(idx, elem)| *elem += vec_b[idx])
}

fn vecbit_to_integer(binary: &[u8]) -> u32 {
    let mut res = 0u32;
    binary.iter().rev().enumerate()
        .for_each(|(idx, elem)| res += (*elem as u32).shl(idx) as u32);
    res
}

fn vecbit_flip(binary: &mut [u8]) {
    binary.iter_mut().for_each(|bit| *bit = *bit ^ 1)
}

#[cfg(test)]
mod tests {
    use crate::{add_elems_to_first, get_co2_rating, get_o2_rating, str_to_numbers, vecbit_flip, vecbit_to_integer};

    static TEST_DATA: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn from_test_data() {
        let mut res = vec![0, 0, 0, 0, 0];
        let len = TEST_DATA.lines()
            .map(|line| str_to_numbers(line))
            .map(|nums| {
                add_elems_to_first(&mut res, nums);
            }
            )
            .count();
        println!("Input len: {}, Result vec: {:?}", len, res);
        let divided: Vec<u8> = res.into_iter()
            .map(|num| (num as f64) / (len as f64))
            .map(|num| num.round() as u8)
            .collect();
        let gamma = vecbit_to_integer(divided.as_slice());
        let mut epsilon_vecbit = divided.clone();
        vecbit_flip(epsilon_vecbit.as_mut_slice());
        println!("After division {:?} | gamma: {} | epsilon: {}", divided, gamma, vecbit_to_integer(epsilon_vecbit.as_slice()));
    }

    #[test]
    fn part_2_test() {
        let input: Vec<Vec<u8>> = TEST_DATA.lines()
            .map(|line| str_to_numbers(line))
            .collect();
        assert_eq!(get_o2_rating(&input), 23);
        assert_eq!(get_co2_rating(&input), 10);
    }
}