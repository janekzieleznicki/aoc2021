use std::collections::HashMap;
use std::fs;
use std::fs::{File, read};
use std::str::Lines;
use itertools::Itertools;


fn get_part_1(lines: Lines) -> usize
{
    lines.map(|line| line.split_once("|").unwrap().1
        .split_whitespace().into_iter()
        .map(|active_wires| active_wires.trim().len())
        .filter(|len| vec![2, 4, 3, 7].contains(len))
        .count())
        .sum()
}

fn find_simple_number(codes: &mut HashMap<u8, String>, number_codes: &mut Vec<String>, number: u8, used_segemnts: usize) {
    codes.insert(number, number_codes.iter().find(|code| code.len() == used_segemnts).unwrap().to_string());
    number_codes.retain(|code| code != codes.get(&number).unwrap());
}

fn negate_segments(str: &str) -> String {
    let mut chars: Vec<char> = "abcdefg".chars().into_iter().collect();
    chars.retain(|c| !str.chars().any(|original| original == *c));
    chars.into_iter().collect()
}

fn decode_numbers(entry: &str) -> usize {
    let mut number_codes: Vec<String> = entry.replace('|', "").split_whitespace()
        .into_iter().map(|str| String::from(str.chars().sorted().rev().collect::<String>())).collect();
    let mut codes: HashMap<u8, String> = HashMap::new();
    // Look for 1
    find_simple_number(&mut codes, &mut number_codes, 1, 2);
    // Look for 7
    find_simple_number(&mut codes, &mut number_codes, 7, 3);
    // Look for 8
    find_simple_number(&mut codes, &mut number_codes, 8, 7);
    // Look for 4
    find_simple_number(&mut codes, &mut number_codes, 4, 4);
    // Look for 6 (6 is number that has a missing segment from 1)
    {
        let six_code = number_codes.as_slice().iter()
            .filter(|code| code.len() == 6)
            .find(|code| negate_segments(code).chars()
                .all(|c| codes.get(&1u8).unwrap().contains(c))).unwrap();
        codes.insert(6, six_code.to_string());
    }
    number_codes.retain(|code| code != codes.get(&6u8).unwrap());
    // Look for '9' ('9' is number with 6 segments and contains all '4' segments)
    {
        let nine_code = number_codes.as_slice().iter()
            .filter(|code| code.len() == 6)
            .find(|code|
                codes.get(&4u8).unwrap().chars().all(|four_segemnt|
                    code.contains(four_segemnt)
                )
            )
            .unwrap();
        codes.insert(9, nine_code.to_string());
    }
    number_codes.retain(|code| code != codes.get(&9u8).unwrap());
    // Look for '9' ('9' is number with 6 segments and contains all '4' segments)
    {
        let zero_code = number_codes.as_slice().iter()
            .find(|code| code.len() == 6)
            .unwrap();
        codes.insert(0, zero_code.to_string());
    }
    number_codes.retain(|code| code != codes.get(&0u8).unwrap());
    // Look for '5' ('5' is number with 5 segments and i missing the same segemnts as '6' and '9'
    {
        let five_code = number_codes.as_slice().iter()
            .filter(|code| code.len() == 5)
            .filter(|code| code.chars().all(|c|
                !negate_segments(codes.get(&6u8).unwrap())
                    .chars()
                    .any(|six_missing| six_missing == c)
            ))
            .find(|code| code.chars().all(|c|
                !negate_segments(codes.get(&9u8).unwrap())
                    .chars()
                    .any(|nine_missing| nine_missing == c)
            ))
            .unwrap();
        codes.insert(5, five_code.to_string());
    }
    number_codes.retain(|code| code != codes.get(&5u8).unwrap());
    // Look for '3' ('3' is number with 5 segments and contains '1' segments
    {
        let three_code = number_codes.as_slice().iter()
            .filter(|code| code.len() == 5)
            .find(|code|
                // code.contains(codes.get(&1u8).unwrap())
                codes.get(&1u8).unwrap().chars().all(
                    |c|code.contains(c)
                )
            )
            .unwrap();
        codes.insert(3, three_code.to_string());
    }
    number_codes.retain(|code| code != codes.get(&3u8).unwrap());
    // Look for '2'
    codes.insert(2, number_codes[0].to_string());
    number_codes.retain(|code| code != codes.get(&2u8).unwrap());
    println!("{:?}\n{:?}", codes, number_codes);
    // Convert output
    entry.split_once('|').unwrap().1.split_whitespace().into_iter()
        .map(|code| code.chars().sorted().rev().collect::<String>())
        .map(|code| codes.iter()
            .find(|(key,val)| val.as_str() == code).unwrap())
        .map(|(key,_)|key.to_string())
        .join("").parse().unwrap()
}

fn get_part_2(lines: Lines) -> usize {
    lines.map(|line| decode_numbers(line)).sum()
}

fn main() {
    let string = fs::read_to_string("day8/input_data.dat").unwrap();
    println!("Result part 1: {}", get_part_1(string.as_str().lines()));
    println!("Result part 2: {}", get_part_2(string.as_str().lines()));
}

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use crate::{get_part_1, get_part_2, negate_segments};

    static INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn tools() {
        assert_eq!(negate_segments("cdfgeb"), "a".to_string());
    }

    #[test]
    fn test_data() {
        assert_eq!(get_part_1(INPUT.lines()), 26);
        assert_eq!(get_part_2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".lines()), 5353);
        assert_eq!(get_part_2(INPUT.lines()), 61229);
    }
}