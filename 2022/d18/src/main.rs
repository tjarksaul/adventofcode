use std::collections::HashSet;
use std::fmt;

fn main() {
    let cubes = read::parse_all_lines(include_str!("../input.txt"));

    let surface_area = calculate_surface_area(&cubes);

    dbg!(surface_area);
}

fn calculate_surface_area(cubes: &HashSet<Cube>) -> u64 {
    let mut touching_area = 0;

    let cube_vec: Vec<&Cube> = cubes.iter().collect();

    for i in 0..cube_vec.len() {
        let cube = cube_vec[i];

        touching_area += cube_vec
            .iter()
            .filter(|&&a| a != cube)
            .fold(0, |a, current_cube| {
                a + (current_cube.touches(&cube) as u64)
            });
    }

    (cubes.len() as u64) * 6 - touching_area
}

#[derive(
    PartialEq,
    Eq,
    Debug,
    Clone,
    Copy,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
)]
pub struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Cube {
    origin: Point,
}

impl Cube {
    fn points(&self) -> HashSet<Point> {
        HashSet::from([
            self.origin,
            self.origin + Point { x: 1, y: 0, z: 0 },
            self.origin + Point { x: 1, y: 0, z: 1 },
            self.origin + Point { x: 1, y: 1, z: 1 },
            self.origin + Point { x: 1, y: 1, z: 0 },
            self.origin + Point { x: 0, y: 0, z: 1 },
            self.origin + Point { x: 0, y: 1, z: 0 },
            self.origin + Point { x: 0, y: 1, z: 1 },
        ])
    }

    fn touches(&self, other: &Cube) -> bool {
        let self_points = self.points();
        let other_points = other.points();
        let intersection = self_points
            .intersection(&other_points)
            .collect::<HashSet<_>>();

        intersection.len() == 4
    }

    fn parse<'a>(input: &'a str) -> Self {
        let mut splits = input.split(',');

        let x: u64 = splits.next().unwrap().parse().unwrap();
        let y: u64 = splits.next().unwrap().parse().unwrap();
        let z: u64 = splits.next().unwrap().parse().unwrap();

        Self {
            origin: Point { x, y, z },
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.origin)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_points() -> HashSet<Cube> {
        HashSet::from([
            Cube {
                origin: Point { x: 2, y: 2, z: 2 },
            },
            Cube {
                origin: Point { x: 1, y: 2, z: 2 },
            },
            Cube {
                origin: Point { x: 3, y: 2, z: 2 },
            },
            Cube {
                origin: Point { x: 2, y: 1, z: 2 },
            },
            Cube {
                origin: Point { x: 2, y: 3, z: 2 },
            },
            Cube {
                origin: Point { x: 2, y: 2, z: 1 },
            },
            Cube {
                origin: Point { x: 2, y: 2, z: 3 },
            },
            Cube {
                origin: Point { x: 2, y: 2, z: 4 },
            },
            Cube {
                origin: Point { x: 2, y: 2, z: 6 },
            },
            Cube {
                origin: Point { x: 1, y: 2, z: 5 },
            },
            Cube {
                origin: Point { x: 3, y: 2, z: 5 },
            },
            Cube {
                origin: Point { x: 2, y: 1, z: 5 },
            },
            Cube {
                origin: Point { x: 2, y: 3, z: 5 },
            },
        ])
    }

    #[test]
    fn test_calculates_surface_area_correctly() {
        let points = get_points();

        let surface_area = calculate_surface_area(&points);

        assert_eq!(surface_area, 64);
    }

    #[test]

    fn test_parser() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        let expected_points = get_points();

        let parsed = read::parse_all_lines(&input);

        assert_eq!(parsed, expected_points);
    }
}

mod read {
    use super::Cube;
    use std::collections::HashSet;

    pub fn parse_all_lines<'a>(i: &'a str) -> HashSet<Cube> {
        i.lines().map(|l| Cube::parse(l)).collect()
    }
}
