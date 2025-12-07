use std::collections::HashMap;
use common::parse;

fn main() {
    let data = parse(include_bytes!("../../../data/day_7.txt"));

    let result = count(data);

    println!("Day 7 Part 2 result: {result}");
}

fn count(data: Vec<Vec<u8>>) -> usize {
    let col = data[0].iter().position(|c| *c == b'S').unwrap();

    let mut map = HashMap::new();
    follow_path_m(&data, 1, col, &mut map)
}

fn follow_path_m(data: &Vec<Vec<u8>>, row: usize, col: usize, m: &mut HashMap<(usize, usize), usize>) -> usize {
    match m.get(&(row, col)) {
        None => {
            let count = follow_path(data, row, col, m);
            m.insert((row, col), count);
            count
        }
        Some(count) => *count
    }
}

fn follow_path(data: &Vec<Vec<u8>>, row: usize, col: usize, m: &mut HashMap<(usize, usize), usize>) -> usize {
    if row >= data.len() {
        return 1;
    }

    match data[row][col] {
        b'^' => follow_path_m(data, row + 1, col - 1, m) + follow_path_m(data, row + 1, col + 1, m),
        b'.' => follow_path_m(data, row + 1, col, m),
        _ => panic!("Unrecognised character {}", data[row][col])
    }
}

#[cfg(test)]
mod test {
    use common::{parse, TEST_DATA};
    use super::*;

    #[test]
    fn test_count() {
        let data = parse(TEST_DATA);

        let result = count(data);

        assert_eq!(result, 40);
    }
}
