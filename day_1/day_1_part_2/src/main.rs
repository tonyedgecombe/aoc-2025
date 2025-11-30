use common::{parse, Direction};

fn main() {
    let input = include_str!("../../../data/day_1_part_1.txt");
    let result = part_2(input);

    println!("Day 1 Part 2: {result}");
}

fn part_2(input: &str) -> i32 {
    let mut position = 50;

    parse(input).
        iter()
        .map(|entry| {
            let movement: i32 = match entry.0 {
                Direction::Left => -entry.1,
                Direction::Right => entry.1,
            };

            let new_position = position + movement;

            let result = if new_position == 0 {
                1
            }
            else {
                if position != 0 && new_position < 0 {
                    new_position.abs() / 100 + 1
                } else {
                    new_position.abs() / 100
                }
            };

            position = new_position.rem_euclid(100);

            result
        })
        .sum()
}



#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA;

    #[test]
    fn test_no_of_zeros() {
        let result = part_2(TEST_DATA);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_no_of_zeros_r1000() {
        let input = "R1000";

        let result = part_2(input);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_no_of_zeros_r100() {
        let input = "R100";

        let result = part_2(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_of_zeros_r50() {
        let input = "R50";

        let result = part_2(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_of_zeros_r50l5() {
        let input = "R50\nL5";

        let result = part_2(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_of_zeros_r50r5() {
        let input = "R50\nR5";

        let result = part_2(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_of_zeros_r50r100() {
        let input = "R50\nR100";

        let result = part_2(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_no_of_zeros_r50l150() {
        let input = "R50\nL105";

        let result = part_2(input);

        assert_eq!(result, 2);
    }
}