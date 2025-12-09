pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let items: Vec<_> = line.split(",").map(|line| line.parse::<i64>().unwrap()).collect();
            Point { x: items[0], y: items[1]}
        })
        .collect()
}


pub fn area(c1: Point, c2: Point) -> i64 {
    ((c1.x - c2.x).abs() + 1) * ((c1.y - c2.y).abs() + 1)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Edge {
    pub from: Point,
    pub to: Point,
}

impl Edge {
    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
}

pub const TEST_DATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_parse() {
        let data = parse(TEST_DATA);

        assert_eq!(data.len(), 8);
        assert_eq!(data[0], Point {x:7, y:1});
        assert_eq!(data[7], Point {x:7, y:3});
    }
}