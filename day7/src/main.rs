use std::fs;
use std::ops::Div;

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

#[cfg(test)]
mod test {
    use crate::calc_cost;

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
    }
}