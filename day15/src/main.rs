#![feature(int_abs_diff)]
#![feature(mixed_integer_ops)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use itertools::Itertools;
use ndarray::{array, Array1, Array2, Axis, concatenate};

fn main() {
    use std::time::Instant;
    {
        let str = fs::read_to_string("day15/input_data.dat").unwrap();
        let arr = parse(str.as_str());
        let now = Instant::now();
        let ans = find_shortest_path_cost(arr);
        println!("Elapsed: {} | Part1 with puzzle input: {}", now.elapsed().as_millis(), ans);
    }
    {
        let str = fs::read_to_string("day15/input_data.dat").unwrap();
        let arr = part2_tiles(parse(str.as_str()));
        let now = Instant::now();
        let ans = find_shortest_path_cost(arr);
        println!("Elapsed: {} | Part2 with puzzle input: {}", now.elapsed().as_millis(), ans);
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Path {
    nodes: Vec<(usize, usize)>,
    cost: usize,
}

fn parse(str: &str) -> Array2<u8> {
    let row_len = str.lines().peekable().peek().unwrap().len();
    let col_len = str.lines().count();
    let arr = str.lines().into_iter().map(|line|
        line.chars().into_iter().map(|c| c.to_string().parse::<u8>().unwrap())
    )
        .flatten()
        .collect::<Array1<u8>>()
        .into_shape((col_len, row_len)).unwrap();
    arr
}

fn explore_from_node(node: (usize, usize)) -> Vec<(usize, usize)> {
    array![(0,-1),(-1,0),(0,1),(1,0),].into_iter()
        .filter(|(x, _)| node.0.checked_add_signed(*x).is_some())
        .filter(|(_, y)| node.1.checked_add_signed(*y).is_some())
        .map(|(x, y)| (node.0.checked_add_signed(x).unwrap(), node.1.checked_add_signed(y).unwrap()))
        .collect_vec()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathNode {
    cost: usize,
    position: (usize, usize),
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path_cost(arr: Array2<u8>) -> usize {
    let mut costs = Array2::from_elem(arr.dim(), usize::MAX);

    let target = (arr.dim().0 - 1, arr.dim().1 - 1);

    let mut heap = BinaryHeap::new();

    costs[(0, 0)] = 0;
    heap.push(PathNode { cost: 0, position: (0, 0) });

    while let Some(PathNode { cost, position }) = heap.pop() {
        if position == target { return cost; }
        if cost > *costs.get(position).unwrap() { continue; }

        for pos in explore_from_node(position).into_iter() {
            if let Some(&risk) = arr.get(pos) {
                let next = PathNode { cost: cost + risk as usize, position: pos };
                if next.cost < costs[next.position] {
                    heap.push(next);
                    costs[next.position] = next.cost
                }
            }
        }
    }
    unreachable!()
}

fn part2_tiles(mut arr: Array2<u8>) -> Array2<u8> {
    let orig_dim = arr.dim();
    for i in 0..4 {
        let mut right_arr = arr.view().split_at(Axis(1), orig_dim.1 * i).1.clone().to_owned();
        increment_elem_wrapping(&mut right_arr);
        arr = concatenate!(Axis(1),arr,right_arr);
    }
    for i in 0..4 {
        let mut bottom_arr = arr.view().split_at(Axis(0), orig_dim.0 * i).1.clone().to_owned();
        increment_elem_wrapping(&mut bottom_arr);
        arr = concatenate!(Axis(0),arr,bottom_arr);
    }
    arr
}

fn increment_elem_wrapping(bottom_arr: &mut Array2<u8>) {
    bottom_arr.iter_mut().for_each(|elem| {
        *elem += 1;
        if *elem > 9 { *elem = 1; }
    });
}

#[cfg(test)]
mod risky_caves_test {
    use ndarray::SliceInfoElem::Index;
    use crate::{explore_from_node, find_shortest_path_cost, parse, part2_tiles};
    static TEST_DATA: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
    #[test]
    fn dim_playground() {
        let dim: (usize, usize) = (0, 0);
        assert!(explore_from_node(dim).contains(&(0, 1)));
        assert!(explore_from_node(dim).contains(&(1, 0)));
        assert!(explore_from_node((1, 1)).contains(&(0, 1)));
        assert!(explore_from_node((1, 1)).contains(&(1, 2)));
        assert!(explore_from_node((1, 1)).contains(&(1, 0)));
        assert!(explore_from_node((1, 1)).contains(&(2, 1)));
    }

    #[test]
    fn parse_test_data() {
        let arr = parse(TEST_DATA);
        println!("Arr: {:?}", arr);
        let new_arr = part2_tiles(arr.clone());
        assert_eq!(find_shortest_path_cost(arr), 40);
        assert_eq!(find_shortest_path_cost(new_arr), 315);
    }
}