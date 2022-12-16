use std::collections::HashSet;

fn main() {
    let input = read::parse_all_lines(include_str!("../input.txt"));

    let impossible_positions = find_amount_impossible_positions_by_row(&input, 2000000);
    let tuning_frequency = find_empty_position_tuning_frequency(&input, 4000000);

    dbg!(impossible_positions);
    dbg!(tuning_frequency);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Position(i64, i64);

impl Position {
    fn distance(&self, other: &Position) -> u64 {
        ((other.1 - self.1).abs() + (other.0 - self.0).abs()) as u64
    }
}

fn get_x_extremes(nodes: &Vec<(Position, Position)>) -> (i64, i64) {
    let mut x_s = vec![];
    for (node, beacon) in nodes {
        x_s.push(node.0);
        x_s.push(beacon.0);
    }
    let max_x = *x_s.iter().max().unwrap();
    let min_x = *x_s.iter().min().unwrap();

    (min_x, max_x)
}

fn get_node_distances(nodes: &Vec<(Position, Position)>) -> (Vec<(Position, u64)>, u64) {
    let mut max_distance = 0;
    let node_distances: Vec<_> = nodes
        .iter()
        .map(|nodes| {
            let (node, beacon) = nodes;
            let distance = node.distance(beacon);
            if distance > max_distance {
                max_distance = distance;
            }
            (*node, distance)
        })
        .collect();
    (node_distances, max_distance)
}

fn find_amount_impossible_positions_by_row(nodes: &Vec<(Position, Position)>, row: i64) -> u64 {
    let (min_x, max_x) = get_x_extremes(nodes);

    let mut nodes_at_row = HashSet::new();
    for (node, beacon) in nodes {
        if node.1 == row {
            nodes_at_row.insert(node);
        }
        if beacon.1 == row {
            nodes_at_row.insert(beacon);
        }
    }

    let (node_distances, max_distance) = get_node_distances(nodes);
    let dx = max_distance as i64;

    let mut impossible_positions = 0;
    for x in (min_x - dx)..=(max_x + dx) {
        for (node, distance) in &node_distances {
            let current_distance = node.distance(&Position(x, row));
            if current_distance <= *distance {
                impossible_positions += 1;
                break;
            }
        }
    }

    impossible_positions - (nodes_at_row.len() as u64)
}

fn is_valid_beacon_position(node: Position, node_distances: &Vec<(Position, u64)>) -> bool {
    for (other, distance) in node_distances {
        if node.distance(&other) <= *distance {
            return false;
        }
    }
    return true;
}

fn find_empty_position_tuning_frequency(nodes: &Vec<(Position, Position)>, max_coord: i64) -> u64 {
    let (node_distances, _) = get_node_distances(nodes);

    for (node, distance) in &node_distances {
        for dx in 0..distance + 2 {
            let dy = (distance + 1) - dx;
            let dx = dx as i64;
            let dy = dy as i64;
            // all four directions
            for (sx, sy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                let x = node.0 + (dx * sx);
                let y = node.1 + (dy * sy);
                if x < 0 || x > max_coord || y < 0 || y > max_coord {
                    continue;
                }

                if is_valid_beacon_position(Position(x, y), &node_distances) {
                    return (x * 4000000 + y) as u64;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<(Position, Position)> {
        vec![
            (Position(2, 18), Position(-2, 15)),
            (Position(9, 16), Position(10, 16)),
            (Position(13, 2), Position(15, 3)),
            (Position(12, 14), Position(10, 16)),
            (Position(10, 20), Position(10, 16)),
            (Position(14, 17), Position(10, 16)),
            (Position(8, 7), Position(2, 10)),
            (Position(2, 0), Position(2, 10)),
            (Position(0, 11), Position(2, 10)),
            (Position(20, 14), Position(25, 17)),
            (Position(17, 20), Position(21, 22)),
            (Position(16, 7), Position(15, 3)),
            (Position(14, 3), Position(15, 3)),
            (Position(20, 1), Position(15, 3)),
        ]
    }

    #[test]
    fn test_find_correct_amount_of_impossible_positions() {
        let input: Vec<(Position, Position)> = get_input();

        let impossible_positions = find_amount_impossible_positions_by_row(&input, 10);

        assert_eq!(impossible_positions, 26);
    }

    #[test]
    fn test_parser() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let expected_output = get_input();

        let parsed = read::parse_all_lines(input);

        assert_eq!(parsed, expected_output);
    }

    #[test]
    fn test_find_empty_position_tuning_frequency_correctly() {
        let input: Vec<(Position, Position)> = get_input();

        let tuning_frequency = find_empty_position_tuning_frequency(&input, 20);

        assert_eq!(tuning_frequency, 56000011);
    }
}

mod read {
    // Sample input:
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming,
        sequence::tuple, Finish, IResult,
    };

    use super::Position;

    fn parse_position(i: &str) -> IResult<&str, Position> {
        let (i, (_, x, _, y)) = tuple((tag("x="), cc::i64, tag(", y="), cc::i64))(i)?;
        Ok((i, Position(x, y)))
    }

    fn parse_line(i: &str) -> IResult<&str, (Position, Position)> {
        let (i, (_, pos1, _, pos2)) = tuple((
            tag("Sensor at "),
            parse_position,
            tag(": closest beacon is at "),
            parse_position,
        ))(i)?;
        Ok((i, (pos1, pos2)))
    }

    pub fn parse_all_lines(i: &'static str) -> Vec<(Position, Position)> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }
}
