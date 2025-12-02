use common::{no_of_digits, parse};

fn main() {
    let result = sum_invalid_ids(include_str!("../../../data/day_2.txt"));

    println!("Day 2 Part 2 result: {result}");
}

fn sum_invalid_ids(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|range| sum_invalid_ids_in_range(*range))
        .sum()
}

fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
    (range.0..=range.1)
        .filter(|id| is_invalid_id_2(*id))
        .sum()
}


fn is_invalid_id_2(id: u64) -> bool {
    let digits = no_of_digits(id);

    (1..=(digits / 2))
        .filter(|i| digits % i == 0)
        .any(|i| is_invalid_id_2_part(id, i))
}

fn is_invalid_id_2_part(id: u64, part_size: u32) -> bool {
    let mut id = id;

    let m = 10u64.pow(part_size);
    let length = no_of_digits(id) / part_size;
    let rep = id % m;

    for _ in 0..length {
        if id % m != rep {
            return false;
        }

        id = id / m;
    }

    true
}

#[cfg(test)]
mod test {
    use common::TEST_INPUT;
    use super::*;

    #[test]
    fn test_is_invalid_id_2_part() {
        assert!(is_invalid_id_2_part(1, 1));
        assert!(is_invalid_id_2_part(11, 1));
        assert!(!is_invalid_id_2_part(12, 1));
        assert!(is_invalid_id_2_part(111, 1));
        assert!(is_invalid_id_2_part(1111, 1));
        assert!(is_invalid_id_2_part(1111, 2));
        assert!(!is_invalid_id_2_part(1111, 3));
        assert!(!is_invalid_id_2_part(1212, 1));
        assert!(is_invalid_id_2_part(1212, 2));
        assert!(is_invalid_id_2_part(121212, 2));
        assert!(!is_invalid_id_2_part(121212, 3));
        assert!(!is_invalid_id_2_part(1212123, 3));
    }

    #[test]
    fn test_is_invalid_id_2() {
        assert!(!is_invalid_id_2(1));
        assert!(is_invalid_id_2(11));
        assert!(is_invalid_id_2(111));
        assert!(is_invalid_id_2(1111));
        assert!(is_invalid_id_2(1111));
        assert!(is_invalid_id_2(1212));
        assert!(is_invalid_id_2(1212));
        assert!(is_invalid_id_2(121212));
        assert!(is_invalid_id_2(121212));
        assert!(!is_invalid_id_2(1212123));
        assert!(!is_invalid_id_2(121213));
    }

    #[test]
    fn test_sum_invalid_ids_in_range() {
        assert_eq!(sum_invalid_ids_in_range((11, 11)), 11);
        assert_eq!(sum_invalid_ids_in_range((11, 22)), 33);
        assert_eq!(sum_invalid_ids_in_range((95, 115)), 99+111);
        assert_eq!(sum_invalid_ids_in_range((998, 1012)), 999+1010);
        assert_eq!(sum_invalid_ids_in_range((1188511880, 1188511890)), 1188511885);
        assert_eq!(sum_invalid_ids_in_range((222220, 222224)), 222222);
        assert_eq!(sum_invalid_ids_in_range((1698522, 1698528)), 0);
        assert_eq!(sum_invalid_ids_in_range((446443, 446449)), 446446);
        assert_eq!(sum_invalid_ids_in_range((38593856, 38593862)), 38593859);
        assert_eq!(sum_invalid_ids_in_range((565653, 565659)), 565656);
        assert_eq!(sum_invalid_ids_in_range((824824821, 824824827)), 824824824);
        assert_eq!(sum_invalid_ids_in_range((2121212118, 2121212124)), 2121212121);

        assert_eq!(sum_invalid_ids_in_range((12341234, 12341234)), 12341234);
    }

    #[test]
    fn test_sum_invalid_ids() {
        let result = sum_invalid_ids(TEST_INPUT);
        assert_eq!(result, 4174379265);
    }
}

