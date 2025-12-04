use common::{parse, removable_rolls};

fn main() {
    let input = parse(include_bytes!("../../../data/day_4.txt"));

    let result = count_removable_rolls(&input);

    println!("Day 4 Part 1 result: {result}");
}

fn count_removable_rolls(input: &Vec<Vec<u8>>) -> usize {
    removable_rolls(input).iter().count()
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA;

    #[test]
    fn test_count_rolls() {
        let input = parse(TEST_DATA);
        let result = count_removable_rolls(&input);

        assert_eq!(result, 13);
    }
}

