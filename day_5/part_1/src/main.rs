use common::parse;

fn main() {
    let result = count(include_str!("../../../data/day_5.txt"));

    println!("Day 5 Part 1 result {result}");
}

fn count(input: &str) -> usize {
    let (fresh, available) = parse(input);

    available
        .iter()
        .filter(|id| {
            fresh.iter().any(|(from, to)| *id >= from && *id <= to)
        })
        .count()
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_count() {
        let result = count(TEST_DATA);
        assert_eq!(result, 3);
    }
}