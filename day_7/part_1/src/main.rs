use common::parse;

fn main() {
    let mut data = parse(include_bytes!("../../../data/day_7.txt"));

    split_all_rows(&mut data);
    let result = count(&data);

    println!("Day 7 Part 1 result: {result}");
}


fn split_all_rows(data: &mut Vec<Vec<u8>>) {
    for row in 0..data.len() - 1 {
        split_row(data, row);
    }
}

fn split_row(data: &mut Vec<Vec<u8>>, row: usize) {
    for column in 0..data[row].len() {
        if data[row][column] == b'S' || data[row][column] == b'|' {
            split_row_at_column(data, row + 1, column);
        }
    }
}

fn split_row_at_column(data: &mut Vec<Vec<u8>>, row: usize, column: usize) {
    match data[row][column] {
        b'^' => {
            if column > 0 {
                data[row][column - 1] = b'|';
            }

            if column < data[row].len() - 1 {
                data[row][column + 1] = b'|';
            }
        },
        b'.' => {
            data[row][column] = b'|';
        },
        b'|' => {},
        _ => panic!()
    }
}

fn count(data: & Vec<Vec<u8>>) -> usize {
    (1..data.len())
        .map(|row| count_row(data, row))
        .sum()
}

fn count_row(data: & Vec<Vec<u8>>, row: usize) -> usize {
    (0..data[row].len())
        .filter(|col| data[row][*col] == b'^')
        .filter(|col| data[row - 1][*col] == b'|')
        .count()
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_split_row() {
        let mut data = parse(TEST_DATA);

        split_row(&mut data, 0);
        assert_eq!(data[1], b".......|.......".to_vec());

        split_row(&mut data, 1);
        assert_eq!(data[2], b"......|^|......".to_vec());

        split_row(&mut data, 2);
        assert_eq!(data[3], b"......|.|......".to_vec());
    }

    #[test]
    fn test_split() {
        let mut data = parse(TEST_DATA);

        split_all_rows(&mut data);

        assert_eq!(data[15], b"|.|.|.|.|.|||.|".to_vec())
    }

    #[test]
    fn test_count() {
        let mut data = parse(TEST_DATA);

        split_all_rows(&mut data);
        let result = count(&data);

        assert_eq!(result, 21);
    }
}