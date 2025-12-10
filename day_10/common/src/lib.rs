pub fn parse(input: &[u8]) -> (Vec<Vec<i64>>, Vec<Vec<Vec<i64>>>, Vec<Vec<i64>>) {
    let mut indicator_sets = vec![];
    let mut button_sets = vec![];
    let mut joltage_sets = vec![];

    let lines: Vec<_> = input
        .split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .collect();

    for line in lines {
        let start = line.iter().position(|c| *c == b'[').unwrap() + 1;
        let end = line.iter().position(|c| *c == b']').unwrap();
        let indicator = line[start..end]
            .iter()
            .map(|c| if *c == b'#' {1} else {0})
            .collect();
        indicator_sets.push(indicator);

        let start = line.iter().position(|c| *c == b'(').unwrap();
        let end = line.iter().rposition(|c| *c == b')').unwrap();
        let button: Vec<_> = line[start..=end]
            .split(|c| *c == b' ')
            .map(|set| set[1..set.len() - 1]
                .split(|c| *c == b',')
                .map(|val| val.iter().fold(0i64, |acc, e| acc * 10 + ((*e - b'0') as i64)))
                .collect::<Vec<i64>>())
            .collect();
        button_sets.push(button);

        let start = line.iter().position(|c| *c == b'{').unwrap() + 1;
        let end = line.iter().rposition(|c| *c == b'}').unwrap();
        let joltage: Vec<_> = line[start..end]
            .split(|c| *c == b',')
            .map(|val| val.iter().fold(0i64, |acc, e| acc * 10 + ((*e - b'0') as i64)))
            .collect::<Vec<i64>>();

        joltage_sets.push(joltage);
    }

    (indicator_sets, button_sets, joltage_sets)
}

pub const TEST_DATA: &[u8] = b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_indicators() {
        let (indicator_sets, _, _) = parse(TEST_DATA);

        assert_eq!(indicator_sets.len(), 3);

        assert_eq!(indicator_sets[0], [0, 1, 1, 0]);
        assert_eq!(indicator_sets[1], [0, 0, 0, 1, 0]);
        assert_eq!(indicator_sets[2], [0, 1, 1, 1, 0, 1]);
    }

    #[test]
    fn test_parse_buttons() {
        let (_, button_sets, _) = parse(TEST_DATA);

        assert_eq!(button_sets.len(), 3);
        assert_eq!(button_sets[0], [[3].to_vec(), [1, 3].to_vec(), [2].to_vec(), [2, 3].to_vec(), [0, 2].to_vec(), [0, 1].to_vec()]);
    }
    #[test]
    fn test_parse_joltages() {
        let (_, _, joltage_sets) = parse(TEST_DATA);

        assert_eq!(joltage_sets.len(), 3);
        assert_eq!(joltage_sets[0], [3,5,4,7].to_vec());
        assert_eq!(joltage_sets[1], [7,5,12,7,2].to_vec());
        assert_eq!(joltage_sets[2], [10,11,11,5,10,5].to_vec());
    }
}