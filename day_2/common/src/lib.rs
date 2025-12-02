pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|range| parse_range(range))
        .collect()
}

pub fn parse_range(range: &str) -> (u64, u64) {
    let mut range = range.split("-");

    let from = range.next().unwrap().parse::<u64>().unwrap();
    let to = range.next().unwrap().parse::<u64>().unwrap();

    assert_eq!(range.next(), None);

    (from, to)
}

pub fn no_of_digits(id: u64) -> u32 {
    id.ilog10() + 1
}


pub const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(TEST_INPUT);

        assert_eq!(result.len(), 11);
        assert_eq!(result[0], (11, 22));
        assert_eq!(result[10], (2121212118, 2121212124));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("11-22"), (11, 22));
    }

    #[test]
    fn test_no_of_digits() {
        assert_eq!(no_of_digits(9), 1);
        assert_eq!(no_of_digits(10), 2);
        assert_eq!(no_of_digits(99), 2);
        assert_eq!(no_of_digits(100), 3);
        assert_eq!(no_of_digits(999), 3);
        assert_eq!(no_of_digits(1000000000), 10);
        assert_eq!(no_of_digits(2147483647), 10);

    }

}