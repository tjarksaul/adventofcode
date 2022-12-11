fn main() {
    let operations = read::read_input(String::from("input.txt"));

    let sum_signal_strength =
        get_signal_strength_sum(&operations, &vec![20, 60, 100, 140, 180, 220]);

    let crt_output = draw_output(&operations);

    println!("Sum of signal strengths: {sum_signal_strength}");
    println!("CRT Output:\n{crt_output}");
}

fn get_signal_strength_sum(input: &Vec<Operation>, cycles: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    #[allow(non_snake_case)]
    let mut X: i32 = 1;
    let mut remaining_cycles = 0;
    let mut current_operation: &Operation = &Operation::NoOp;
    let mut operations = input.into_iter();

    for cycle in 0..=cycles[cycles.len() - 1] {
        if (cycle % 20) == 0 || cycle == 1 {
            println!("Cycle: {}, X: {X}", cycle);
        }
        if cycles.iter().any(|&x| x == cycle) {
            sum += cycle * X;
        }

        if remaining_cycles == 0 {
            // execute current operation
            if let &Operation::AddX(add_value) = current_operation {
                X += add_value
            }

            // get new operation
            let next_operation = operations.next();
            if next_operation.is_none() {
                break;
            }
            current_operation = next_operation.unwrap();
            remaining_cycles = if &Operation::NoOp == current_operation {
                1
            } else {
                2
            }
        }
        remaining_cycles -= 1;
    }

    return sum;
}

fn draw_output(input: &Vec<Operation>) -> String {
    let mut output: String = "".to_owned();
    #[allow(non_snake_case)]
    let mut X: i32 = 1;
    let mut remaining_cycles = 0;
    let mut current_operation: &Operation = &Operation::NoOp;
    let mut operations = input.into_iter();

    for cycle in 0..241 {
        // we don't draw in cycle 0 bc it's just for setup
        if cycle > 0 {
            let modulo = cycle % 40;
            let position = if modulo == 0 { 39 } else { modulo - 1 };
            if position == X || position == X - 1 || position == X + 1 {
                output.push_str(&"#");
            } else {
                output.push_str(&".");
            }

            if position == 39 {
                output.push_str(&"\n");
            }
        }

        if remaining_cycles == 0 {
            // execute current operation
            if let &Operation::AddX(add_value) = current_operation {
                X += add_value
            }

            // get new operation
            let next_operation = operations.next();
            if next_operation.is_none() {
                break;
            }
            current_operation = next_operation.unwrap();
            remaining_cycles = if &Operation::NoOp == current_operation {
                1
            } else {
                2
            }
        }
        remaining_cycles -= 1;
    }

    return output;
}

#[derive(PartialEq)]
pub enum Operation {
    NoOp,
    AddX(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Operation> {
        return vec![
            Operation::AddX(15),
            Operation::AddX(-11),
            Operation::AddX(6),
            Operation::AddX(-3),
            Operation::AddX(5),
            Operation::AddX(-1),
            Operation::AddX(-8),
            Operation::AddX(13),
            Operation::AddX(4),
            Operation::NoOp,
            Operation::AddX(-1),
            Operation::AddX(5),
            Operation::AddX(-1),
            Operation::AddX(5),
            Operation::AddX(-1),
            Operation::AddX(5),
            Operation::AddX(-1),
            Operation::AddX(5),
            Operation::AddX(-1),
            Operation::AddX(-35),
            Operation::AddX(1),
            Operation::AddX(24),
            Operation::AddX(-19),
            Operation::AddX(1),
            Operation::AddX(16),
            Operation::AddX(-11),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(21),
            Operation::AddX(-15),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(-3),
            Operation::AddX(9),
            Operation::AddX(1),
            Operation::AddX(-3),
            Operation::AddX(8),
            Operation::AddX(1),
            Operation::AddX(5),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(-36),
            Operation::NoOp,
            Operation::AddX(1),
            Operation::AddX(7),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(2),
            Operation::AddX(6),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(1),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(7),
            Operation::AddX(1),
            Operation::NoOp,
            Operation::AddX(-13),
            Operation::AddX(13),
            Operation::AddX(7),
            Operation::NoOp,
            Operation::AddX(1),
            Operation::AddX(-33),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(2),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(8),
            Operation::NoOp,
            Operation::AddX(-1),
            Operation::AddX(2),
            Operation::AddX(1),
            Operation::NoOp,
            Operation::AddX(17),
            Operation::AddX(-9),
            Operation::AddX(1),
            Operation::AddX(1),
            Operation::AddX(-3),
            Operation::AddX(11),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(1),
            Operation::NoOp,
            Operation::AddX(1),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(-13),
            Operation::AddX(-19),
            Operation::AddX(1),
            Operation::AddX(3),
            Operation::AddX(26),
            Operation::AddX(-30),
            Operation::AddX(12),
            Operation::AddX(-1),
            Operation::AddX(3),
            Operation::AddX(1),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(-9),
            Operation::AddX(18),
            Operation::AddX(1),
            Operation::AddX(2),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(9),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(-1),
            Operation::AddX(2),
            Operation::AddX(-37),
            Operation::AddX(1),
            Operation::AddX(3),
            Operation::NoOp,
            Operation::AddX(15),
            Operation::AddX(-21),
            Operation::AddX(22),
            Operation::AddX(-6),
            Operation::AddX(1),
            Operation::NoOp,
            Operation::AddX(2),
            Operation::AddX(1),
            Operation::NoOp,
            Operation::AddX(-10),
            Operation::NoOp,
            Operation::NoOp,
            Operation::AddX(20),
            Operation::AddX(1),
            Operation::AddX(2),
            Operation::AddX(2),
            Operation::AddX(-6),
            Operation::AddX(-11),
            Operation::NoOp,
            Operation::NoOp,
            Operation::NoOp,
        ];
    }

    #[test]
    fn test_sums_operations_correctly() {
        let operations = get_input();

        let sum_signal_strength =
            get_signal_strength_sum(&operations, &vec![20, 60, 100, 140, 180, 220]);

        assert_eq!(sum_signal_strength, 13140);
    }

    #[test]
    fn test_draws_letters_correctly() {
        let operations = get_input();

        let output = draw_output(&operations);

        let expected_output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(output, expected_output);
    }
}

mod read {
    use std::fs;

    use super::Operation;

    pub fn read_input(fname: String) -> Vec<Operation> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let lines: Vec<&str> = contents.lines().collect();

        return lines
            .iter()
            .map(|line| {
                let mut splits = line.split(' ');
                let operation = splits.next().unwrap();

                if operation == "noop" {
                    return Operation::NoOp;
                } else {
                    let value: i32 = splits.next().unwrap().parse().unwrap();

                    return Operation::AddX(value);
                }
            })
            .collect();
    }
}
