use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use common::{edges, parse};

fn main() {
    let result = solve_part_2(include_str!("../../../data/day_8.txt"));

    println!("Day 8 Part 2 result: {result}");
}


fn solve_part_2(input: &str) -> i64 {
    let data = parse(input);

    let mut edges = edges(&data);
    edges.sort_by(|a, b| a.distance.cmp(&b.distance));

    let mut heap = BinaryHeap::from_iter(edges.iter().map(|edge| Reverse(edge)));

    let mut circuits: Vec<HashSet<usize>> = (0..data.len())
        .map(|i| HashSet::from([i]))
        .collect();

    while let Some(Reverse(edge)) = heap.pop() {
        let mut from = None;
        let mut to = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&edge.from) {
                from = Some(i);
            }
            if circuit.contains(&edge.to) {
                to = Some(i);
            }
        }

        if let (Some(from), Some(to)) = (from, to) {
            if from != to {
                let other = circuits[to].clone();
                circuits[from].extend(other);
                circuits.remove(to);

                if circuits.len() == 1 {
                    return data[edge.from].x * data[edge.to].x;
                }
            }
        }
    }

    panic!();
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(TEST_DATA);

        assert_eq!(result, 25272);
    }
}