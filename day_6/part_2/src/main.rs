fn main() {
    let input = parse(include_bytes!("../../../data/day_6.txt"));

    let result = sum_results(input);

    println!("Day 6 Part 2 result: {result}");
}

pub fn sum_results(data: Vec<Vec<u8>>) -> i64 {
    columns(data).iter().sum()
}

pub fn columns(columns: Vec<Vec<u8>>) -> Vec<i64> {
    let mut result = vec![];
    let mut symbol = b' ';
    let mut numbers = vec![];

    for column in columns.iter().rev() {
        let mut v: i64 = 0;

        // Skip past empty column
        if column.iter().all(|c| *c == b' ') {
            continue;
        }

        for chr in column {
            match chr {
                b'0'..=b'9' => { v = v * 10 + (chr - b'0') as i64 },
                b'+' | b'*' => symbol = *chr,
                _ => {},
            }
        }

        numbers.push(v);

        match symbol {
            b'+' => {
                result.push(numbers.iter().sum());
                numbers.clear();
            },
            b'*' => {
                result.push(numbers.iter().product());
                numbers.clear();
            },
            _ => {},
        }

        symbol = b' ';
    }

    result
}

fn parse(input: &[u8]) -> Vec<Vec<u8>> {
    let lines: Vec<_> = input
        .split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .collect();

    let line_length = lines.iter().map(|line| line.len()).max().unwrap();
    assert!(lines.iter().all(|line| line.len() == line_length));

    (0..lines[0].len())
        .map(|i| {
            let col: Vec<u8> = lines.iter().map(|line| line[i]).collect();
            col
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    pub const TEST_DATA: &[u8] = b"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_parse() {
        let result = parse(TEST_DATA);
        assert_eq!(result.len(), 15);

        assert_eq!(result[0], b"1  *");
        assert_eq!(result[1], b"24  ");
        assert_eq!(result[2], b"356 ");
        assert_eq!(result[14], b"  4 ");
    }

    #[test]
    fn test_columns() {
        let data = parse(TEST_DATA);
        let result = columns(data);

        assert_eq!(result, [1058, 3253600, 625, 8544]);
    }

    #[test]
    fn test_sum_results() {
        let data = parse(TEST_DATA);
        let result = sum_results(data);
        assert_eq!(result, 3263827);
    }
}