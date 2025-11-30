#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|entry| {
            let split = entry.split_at(1);
            let direction = if split.0 == "L" {Direction::Left} else {Direction::Right};
            let count = split.1.parse::<i32>().unwrap();
            assert_ne!(count, 0);
            (direction, count)
        }).
        collect()
}

pub const TEST_DATA: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

#[test]
fn test_parse() {
    let result = parse(TEST_DATA);

    assert_eq!(result.len(), 10);

    assert_eq!(result[0], (Direction::Left, 68));
}