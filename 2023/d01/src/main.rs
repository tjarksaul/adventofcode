fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let result = calculate_calibration_value(&input);

    dbg!(result);
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

#[cfg(test)]
mod tests {
    fn get_test_input() -> Vec<Vec<char>> {
        return vec![
            vec!['1', 'a', 'b', 'c', '2', ],
            vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x'],
            vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f'],
            vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't', ],
        ];
    }

    #[test]
    fn calculates_calibration_value_correctly() {
        let calibration_value = super::calculate_calibration_value(&get_test_input());

        assert_eq!(calibration_value, 142);
    }
}

mod read {
    pub fn read_all_lines(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.to_owned().chars().collect()).collect()
    }
}

