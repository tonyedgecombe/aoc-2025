use std::collections::{HashSet, VecDeque};
use common::parse;

fn main() {
    let result = solve_part_1(include_bytes!("../../../data/day_10.txt"));

    println!("Day 10 Part 1 result: {result}");
}

fn solve_part_1(input: &[u8]) -> usize {
    let (indicator_sets, button_sets, _) = parse(input);

    (0..indicator_sets.len())
        .map(|i| solve_for_line(&indicator_sets[i], &button_sets[i]))
        .sum()
}

fn solve_for_line(indicators: &Vec<i64>, button_sets: &Vec<Vec<i64>>) -> usize {
    let start = vec![0i64; indicators.len()];

    if &start == indicators {
        panic!();
    }

    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((current, depth)) = queue.pop_front() {
        if current == *indicators {
            return depth;
        }

        for buttons in button_sets {
            let mut next = current.clone();

            for button in buttons {
                next[*button as usize] = (next[*button as usize] + 1) % 2;
            }

            if !visited.contains(&next) {
                visited.insert(next.clone());
                queue.push_back((next, depth + 1));
            }

        }
    }

    panic!("Oops, something went wrong!");
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(TEST_DATA);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_solve_for_line() {
        let (indicator_sets, button_sets, _) = parse(TEST_DATA);

        assert_eq!(solve_for_line(&indicator_sets[0], &button_sets[0]), 2);
        assert_eq!(solve_for_line(&indicator_sets[1], &button_sets[1]), 3);
        assert_eq!(solve_for_line(&indicator_sets[2], &button_sets[2]), 2);
    }
}