fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let result = calculate_calibration_value(&input);
    let result2 = calculate_calibration_value_including_string(&input);

    dbg!(result, result2);
}

fn calculate_calibration_value(input: &Vec<Vec<char>>) -> i64 {
    input.iter().map(|it| calculate_line(it)).sum()
}

fn calculate_line(line: &Vec<char>) -> i64 {
    let first = line.iter().find(|it| it.is_numeric()).unwrap();
    let last = line.iter().rfind(|it| it.is_numeric()).unwrap();

    let value: String = vec![*first, *last].into_iter().collect();
    return value.parse::<>().unwrap();
}

fn calculate_calibration_value_including_string(input: &Vec<Vec<char>>) -> i64 {
    input.iter().map(|it| calculate_line_including_string(it)).sum()
}

fn calculate_line_including_string(line: &Vec<char>) -> i64 {
    let l: String = line.into_iter().collect();
    let numbers = vec![
        vec!["zero", "0"],
        vec!["one", "1"],
        vec!["two", "2"],
        vec!["three", "3"],
        vec!["four", "4"],
        vec!["five", "5"],
        vec!["six", "6"],
        vec!["seven", "7"],
        vec!["eight", "8"],
        vec!["nine", "9"],
    ];

    let mut first = 0;
    let mut first_index = l.len();
    let mut last = 0;
    let mut last_index = 0;

    for idx in 0..l.len() {
        for (i, n) in numbers.iter().enumerate() {
            for (_j, m) in n.iter().enumerate() {
                if let Some(result) = l[idx..].find(m).map(|i| i + idx) {
                    if result <= first_index {
                        first_index = result;
                        first = i;
                    }
                    if result >= last_index {
                        last_index = result;
                        last = i;
                    }
                }
            }
        }
    }

    return (first * 10 + last) as i64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculates_calibration_value_correctly() {
        let input = vec![
            vec!['1', 'a', 'b', 'c', '2', ],
            vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x'],
            vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f'],
            vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't', ],
        ];

        let calibration_value = super::calculate_calibration_value(&input);

        assert_eq!(calibration_value, 142);
    }

    #[test]
    fn calculates_calibration_value_with_string_correctly() {
        let input = vec![
            vec!['t', 'w', 'o', '1', 'n', 'i', 'n', 'e'],
            vec!['e', 'i', 'g', 'h', 't', 'w', 'o', 't', 'h', 'r', 'e', 'e'],
            vec!['a', 'b', 'c', 'o', 'n', 'e', '2', 't', 'h', 'r', 'e', 'e', 'x', 'y', 'z'],
            vec!['x', 't', 'w', 'o', 'n', 'e', '3', 'f', 'o', 'u', 'r'],
            vec!['4', 'n', 'i', 'n', 'e', 'e', 'i', 'g', 'h', 't', 's', 'e', 'v', 'e', 'n', '2'],
            vec!['z', 'o', 'n', 'e', 'i', 'g', 'h', 't', '2', '3', '4'],
            vec!['7', 'p', 'q', 'r', 's', 't', 's', 'i', 'x', 't', 'e', 'e', 'n'],
        ];

        let calibration_value = super::calculate_calibration_value_including_string(&input);

        assert_eq!(calibration_value, 281);
    }
}

mod read {
    pub fn read_all_lines(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.to_owned().chars().collect()).collect()
    }
}

