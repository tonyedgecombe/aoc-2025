use common::{edges, parse};
use std::collections::HashSet;

fn main() {
    let result = solve_part_1(1000, 3, include_str!("../../../data/day_8.txt"));

    println!("Day 8 Part 1 result: {result}");
}


fn solve_part_1(distance_count: usize, circuit_count: usize, input: &str) -> i64 {
    let data = parse(input);

    let mut edges = edges(&data);
    edges.sort_by(|a, b| a.distance.cmp(&b.distance));
    let mut circuits: Vec<_> = edges
        .iter()
        .take(distance_count)
        .map(|connection| HashSet::from([connection.from, connection.to]))
        .collect();

    loop {
        let mut to_merge = None;

        'outer: for i in 0..circuits.len() {
            for j in i + 1.. circuits.len() {
                if circuits[i].intersection(&circuits[j]).count() > 0 {
                    to_merge= Some((i, j));
                    break 'outer;
                }
            }
        }

        match to_merge {
            None => {
                circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
                return circuits.iter().take(circuit_count).map(move |us| us.len() as i64).product()
            },
            Some((i, j)) => {
                let to_move: Vec<_> = circuits[j].iter().map(|v| *v).collect();
                circuits[i].extend(to_move);
                circuits.remove(j);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(10, 3, TEST_DATA);
        assert_eq!(result, 40);
    }
}