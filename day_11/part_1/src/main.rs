use std::collections::HashMap;
use common::parse;

fn main() {
    let result = solve_part_1(include_str!("../../../data/day_11.txt"));

    println!("Day 11 Part 1 result: {result}");
}

fn solve_part_1(input: &str) -> usize{
    let data = parse(input);

    recurse(&data, "you")
}

fn recurse(data: &HashMap<&str, Vec<&str>>, label : &str) -> usize {
    if label == "out" {
        1
    } else {
        data
            .get(label)
            .unwrap()
            .iter()
            .map(|entry| recurse(data, entry))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(TEST_DATA);

        assert_eq!(result, 5);
    }
}