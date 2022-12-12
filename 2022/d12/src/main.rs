use pathfinding::prelude::bfs;
use pathfinding::prelude::Matrix;

fn main() {
    let (matrix, start, target) = read::read_input(String::from("input.txt"));

    let shortest_path_length = find_shortest_path(&matrix, &start, &target);

    println!("Shortest path length: {shortest_path_length}");
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point(usize, usize);

impl Point {
    fn successors(&self, map: &Matrix<i32>) -> Vec<Point> {
        let &Point(x, y) = self;
        let mut neighbors: Vec<Point> = vec![];
        if y > 0 {
            // left neighbor
            neighbors.push(Point(x, y - 1));
        }
        if x > 0 {
            // top neighbor
            neighbors.push(Point(x - 1, y));
        }
        if y < map.columns - 1 {
            // right neighbor
            neighbors.push(Point(x, y + 1));
        }
        if x < map.rows - 1 {
            // bottom neighbor
            neighbors.push(Point(x + 1, y));
        }
        let current_height = map.get(self.to_tuple()).unwrap();
        let neighbors: Vec<Point> = neighbors
            .into_iter()
            .filter(|neighbor| {
                let neighbor_height = map.get(neighbor.to_tuple()).unwrap();
                let height_difference = neighbor_height - current_height;
                height_difference <= 1
            })
            .collect();
        return neighbors;
    }

    fn to_tuple(&self) -> (usize, usize) {
        let &Point(x, y) = self;
        return (x, y);
    }
}

fn find_shortest_path(map: &Matrix<i32>, start: &Point, target: &Point) -> usize {
    let path = bfs(start, |p| p.successors(map), |p| *p == *target);

    return path.expect("no path found").len() - 1;
}

#[allow(dead_code)]
fn print_matrix(matrix: &Matrix<i32>, start: &Point, target: &Point) {
    for x in 0..matrix.rows {
        for y in 0..matrix.columns {
            if &Point(x, y) == start {
                print!("S");
            } else if &Point(x, y) == target {
                print!("E");
            } else {
                print!(
                    "{}",
                    char::from_u32(*matrix.get((x, y)).unwrap() as u32 + 97).unwrap()
                );
            }
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_visit_matrix(matrix: &Matrix<bool>, source_matrix: &Matrix<i32>) {
    for x in 0..matrix.rows {
        for y in 0..matrix.columns {
            if *matrix.get((x, y)).unwrap() {
                print!("#");
            } else {
                print!(
                    "{}",
                    char::from_u32(*source_matrix.get((x, y)).unwrap() as u32 + 97).unwrap()
                );
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pathfinding::prelude::Matrix;

    fn get_input() -> (Matrix<i32>, Point, Point) {
        let start = Point(0, 0);
        let target = Point(2, 5);
        let map: Matrix<i32> = Matrix::from_vec(
            5,
            8,
            #[rustfmt::skip]
            vec![
                'a', 'a', 'b', 'q', 'p', 'o', 'n', 'm',
                'a', 'b', 'c', 'r', 'y', 'x', 'x', 'l',
                'a', 'c', 'c', 's', 'z', 'z', 'x', 'k',
                'a', 'c', 'c', 't', 'u', 'v', 'w', 'j',
                'a', 'b', 'd', 'e', 'f', 'g', 'h', 'i',
            ].iter().map(|chr| (*chr as i32) - 97).collect(),
        )
        .unwrap();
        println!("{:?}", map);
        return (map, start, target);
    }

    #[test]
    fn test_finds_path_correctly() {
        let (map, start, target) = get_input();

        let shortest_path_length = find_shortest_path(&map, &start, &target);

        assert_eq!(shortest_path_length, 31);
    }
}

mod read {
    use pathfinding::prelude::Matrix;
    use std::fs;

    use super::Point;

    pub fn read_input(fname: String) -> (Matrix<i32>, Point, Point) {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let lines: Vec<&str> = contents.lines().collect();
        let rows = lines.len();
        let columns = lines[0].len();

        let mut start = Point(0, 0);
        let mut target = Point(0, 0);
        let mut vec: Vec<i32> = vec![];

        for x in 0..lines.len() {
            let splits = lines[x].chars();
            let mut y = 0;
            for chr in splits {
                let val = if chr == 'S' {
                    start = Point(x, y);
                    0
                } else if chr == 'E' {
                    target = Point(x, y);
                    25
                } else {
                    (chr as i32) - 97
                };

                vec.push(val);
                y += 1;
            }
        }

        let matrix = Matrix::from_vec(rows, columns, vec).expect("Couldn't create matrix");

        return (matrix, start, target);
    }
}
