use common::{parse, removable_rolls};

fn main() {
    let mut input = parse(include_bytes!("../../../data/day_4.txt"));

    let result = count_removable_rolls(&mut input);

    println!("Day 4 Part 2 example: {result}");
}

fn count_removable_rolls(input: &mut Vec<Vec<u8>>) -> usize {
    let mut result = 0;

    loop {
        let count = remove_rolls(input);
        if count == 0 {
            break;
        }

        result += count;
    }

    result
}

fn remove_rolls(input: &mut Vec<Vec<u8>>) -> usize {
    let rolls = removable_rolls(input);

    for (r, c) in rolls.iter() {
        input[*r][*c] = b'x';
    }

    rolls.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA;

    #[test]
    fn test_remove_rolls() {
        let mut input = parse(TEST_DATA);

        let count = remove_rolls(&mut input);
        assert_eq!(count, 13);

        let count = remove_rolls(&mut input);
        assert_eq!(count, 12);

        let count = remove_rolls(&mut input);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_rolls() {
        let mut input = parse(TEST_DATA);

        let count = count_removable_rolls(&mut input);

        assert_eq!(count, 43);
    }
}