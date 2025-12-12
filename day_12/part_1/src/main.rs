use common::{parse, Region};

fn main() {
    let result = solve_part_1(include_str!("../../../data/day_12.txt"));

    println!("Day 12 Part 1 result: {result}");
}

fn solve_part_1(input: &str) -> usize {
    let (shapes, regions) = parse(input);

    regions.iter().filter(|region| fits(region, &shapes)).count()
}

fn fits(region: &Region, _shape: &Vec<Vec<&str>>) -> bool {
    let max = (region.width / 3) * (region.height / 3);
    if region.counts.iter().sum::<i64>() > max {
        return false;
    }

    // TODO z3 again?

    true
}

#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(TEST_DATA);

        assert_eq!(result, 2);
    }
}