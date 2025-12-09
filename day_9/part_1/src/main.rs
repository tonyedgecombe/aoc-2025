use common::{area, parse};

fn main() {
    let result = solve_part_1(include_str!("../../../data/day_9.txt"));

    println!("Day 9 Part 1 result: {result}");
}

fn solve_part_1(input: &str) -> i64 {
    let co_ords = parse(input);

    co_ords
        .iter()
        .enumerate()
        .flat_map(|(i, c1)| (i+1..co_ords.len()).map(|j| area(*c1, co_ords[j])))
        .max()
        .unwrap()
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(TEST_DATA);

        assert_eq!(result, 50);
    }
}