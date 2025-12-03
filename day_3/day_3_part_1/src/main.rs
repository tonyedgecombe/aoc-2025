fn main() {
    let input = include_bytes!("../../../data/day_3.txt");

    let result = total_jotage(input);

    println!("Day 3 Part 1 result {result}");
}

fn total_jotage(input: &[u8]) -> u64 {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let line: Vec<_> = line.iter().map(|c| (*c - b'0') as u64).collect();

            line_jotage(&line)
        }).sum()
}

fn line_jotage(line: &[u64]) -> u64 {
    let mut max = 0;

    for i in 0..(line.len() - 1) {
        for j in i + 1..line.len() {
            let x = line[i] * 10 + line[j];
            if x > max {
                max = x;
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;


    #[test]
    fn test_line_jotage() {
        let result = line_jotage(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]);
        assert_eq!(result, 78);
    }

    #[test]
    fn test_total_jotage() {
        let result = total_jotage(TEST_DATA);
        assert_eq!(result, 357)
    }
}