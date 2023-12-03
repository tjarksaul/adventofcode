use std::collections::HashSet;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let part_sum = calculate_part_number_sum(&input);
    let gear_ratio = calculate_gear_ratio(&input);

    dbg!(part_sum, gear_ratio);
}

fn calculate_part_number_sum(input: &Vec<Vec<char>>) -> i64 {
    let mut number: Vec<char> = vec![];
    let mut is_part = false;
    let mut result: i64 = 0;
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() {
                if number.len() == 0 {
                    is_part = false;
                }
                number.push(*c);
            } else if is_part {
                let value: String = number.into_iter().collect();
                if value.len() > 0 {
                    result += value.parse::<i64>().unwrap();
                }
                number = vec![];
                is_part = false;
            } else {
                number = vec![];
            }

            // Check all adjacents if it's a symbol -> it's a part
            let adjacents = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
            for (x, y) in adjacents {
                let x = i as i32 + x;
                let y = j as i32 + y;
                if x >= 0 && y >= 0 && x < height && y < width {
                    let val = input[x as usize][y as usize];
                    if !val.is_numeric() && val != '.' {
                        is_part = true;
                    }
                }
            }
        }
    }

    return result;
}

fn calculate_gear_ratio(input: &Vec<Vec<char>>) -> i64 {
    let mut gear_ratio = 0;
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut numbers = HashSet::new();

                // Check all adjacents if it's a number
                // if we find two number --> it's a gear!
                let adjacents = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
                for (x, y) in adjacents {
                    let x = i as i32 + x;
                    let y = j as i32 + y;
                    if x >= 0 && y >= 0 && x < height && y < width {
                        let val = input[x as usize][y as usize];
                        if val.is_numeric() {
                            let number = trace_number(&input, x as usize, y as usize);
                            numbers.insert(number);
                        }
                    }
                }

                if numbers.len() == 2 {
                    gear_ratio += numbers.iter().fold(1, |acc, e| acc * e);
                }
            }
        }
    }

    gear_ratio
}

fn trace_number(input: &Vec<Vec<char>>, y: usize, x: usize) -> i64 {
    let mut number: Vec<char> = vec![];
    let mut x = x;
    if x != 0 {
        while x > 0 {
            let xs = x - 1;
            let val = input[y][xs];
            if val.is_numeric() {
                x = xs;
            } else {
                break;
            }
        }
    }
    for xs in x..input[y].len() {
        if input[y][xs].is_numeric() {
            number.push(input[y][xs]);
        } else {
            break;
        }
    }

    let value: String = number.into_iter().collect();
    value.parse::<>().unwrap()
}

#[cfg(test)]
mod tests {
    fn get_input() -> Vec<Vec<char>> {
        vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ]
    }

    #[test]
    fn calculates_calibration_value_correctly() {
        let input = get_input();

        let part_sum = super::calculate_part_number_sum(&input);

        assert_eq!(part_sum, 4361);
    }

    #[test]
    fn calculates_gear_ratio_correctly() {
        let input = get_input();

        let gear_ratio = super::calculate_gear_ratio(&input);

        assert_eq!(gear_ratio, 467835);
    }

}

mod read {
    pub fn read_all_lines(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.to_owned().chars().collect()).collect()
    }
}

