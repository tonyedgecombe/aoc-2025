use std::cmp::Ordering;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vertex {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vertex {
    pub fn distance(from: Vertex, to: Vertex) -> i64 {
        ((from.x - to.x).pow(2) + (from.y - to.y).pow(2) + (from.z - to.z).pow(2)).isqrt()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub distance: i64
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.distance.cmp(&other.distance) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                match self.to.cmp(&other.to) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => self.from.cmp(&other.from),
                    Ordering::Greater => Ordering::Greater,
                }
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

pub fn edges(vertices: &Vec<Vertex>) -> Vec<Edge> {
    (0..vertices.len())
        .flat_map(|i| {
            (i+1..vertices.len())
                .map(move |j| Edge { from: i, to: j, distance: Vertex::distance(vertices[i], vertices[j])})
        })
        .collect()
}

pub fn parse(input: &str) -> Vec<Vertex> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Vertex {
    let values: Vec<_> = line
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    Vertex {
        x: values[0],
        y: values[1],
        z: values[2],
    }
}

pub const TEST_DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(TEST_DATA);

        assert_eq!(result.len(), 20);

        assert_eq!(result[0], Vertex {
            x: 162,
            y: 817,
            z: 812,
        });

        assert_eq!(result[19], Vertex {
            x: 425,
            y: 690,
            z: 689,
        });
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line("162,817,812");

        assert_eq!(result, Vertex {
            x: 162,
            y: 817,
            z: 812,
        })
    }

    #[test]
    fn test_distances() {
        let data = parse(TEST_DATA);
        let distances = edges(&data);

        assert_eq!(distances.len(), 190);

        let first = &distances[0];
        let expected = Vertex::distance(data[first.from], data[first.to]);
        assert_eq!(first.distance, expected);
    }
}