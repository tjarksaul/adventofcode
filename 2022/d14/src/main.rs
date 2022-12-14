fn main() {
    let coordinates = read::parse_all_lines(include_str!("../input.txt"));

    let (mut dot_map, start_index) = generate_dot_map(&coordinates);

    let sand_unit_count = count_sand_units(&mut dot_map, start_index);

    dbg!(sand_unit_count);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dot {
    Rock,
    Sand,
    Air,
}

pub struct Coordinates(usize, usize);

fn count_sand_units(dots: &mut Vec<Vec<Dot>>, start_index: usize) -> usize {
    let mut sand_units = 0;

    loop {
        let mut position = (0, start_index);

        loop {
            let new_position = drop_sand_unit(dots, position.0, position.1);

            if new_position
                == (
                    position.0.try_into().unwrap(),
                    position.1.try_into().unwrap(),
                )
            {
                sand_units += 1;
                break;
            }
            if new_position.1 < 0 {
                // we've reached the abyss
                return sand_units;
            }
            let y_pos: usize = new_position.0.try_into().unwrap();
            let x_pos: usize = new_position.1.try_into().unwrap();
            if y_pos > dots.len() || x_pos > dots[0].len() {
                // we've reached the abyss
                return sand_units;
            }
            position = (y_pos, x_pos);
        }
    }
}

fn drop_sand_unit(dots: &mut Vec<Vec<Dot>>, y: usize, x: usize) -> (i32, i32) {
    let y_i: i32 = y.try_into().unwrap();
    let new_y = y_i + 1;
    let new_y_u = y + 1;
    if new_y_u >= dots.len() {
        return (new_y, x.try_into().unwrap());
    }
    for new_x in [x as i32, (x as i32 - 1), x as i32 + 1] {
        if new_x < 0 || new_x >= dots[new_y_u].len() as i32 {
            return (new_y, new_x);
        }

        if dots[new_y_u][new_x as usize] == Dot::Air {
            dots[new_y_u][new_x as usize] = Dot::Sand;
            dots[y][x] = Dot::Air;
            return (new_y, new_x);
        }
    }

    return (y.try_into().unwrap(), x.try_into().unwrap());
}

fn generate_dot_map(coordinates: &Vec<Vec<Coordinates>>) -> (Vec<Vec<Dot>>, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for coordinate_list in coordinates {
        for coordinate in coordinate_list {
            let Coordinates(x, y) = coordinate;
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
    }

    dbg!(max_x, max_y);

    let mut vec = vec![vec![Dot::Air; max_x - min_x + 1]; max_y + 1];

    dbg!(vec.len(), vec[0].len());

    for rock_list in coordinates {
        for i in 1..rock_list.len() {
            let Coordinates(x_start_d, mut y_start) = rock_list[i - 1];
            let mut x_start = x_start_d - min_x;
            let Coordinates(x_end_d, mut y_end) = rock_list[i];
            let mut x_end = x_end_d - min_x;

            if x_end != x_start {
                // we have a horizontal line
                if x_start > x_end {
                    (x_start, x_end) = (x_end, x_start)
                }
                for x in x_start..=x_end {
                    vec[y_start][x] = Dot::Rock;
                }
            } else {
                // we have a vertical line
                if y_start > y_end {
                    (y_start, y_end) = (y_end, y_start)
                }
                for y in y_start..=y_end {
                    vec[y][x_start] = Dot::Rock;
                }
            }
        }
    }

    return (vec, 500 - min_x);
}

#[allow(dead_code)]
fn print_rocks(rocks: &Vec<Vec<Dot>>) {
    for row in rocks {
        for column in row {
            if column == &Dot::Rock {
                print!("#");
            } else if column == &Dot::Sand {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn get_input() -> (Vec<Vec<Dot>>, usize) {
        let dots = vec![
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Rock,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
            ],
            vec![
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Air,
                Dot::Rock,
                Dot::Air,
            ],
            vec![
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Rock,
                Dot::Air,
            ],
        ];

        (dots, 6)
    }

    #[test]
    fn test_counts_sand_units_correctly() {
        let (mut dots, start_index) = get_input();

        let sand_units = count_sand_units(&mut dots, start_index);

        assert_eq!(sand_units, 24);
    }

    #[test]
    fn test_parses_correctly() {
        let test_input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
        .trim();

        let coordinates = read::parse_all_lines(&test_input);

        let (dot_map, parsed_start_index) = generate_dot_map(&coordinates);
        let (dots, start_index) = get_input();

        assert_eq!(parsed_start_index, start_index);
        assert_eq!(dot_map, dots);
    }
}

mod read {
    // Sample input:
    // 498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming,
        multi::separated_list1, sequence::tuple, Finish, IResult,
    };

    use super::Coordinates;

    fn parse_coordinates(i: &str) -> IResult<&str, Coordinates> {
        let (i, (x, _, y)) = tuple((cc::u32, tag(","), cc::u32))(i)?;
        Ok((i, Coordinates(x.try_into().unwrap(), y.try_into().unwrap())))
    }

    fn parse_line(i: &str) -> IResult<&str, Vec<Coordinates>> {
        separated_list1(tag(" -> "), parse_coordinates)(i)
    }

    pub fn parse_all_lines(i: &'static str) -> Vec<Vec<Coordinates>> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }
}
