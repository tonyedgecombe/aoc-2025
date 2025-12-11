use std::collections::HashMap;
use common::parse;

fn main() {
    let result = solve_part_2(include_str!("../../../data/day_11.txt"));

    println!("Day 11 Part 2: {result}");
}

fn solve_part_2(input: &str) -> usize{
    let data = parse(input);
    let mut cache = HashMap::new();

    count_paths_2(&data, "svr", false, false, &mut cache)
}

type Cache<'a> = HashMap<(&'a str, bool, bool), usize>;

fn count_paths_2<'a>(data: &'a HashMap<&str, Vec<&str>>, label : &'a str, dac: bool, fft: bool, cache: &mut Cache<'a>) -> usize {
    if let Some(result) = cache.get(&(label, dac, fft)) {
        return *result
    }

    if label == "out" {
        if dac && fft {
            1
        } else {
            0
        }
    } else {
        data
            .get(label)
            .unwrap()
            .iter()
            .map(|entry| {
                let dac = dac || label == "dac";
                let fft = fft || label == "fft";

                let result = count_paths_2(data, entry, dac, fft, cache);
                cache.insert((entry, dac, fft), result);
                result
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TEST_DATA_2;

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(TEST_DATA_2);

        assert_eq!(result, 2);
    }
}