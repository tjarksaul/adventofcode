use std::fmt;

fn main() {
    let (mut monkeys, lcm) = read::read_input(String::from("input.txt"));

    let monkey_business = calculate_monkey_business_part_2(&mut monkeys, lcm);

    println!("Monkey business is {monkey_business}");
}

#[derive(Debug)]
pub enum Operation {
    Multiply(usize),
    SelfMultiply,
    Add(usize),
}

pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    true_target: usize,
    false_target: usize,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey: {:?}, operation: {:#?}, test: {} if true go to monkey {} else {}",
            self.items, self.operation, self.test, self.true_target, self.false_target
        )
    }
}

fn calculate_monkey_business_part_1(monkeys: &mut Vec<Monkey>) -> u128 {
    calculate_monkey_business(monkeys, 20, |worry| worry / 3)
}

fn calculate_monkey_business_part_2(monkeys: &mut Vec<Monkey>, lcm: usize) -> u128 {
    calculate_monkey_business(monkeys, 10000, |worry| worry % lcm)
}

fn calculate_monkey_business(
    input: &mut Vec<Monkey>,
    rounds: usize,
    reduce_worry: impl Fn(usize) -> usize,
) -> u128 {
    let mut inspections: Vec<u128> = vec![0; input.len()];
    for _ in 0..rounds {
        for i in 0..input.len() {
            let mut moves: Vec<Vec<usize>> = vec![vec![]; input.len()];
            for item in &input[i].items {
                inspections[i] += 1;
                let new_worry = reduce_worry(perform_operation(item.clone(), &input[i].operation));

                let target = if new_worry % input[i].test == 0 {
                    input[i].true_target
                } else {
                    input[i].false_target
                };
                moves[target].push(new_worry);
            }

            for target in 0..moves.len() {
                for item in &moves[target] {
                    input[target].items.push(*item);
                }
            }
            input[i].items = vec![];
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    let top_two = inspections[0..2].to_vec();

    return top_two.iter().fold(1, |a, b| a * *b);
}

fn perform_operation(input: usize, operation: &Operation) -> usize {
    return match *operation {
        Operation::SelfMultiply => input * input,
        Operation::Multiply(factor) => input * factor,
        Operation::Add(summand) => input + summand,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Monkey> {
        return vec![
            Monkey {
                items: vec![79, 98],
                operation: Operation::Multiply(19),
                test: 23,
                true_target: 2,
                false_target: 3,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation::Add(6),
                test: 19,
                true_target: 2,
                false_target: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Operation::SelfMultiply,
                test: 13,
                true_target: 1,
                false_target: 3,
            },
            Monkey {
                items: vec![74],
                operation: Operation::Add(3),
                test: 17,
                true_target: 0,
                false_target: 1,
            },
        ];
    }

    #[test]
    fn test_calculates_monkey_business_correctly() {
        let mut monkeys = get_input();

        let monkey_business = calculate_monkey_business_part_1(&mut monkeys);

        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn test_calculates_monkey_business_correctly_part_2() {
        let mut monkeys = get_input();

        let monkey_business = calculate_monkey_business_part_2(&mut monkeys, 23 * 19 * 13 * 17);

        assert_eq!(monkey_business, 2713310158);
    }
}

mod read {
    use regex::Regex;
    use std::fs;

    use super::Monkey;
    use super::Operation;

    pub fn read_input(fname: String) -> (Vec<Monkey>, usize) {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let multiply_regex = Regex::new(r"new = old \* ([0-9]+)").unwrap();
        let add_regex = Regex::new(r"new = old \+ ([0-9]+)").unwrap();

        let splits: Vec<&str> = contents.split("\n\n").collect();

        let mut lcm: usize = 1;
        let monkeys: Vec<Monkey> = splits
            .iter()
            .map(|split| {
                let mut lines = split.lines();
                // first line is Monkey n, we can infer that from the iterator
                lines.next();
                let mut starting_items_splits = lines.next().unwrap().split(": ");
                // we only need the second part again
                starting_items_splits.next();
                let items: Vec<usize> = starting_items_splits
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|it| it.parse().unwrap())
                    .collect();

                let operation_line = lines.next().unwrap();
                let captures_multiply = multiply_regex.captures(operation_line);
                let captures_add = add_regex.captures(operation_line);

                let operation = if captures_multiply.is_some() {
                    let unwrapped = captures_multiply.unwrap();
                    let factor: usize = unwrapped.get(1).unwrap().as_str().parse().unwrap();
                    Operation::Multiply(factor)
                } else if captures_add.is_some() {
                    let unwrapped = captures_add.unwrap();
                    let summand: usize = unwrapped.get(1).unwrap().as_str().parse().unwrap();
                    Operation::Add(summand)
                } else {
                    Operation::SelfMultiply
                };

                let test: usize = lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .last()
                    .expect("Should've found a last item")
                    .parse()
                    .unwrap();
                let true_target: usize = lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .last()
                    .expect("Should've found a last item")
                    .parse()
                    .unwrap();
                let false_target: usize = lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .last()
                    .expect("Should've found a last item")
                    .parse()
                    .unwrap();

                lcm *= test;

                return Monkey {
                    items,
                    operation,
                    test,
                    true_target,
                    false_target,
                };
            })
            .collect();

        (monkeys, lcm)
    }
}
