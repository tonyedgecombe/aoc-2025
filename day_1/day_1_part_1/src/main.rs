use common::{parse, Direction};

fn main() {
    let input = include_str!("../../../data/day_1_part_1.txt");
    let result = no_of_zeros(input);

    println!("Day 1 Part 1: {result}");
}

fn no_of_zeros(input: &str) -> usize {
    let mut position = 50;

    parse(input)
        .iter()
        .map(|entry| {
            match entry.0 {
                Direction::Left => {
                    position = (position - entry.1).rem_euclid(100);
                }
                Direction::Right => {
                    position = (position + entry.1).rem_euclid(100);
                }
            }

            position
        })
        .filter(|position| *position == 0)
        .count()
}



#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA;

    #[test]
    fn test_no_of_zeros() {
        let result = no_of_zeros(TEST_DATA);

        assert_eq!(result, 3);
    }
}