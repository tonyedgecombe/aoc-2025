use common::{area, parse, Edge, Point};

fn main() {
    let result = solve_part_2(include_str!("../../../data/day_9.txt"));

    println!("Day 9 Part 2 result: {result}");
}


fn solve_part_2(input: &str) -> i64 {
    let co_ords = parse(input);

    co_ords
        .iter()
        .enumerate()
        .flat_map(|(i, c1)| (i+1..co_ords.len()).map(|j| (*c1, co_ords[j])))
        .filter(|rect| rect_is_green(&co_ords, *rect))
        .map(|rect| area(rect.0, rect.1))
        .max()
        .unwrap()
}

fn rect_is_green(data: &Vec<Point>, rect: (Point, Point)) -> bool {
    let left = i64::min(rect.0.x, rect.1.x);
    let right = i64::max(rect.0.x, rect.1.x);

    let top = i64::max(rect.0.y, rect.1.y);
    let bottom = i64::min(rect.0.y, rect.1.y);

    // Check all corners are inside polygon
    if !(even_odd_rule(data, Point { x: left, y: top })
        && even_odd_rule(data, Point { x: left, y: bottom })
        && even_odd_rule(data, Point { x: right, y: top })
        && even_odd_rule(data, Point { x: right, y: bottom })) {
            return false;
    }

    // Check if any rectangle edges cross polygon edges
    for i in 0..data.len() {
        let edge = Edge { from: data[i], to: data[(i + 1).rem_euclid(data.len())] };

        if edge.is_vertical() {
            if cross(&edge, &Edge { from: Point { x: left, y: top }, to: Point { x: right, y: top } }) ||
                cross(&edge, &Edge { from: Point { x: left, y: bottom }, to: Point { x: right, y: bottom } }) {
                return false;
            }
        } else {
            if cross(&Edge { from: Point { x: left, y: bottom }, to: Point { x: left, y: top } }, &edge) ||
                cross(&Edge { from: Point { x: right, y: bottom }, to: Point { x: right, y: top } }, &edge) {
                return false;
            }
        }
    }

    // Check at least one point from inside the rectangle is inside the polygon
    let a_point = Point { x: left + 1, y: top - 1 };
    even_odd_rule(&data, a_point)
}

fn cross(vertical: &Edge, horizontal: &Edge) -> bool {
    // Vertical should run from bottom to top
    let vert = if vertical.from.y > vertical.to.y { Edge { from: vertical.to, to: vertical.from } } else { *vertical };

    // Horizontal should run from left to right
    let horizontal = if horizontal.from.y > horizontal.to.y { Edge { from: horizontal.to, to: horizontal.from } } else { *horizontal };

    vert.from.x > horizontal.from.x && vert.from.x < horizontal.to.x &&
    vert.from.y < horizontal.from.y && vert.to.y > horizontal.from.y
}


fn even_odd_rule(data: &Vec<Point>, point: Point) -> bool {
    let mut even_odd = false;
    let Point {x, y} = point;

    for i in 0..data.len() {
        let a = data[i];
        let b = data[(i + 1).rem_euclid(data.len())];
        if (x == a.x) && (y == a.y) {
            return true;
        }

        if (a.y > y) != (b.y > y) {
            let slope = (x - a.x) * (b.y - a.y) - (b.x - a.x) * (y - a.y);
            if slope == 0 {
                return true;
            }

            if (slope < 0) != (b.y < a.y) {
                even_odd = !even_odd
            }
        }
    }

    even_odd
}


#[cfg(test)]
mod test {
    use common::TEST_DATA;
    use super::*;

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(TEST_DATA);

        assert_eq!(result, 24);
    }

    #[test]
    fn test_even_odd_rule() {
        let data = parse(TEST_DATA);

        assert!(!even_odd_rule(&data, Point {x:0, y:0}));

        assert!(even_odd_rule(&data, Point {x:7, y:1})); // On a corner

        assert!(even_odd_rule(&data, Point {x:9, y:2})); // Inside
    }

    #[test]
    fn test_rect_is_green() {
        let data = parse(TEST_DATA);

        assert!(rect_is_green(&data, (Point {x: 7,y: 3}, Point {x: 11, y:1})));

        assert!(!rect_is_green(&data, (Point {x:2, y:3}, Point {x:9, y:7})));

        assert!(!rect_is_green(&data, (Point {x:2, y:5}, Point {x:11, y:7})));
    }

    #[test]
    fn test_cross() {
        assert!(cross(&Edge { from: Point { x: 3, y: 0 }, to: Point { x: 3, y: 6 } }, &Edge { from: Point { x: 0, y: 3 }, to: Point { x: 6, y: 3 } }));

        assert!(!cross(&Edge { from: Point { x: 3, y: 0 }, to: Point { x: 3, y: 6 } }, &Edge { from: Point { x: 5, y: 3 }, to: Point { x: 6, y: 3 } }));
    }
}