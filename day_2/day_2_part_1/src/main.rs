use common::{no_of_digits, parse};

fn main() {
    let input = include_str!("../../../data/day_2.txt");
    let result = sum_invalid_ids(input);

    println!("Day 2 Part 1 result: {result}");
}

fn sum_invalid_ids(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|range| sum_invalid_ids_in_range(*range))
        .sum()
}



fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
    let mut result = 0;
    let mut current = range.0;

    loop {
        if is_invalid_id(current) {
            result += current;
        }

        current = next_valid_id(current);

        if current > range.1 {
            break;
        }
    }

    result
}

fn is_invalid_id(id: u64) -> bool {
    let digits = no_of_digits(id);

    if digits % 2 == 0 {
        // Even no of digits
        let m = 10u64.pow(digits / 2);
        id / m == id % m
    } else {
        false
    }
}

fn next_valid_id(id: u64) -> u64 {
    let digits = no_of_digits(id);

    if digits % 2 == 1 {
        // Odd no of digits

        match digits {
            1 => 11,
            3 => 1010,
            5 => 100100,
            7 => 10001000,
            9 => 1000010000,
            _ => panic!("odd number of digits {digits} out of range"),
        }
    } else {
        // Even no of digits
        let m = 10u64.pow(digits / 2);
        let left_half = id / m;
        let right_half = id % m;

        if left_half > right_half {
            left_half * m + left_half
        } else {
            let next_half = id / m + 1;

            if no_of_digits(next_half) > digits / 2 {
                return next_valid_id(next_half * m);
            }

            next_half * m + next_half
        }
    }
}

#[cfg(test)]
mod test {
    use common::TEST_INPUT;
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(!is_invalid_id(12));
        assert!(is_invalid_id(99));

        assert!(!is_invalid_id(100));
        assert!(!is_invalid_id(999));

        assert!(!is_invalid_id(1009));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(9999));

        assert!(!is_invalid_id(10000));
        assert!(!is_invalid_id(99999));
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(sum_invalid_ids_in_range((11, 22)), 33);
        assert_eq!(sum_invalid_ids_in_range((95, 115)), 99);
        assert_eq!(sum_invalid_ids_in_range((998, 1012)), 1010);
        assert_eq!(sum_invalid_ids_in_range((1188511880, 1188511890)), 1188511885);
        assert_eq!(sum_invalid_ids_in_range((222220, 222224)), 222222);
        assert_eq!(sum_invalid_ids_in_range((1698522, 1698528)), 0);
        assert_eq!(sum_invalid_ids_in_range((446443, 446449)), 446446);
        assert_eq!(sum_invalid_ids_in_range((38593856, 38593862)), 38593859);
        assert_eq!(sum_invalid_ids_in_range((565653, 565659)), 0);
        assert_eq!(sum_invalid_ids_in_range((824824821, 824824827)), 0);
        assert_eq!(sum_invalid_ids_in_range((2121212118, 212121212)), 0);
    }

    #[test]
    fn test_next_valid_id() {
        assert_eq!(next_valid_id(10), 11);
        assert_eq!(next_valid_id(11), 22);
        assert_eq!(next_valid_id(91), 99);
        assert_eq!(next_valid_id(100), 1010);
        assert_eq!(next_valid_id(1001), 1010);
        assert_eq!(next_valid_id(1010), 1111);
        assert_eq!(next_valid_id(10000), 100100);
        assert_eq!(next_valid_id(99999), 100100);
    }

    #[test]
    fn test_part_1() {
        let result = sum_invalid_ids(TEST_INPUT);
        assert_eq!(result, 1227775554);
    }
}