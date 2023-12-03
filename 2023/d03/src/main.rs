fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let result = calculate_part_number_sum(&input);

    dbg!(result);
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
                    println!("{}", value);
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

#[cfg(test)]
mod tests {
    #[test]
    fn calculates_calibration_value_correctly() {
        let input = vec![
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
        ];

        let part_sum = super::calculate_part_number_sum(&input);

        assert_eq!(part_sum, 4361);
    }
}

mod read {
    pub fn read_all_lines(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.to_owned().chars().collect()).collect()
    }
}

