fn main() {
    let coordinates = read::parse_all_lines(include_str!("../input.txt"));

    let mut dot_map = generate_dot_map(&coordinates);
    let mut dot_map_2 = generate_dot_map(&coordinates);

    let sand_unit_count = count_sand_units(&mut dot_map);
    add_floor(&mut dot_map_2);
    let sand_unit_count_until_full = count_sand_units_until_full(&mut dot_map_2);

    dbg!(sand_unit_count);
    dbg!(sand_unit_count_until_full);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dot {
    Rock,
    Sand,
    Air,
}

pub struct Coordinates(usize, usize);

fn count_sand_units(dots: &mut Vec<Vec<Dot>>) -> usize {
    let mut sand_units = 0;

    loop {
        let mut position = (0, 500);

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

fn count_sand_units_until_full(dots: &mut Vec<Vec<Dot>>) -> usize {
    let mut sand_units = 0;

    loop {
        let mut position = (0, 500);

        loop {
            let new_position = drop_sand_unit_2(dots, position.0, position.1);

            if new_position == (0, 500) {
                return sand_units + 1;
            }

            if new_position
                == (
                    position.0.try_into().unwrap(),
                    position.1.try_into().unwrap(),
                )
            {
                sand_units += 1;
                break;
            }
            let y_pos: usize = new_position.0.try_into().unwrap();
            let x_pos: usize = new_position.1.try_into().unwrap();
            if y_pos >= dots.len() {
                // this should never happen
                panic!("this shouldn't happen");
            }
            // print_rocks(&dots, 488, 512);
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

fn drop_sand_unit_2(dots: &mut Vec<Vec<Dot>>, y: usize, x: usize) -> (i32, i32) {
    let y_i: i32 = y.try_into().unwrap();
    let new_y = y_i + 1;
    let new_y_u = y + 1;
    if new_y_u >= dots.len() - 1 {
        // panic!("This should never happen bc we stopped on the floor");
        return (y.try_into().unwrap(), x.try_into().unwrap());
    }
    for new_x in [x as i32, (x as i32 - 1), x as i32 + 1] {
        if new_x < 0 {
            // we underflow and just return
            return (new_y, new_x);
        }
        if new_x as usize >= dots[new_y_u].len() {
            // we overflow. So we have to add a new column
            for i in 0..dots.len() {
                dots[i].push(Dot::Air);
            }
            dots[y][x] = Dot::Air;
            dots[new_y_u][new_x as usize] = Dot::Sand;
            // we also need to add new floor here
            let max_y = dots.len() - 1;
            dots[max_y][new_x as usize] = Dot::Rock;
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

fn generate_dot_map(coordinates: &Vec<Vec<Coordinates>>) -> Vec<Vec<Dot>> {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for coordinate_list in coordinates {
        for coordinate in coordinate_list {
            let Coordinates(x, y) = coordinate;
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
    }

    let mut vec = vec![vec![Dot::Air; max_x + 1]; max_y + 1];

    for rock_list in coordinates {
        for i in 1..rock_list.len() {
            let Coordinates(mut x_start, mut y_start) = rock_list[i - 1];
            let Coordinates(mut x_end, mut y_end) = rock_list[i];

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

    vec
}

fn add_floor(map: &mut Vec<Vec<Dot>>) {
    map.push(vec![Dot::Air; map[0].len()]);
    map.push(vec![Dot::Rock; map[0].len()]);
}

#[allow(dead_code)]
fn print_rocks(rocks: &Vec<Vec<Dot>>, start_col: usize, end_col: usize) {
    assert!(end_col >= start_col);
    for i in [100, 10] {
        for col in start_col..=end_col {
            print!("{}", col / i % i);
        }
        println!("");
    }
    for col in start_col..=end_col {
        print!("{}", col % 10);
    }
    println!("");
    for y in 0..rocks.len() {
        for x in 0..rocks[y].len() {
            if x < start_col || x > end_col {
                continue;
            }

            let column = rocks[y][x];
            if column == Dot::Rock {
                print!("#");
            } else if column == Dot::Sand {
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

    fn get_input() -> &'static str {
        "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
        .trim()
    }

    fn get_dot_map() -> Vec<Vec<Dot>> {
        let test_input = get_input();

        let coordinates = read::parse_all_lines(&test_input);

        generate_dot_map(&coordinates)
    }

    #[test]
    fn test_counts_sand_units_correctly() {
        let mut dot_map = get_dot_map();

        let sand_units = count_sand_units(&mut dot_map);

        assert_eq!(sand_units, 24);
    }

    #[test]
    fn test_counts_sand_units_until_full_correctly() {
        let mut dot_map = get_dot_map();

        add_floor(&mut dot_map);

        let sand_units = count_sand_units_until_full(&mut dot_map);

        assert_eq!(sand_units, 93);
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
