fn main() {
    let input = parse(include_str!("../../../data/day_6.txt"));

    let result = sum_results(input);

    println!("Day 6 Part 1 result: {result}");
}

fn sum_results(columns: Vec<(Vec<i64>, &str)>) -> i64 {
    evaluate_columns(columns)
        .iter()
        .sum()
}

fn evaluate_columns(columns: Vec<(Vec<i64>, &str)>) -> Vec<i64> {
    columns
        .iter()
        .map(|column| evaluate_column(&column.0, column.1))
        .collect()
}

fn evaluate_column(entry: &[i64], chr: &str) -> i64 {
    match chr {
        "+" => entry.iter().sum(),
        "*" => entry.iter().product(),
        _ => panic!()
    }
}

pub fn parse(input: &str) -> Vec<(Vec<i64>, &str)> {
    let lines: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let symbols: Vec<&str> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .collect();

    let lines: Vec<_> = lines
        .iter()
        .filter(|line| !(line.contains("*") || line.contains("+")))
        .map(|line| line
            .split_whitespace()
            .map(|entry| entry.parse::<i64>().unwrap())
            .collect::<Vec<i64>>())
        .collect();

    assert_eq!(symbols.len(), lines[0].len());

    let result: Vec<(Vec<i64>, &str)> = (0..symbols.len())
        .map(|i| {
            let lines: Vec<i64> = lines.iter().map(|line| line[i]).collect();
            let symbol = symbols[i];
            (lines, symbol)
        })
        .collect();

    result
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_evaluate_column() {
        let data = parse(TEST_DATA);

        let result = evaluate_column(&data[0].0, data[0].1);
        assert_eq!(result, 33210);
    }

    #[test]
    fn test_evaluate_columns() {
        let data = parse(TEST_DATA);

        let result = evaluate_columns(data);
        assert_eq!(result, [33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_sum_results() {
        let data = parse(TEST_DATA);

        let result = sum_results(data);

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_parse() {
        let entries = parse(TEST_DATA);

        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0], ([123, 45, 6].to_vec(), "*"));
        assert_eq!(entries[1], ([328, 64, 98].to_vec(), "+"));
        assert_eq!(entries[2], ([51, 387, 215].to_vec(), "*"));
        assert_eq!(entries[3], ([64, 23, 314].to_vec(), "+"));
    }
}