use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::str::Chars;
use crate::Bracket::{Close, Open};
use custom_error::custom_error;
use itertools::Itertools;

fn main() {
    println!("Part 1 : {}", part1(fs::read_to_string("day10/input_data.dat").unwrap().as_str()));
    println!("Part 2 : {}", part2(fs::read_to_string("day10/input_data.dat").unwrap().as_str()));
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' | '[' | '{' | '<' => Some(Open(c)),
            ')' => Some(Close('(')),
            ']' => Some(Close('[')),
            '}' => Some(Close('{')),
            '>' => Some(Close('<')),
            _ => None
        }
    }
    pub fn close(self) -> Option<char> {
        match self {
            Close(_) => None,
            Open('(') => Some(')'),
            Open('{') => Some('}'),
            Open('[') => Some(']'),
            Open('<') => Some('>'),
            _ => panic!()
        }
    }
}

impl fmt::Display for Bracket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

custom_error! {
    #[derive(PartialEq,PartialOrd)]
    pub BracketBalanceError
    WrongCloseError{illegal: char} = "Trying to close with wrong bracket",
    Unbalanced{stack: String} = "Unbalanced stack",
}

pub fn are_brackets_balanced(input: &str) -> Result<(), BracketBalanceError> {
    let mut stack: Vec<Bracket> = vec![];
    for char in input.chars().into_iter() {
        match Bracket::from_char(char) {
            Some(Open(char)) => { stack.push(Bracket::Open(char)) }
            Some(Close(open_br)) => {
                match stack.pop() {
                    None => {}
                    Some(Open(c)) => {
                        if c != open_br { return Err(BracketBalanceError::WrongCloseError { illegal: char }); }
                    }
                    _ => panic!("Should be unreachable")
                }
            }
            None => {}
        }
    }
    if stack.is_empty() {
        Ok(())
    } else {
        Err(BracketBalanceError::Unbalanced {
            stack: stack.into_iter().filter_map(|br| match br {
                Open(c) => Some(c),
                _ => None
            }).collect::<String>()
        })
    }
}

fn part1(str: &str) -> usize {
    let points: HashMap<char, usize> = HashMap::from_iter(vec![
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    str.lines().into_iter()
        .map(are_brackets_balanced)
        .filter_map(|res| match res {
            Ok(_) => None,
            Err(BracketBalanceError::Unbalanced { stack: _ }) => None,
            Err(BracketBalanceError::WrongCloseError { illegal: c }) => Some(c)
        })
        .map(|illegal| points.get(&illegal).unwrap())
        .sum()
}

fn completion_chars(chars: Chars) -> Vec<char> {
    chars.into_iter().rev()
        .filter_map(|c| Bracket::from_char(c))
        .filter_map(|br| br.close())
        .collect()
}
fn part2score(brackets: &str) -> usize{
    let points: HashMap<char, usize> = HashMap::from_iter(vec![
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
    brackets.chars().into_iter()
        .map(|c|points.get(&c).unwrap())
        .fold(0,|acc,point|(acc*5)+point)
}
fn part2(str: &str) -> usize {

    let stack = str.lines().into_iter()
        .map(are_brackets_balanced)
        .filter_map(|res| match res {
            Ok(_) => None,
            Err(BracketBalanceError::Unbalanced { stack: s }) => Some(s),
            Err(BracketBalanceError::WrongCloseError { illegal: _ }) => None
        })
        .map(|stack|completion_chars(stack.chars()).into_iter().collect::<String>())
        .map(|missing_brackets|part2score(missing_brackets.as_str()))
        .sorted().collect::<Vec<usize>>();
    *stack.get((stack.len()-1)/2).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{are_brackets_balanced, BracketBalanceError, completion_chars, part1, part2, part2score};

    static TEST_DATA: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn part1test() {
        assert_eq!(part1(TEST_DATA), 26397);
        assert_eq!(part2(TEST_DATA), 288957);
    }

    #[test]
    fn test_data() {
        assert_eq!(are_brackets_balanced("[]"), Ok(()));
        assert_eq!(are_brackets_balanced("([])"), Ok(()));
        assert_eq!(are_brackets_balanced("{()()()}"), Ok(()));
        assert_eq!(are_brackets_balanced("<([{}])>"), Ok(()));
        assert_eq!(are_brackets_balanced("(((((((((())))))))))"), Ok(()));
        assert_eq!(are_brackets_balanced("[<>({}){}[([])<>]]"), Ok(()));
        assert_eq!(are_brackets_balanced("{([(<{}[<>[]}>{[]{[(<()>"), Err(BracketBalanceError::WrongCloseError { illegal: '}' }));
        assert_eq!(are_brackets_balanced("[[<[([]))<([[{}[[()]]]"), Err(BracketBalanceError::WrongCloseError { illegal: ')' }));
        assert_eq!(are_brackets_balanced("[{[{({}]{}}([{[{{{}}([]"), Err(BracketBalanceError::WrongCloseError { illegal: ']' }));
        assert_eq!(are_brackets_balanced("[<(<(<(<{}))><([]([]()"), Err(BracketBalanceError::WrongCloseError { illegal: ')' }));
        assert_eq!(are_brackets_balanced("<{([([[(<>()){}]>(<<{{"), Err(BracketBalanceError::WrongCloseError { illegal: '>' }));
    }

    #[test]
    fn part2_helpers() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(are_brackets_balanced((line.to_owned() + "}}]])})]").as_str()), Ok(()));
        match are_brackets_balanced(line) {
            Err(BracketBalanceError::Unbalanced { stack: stack }) =>
                assert_eq!(completion_chars(stack.chars()), "}}]])})]".chars().collect::<Vec<char>>()),
            _ => panic!()
        }
        assert_eq!(part2score("}}]])})]"),288957);
        assert_eq!(part2score("}}>}>))))"),1480781);
    }
}