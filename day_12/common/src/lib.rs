pub struct Region {
    pub width: i64,
    pub height: i64,
    pub counts: Vec<i64>,
}

pub fn parse(input: &str) -> (Vec<Vec<&str>>, Vec<Region>) {
    let mut shapes = vec![];
    let mut regions = vec![];

    let mut lines = input.split("\n");

    while let Some(line) = lines.next() {
        if line.contains(&":") && !line.contains(&"x") {
            let mut shape = vec![];

            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                shape.push(line);
            }

            shapes.push(shape);
        }

        if line.contains(&"x") {
            let halves: Vec<_> = line.split(": ").collect();
            let dimensions: Vec<i64> = halves[0]
                .split('x')
                .map(|v| v.parse::<i64>().unwrap())
                .collect();

            let counts = halves[1]
                .split(" ")
                .map(|v| {
                    v.parse::<i64>().unwrap()
                })
                .collect();

            let region = Region {
                width: dimensions[0],
                height: dimensions[1],
                counts,
            };

            regions.push(region)

        }
    }

    (shapes, regions)
}


pub const TEST_DATA: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_shapes() {
        let (shapes, _) = parse(TEST_DATA);

        assert_eq!(shapes.len(), 6);

        assert_eq!(shapes[0][0], "###");
        assert_eq!(shapes[0][1], "##.");
        assert_eq!(shapes[0][2], "##.");

        assert_eq!(shapes[5][0], "###");
        assert_eq!(shapes[5][1], ".#.");
        assert_eq!(shapes[5][2], "###");
    }

    #[test]
    fn test_parse_regions() {
        let (_, regions) = parse(TEST_DATA);

        assert_eq!(regions.len(), 3);
        
        assert_eq!(regions[0].width, 4);
        assert_eq!(regions[0].height, 4);
        assert_eq!(regions[0].counts, [0, 0, 0, 0, 2, 0]);
        
        assert_eq!(regions[2].width, 12);
        assert_eq!(regions[2].height, 5);
        assert_eq!(regions[2].counts, [1, 0, 1, 0, 3, 2]);
    }
}