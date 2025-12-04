
pub fn parse(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .split(|c| *c == b'\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.to_vec())
        .collect()
}

pub fn count_adjacent_rolls(input: &Vec<Vec<u8>>, row: i64, column: i64) -> usize {
    assert_eq!(input[row as usize][column as usize], b'@');

    let left = i64::max(0, row - 1);
    let right = i64::min(input.len() as i64 - 1, row + 1);
    let top = i64::max(0, column - 1);
    let bottom = i64::min(input[row as usize].len() as i64 - 1, column + 1);

    (left..=right)
        .flat_map(|r| (top..=bottom).map(move |c| (r,c)))
        .filter(|(r, c)| *r != row || *c != column)
        .filter(|(r, c)| input[*r as usize][*c as usize] == b'@')
        .count()
}

pub fn removable_rolls(input: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    (0..input.len())
        .flat_map(|r| (0..input[r].len()).map(move |c| (r, c)))
        .filter(|(r, c)| input[*r][*c] == b'@')
        .filter(|(r, c)| count_adjacent_rolls(input, *r as i64, *c as i64) < 4)
        .collect()
}



pub const TEST_DATA: &[u8] = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(TEST_DATA);
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], b"..@@.@@@@.");
    }

    #[test]
    fn test_count_adjacent_rolls() {
        let input = parse(TEST_DATA);

        assert_eq!(count_adjacent_rolls(&input, 0, 2), 3);
        assert_eq!(count_adjacent_rolls(&input, 0, 3), 3);
        assert_eq!(count_adjacent_rolls(&input, 1, 1), 6);
    }
}