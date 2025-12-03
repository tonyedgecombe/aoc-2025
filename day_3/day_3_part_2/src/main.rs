fn main() {
    let input = include_bytes!("../../../data/day_3.txt");

    let result = total_jotage_part_2(input);

    println!("Day 3 Part 2 result {result}");
}

fn total_jotage_part_2(input: &[u8]) -> u64 {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let line: Vec<_> = line.iter().map(|c| (*c - b'0') as u64).collect();
            line_jotage_part_2(line)
        })
        .sum()
}

fn line_jotage_part_2(mut line: Vec<u64>) -> u64 {
    while line.len() > 12 {
        remove_digit(&mut line);
    }

    line.into_iter().reduce(|acc, e| acc * 10 + e).unwrap()
}

fn remove_digit(line: &mut Vec<u64>) {
    for i in 0..line.len() - 1 {
        if line[i] < line[i + 1] {
            line.remove(i);
            return;
        }
    }

    line.remove(line.len() - 1);
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_line_jotage_part_2() {
        let result = line_jotage_part_2([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8].to_vec());
        assert_eq!(result, 434234234278);
    }

    #[test]
    fn test_total_jotage_part_2() {
        let result = total_jotage_part_2(TEST_DATA);
        assert_eq!(result, 3121910778619);
    }
}