use common::parse;

fn main() {
    let result = solve_part_2(include_bytes!("../../../data/day_10.txt"));

    println!("Day 10 Part 2 result: {result}");
}

fn solve_part_2(input: &[u8]) -> usize {
    let (indicator_sets, button_sets, _) = parse(input);

    (0..indicator_sets.len())
        .map(|i| solve_for_line(&indicator_sets[i], &button_sets[i]))
        .sum()
}

fn solve_for_line(_indicators: &Vec<i64>, _button_sets: &Vec<Vec<i64>>) -> usize {
    panic!("I'm not using external libraries like Z3 so this will have to wait.")
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_for_line() {
        let (indicator_sets, button_sets, _) = parse(TEST_DATA);

        assert_eq!(solve_for_line(&indicator_sets[0], &button_sets[0]), 10);
        assert_eq!(solve_for_line(&indicator_sets[1], &button_sets[1]), 12);
        assert_eq!(solve_for_line(&indicator_sets[2], &button_sets[2]), 11);

    }

}