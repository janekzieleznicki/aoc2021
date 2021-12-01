use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main() {
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        let data: Vec<i32> = reader.lines()
            .filter_map(|line| line.unwrap().parse().ok())
            .collect();
        println!("Growing depth on {} items", how_many_grown(data.as_slice()));
        println!("Growing avaraged depth on {} items", how_many_grown(sum_three_window(data.as_slice()).as_slice()));
    }
}
pub fn sum_three_window(input: &[i32]) -> Vec<i32> {
    input
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect()
}

pub fn how_many_grown(input: &[i32]) -> usize {
    input
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|diff| diff > &0)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{how_many_grown, sum_three_window};

    #[test]
    fn test_data() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(how_many_grown(&data[..]),7);
        assert_eq!(how_many_grown(&sum_three_window(&data[..])[..]), 5);
    }
}
