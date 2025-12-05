use std::cmp::Ordering;
use common::parse;

fn main() {
    let result = count(include_str!("../../../data/day_5.txt"));

    println!("Day 5 Part 2 result: {result}");
}

fn count(input: &str) -> i64 {
    let (mut fresh, _) = parse(input);

    fresh.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        }
    });

    fresh
        .iter()
        .fold((0, 0), |(max, result), (from, to)| {
            if *to >= max {
                (to + 1, result + to - i64::max(*from, max) + 1)
            } else {
                (max, result)
            }
        })
        .1
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_count() {
        let result = count(TEST_DATA);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_count_duplicates_1() {
        let result = count("1-5\n1-1\n\n");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_count_duplicates_2() {
        let result = count("1-1\n1-5\n\n");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_count_no_overlap() {
        let result = count("1-10\n21-30\n\n");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_count_no_overlap_reverse() {
        let result = count("21-30\n1-10\n\n");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_count_overlap() {
        let result = count("1-10\n5-20\n\n");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_count_overlap_reverse() {
        let result = count("5-20\n1-10\n\n");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_count_dup() {
        let result = count("1-20\n1-20\n\n");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_count_triple() {
        let result = count("10-15\n10-20\n10-15\n\n");
        assert_eq!(result, 11);
    }
}