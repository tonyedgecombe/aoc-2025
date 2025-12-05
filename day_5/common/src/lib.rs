pub fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut split = input.split("\n\n");
    let fresh = split.next().unwrap();
    let available = split.next().unwrap();

    let fresh = fresh
        .lines()
        .map(|line| {
            let mut range = line.split("-").map(|id| id.parse::<i64>().unwrap());
            (range.next().unwrap(), range.next().unwrap())
        })
        .collect();

    let available = available
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    (fresh, available)
}

pub const TEST_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let (fresh, available) = parse(TEST_DATA);

        assert_eq!(fresh.len(), 4);
        assert_eq!(available.len(), 6);
    }
}