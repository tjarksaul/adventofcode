use std::collections::HashMap;
use std::ops;

fn main() {
    let input = read::parse_all_lines(include_str!("../input.txt"));

    let monkey = input.get("root").unwrap();
    let result = get_and_perform_monkey(monkey, &input);

    dbg!(result);
}

#[derive(PartialEq, Debug)]
pub enum Monkey<'a> {
    Number(i64),
    Operation(Operation<'a>),
}

#[derive(PartialEq, Debug)]
pub enum Operation<'a> {
    Sum(&'a str, &'a str),
    Difference(&'a str, &'a str),
    Product(&'a str, &'a str),
    Quotient(&'a str, &'a str),
}

fn perform_operation(operation: &Operation, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match operation {
        Operation::Sum(left, right) => {
            let left = monkeys.get(left).unwrap();
            let right = monkeys.get(right).unwrap();
            get_monkeys_and_perform(left, right, monkeys, ops::Add::add)
        }
        Operation::Difference(left, right) => {
            let left = monkeys.get(left).unwrap();
            let right = monkeys.get(right).unwrap();
            get_monkeys_and_perform(left, right, monkeys, ops::Sub::sub)
        }
        Operation::Product(left, right) => {
            let left = monkeys.get(left).unwrap();
            let right = monkeys.get(right).unwrap();
            get_monkeys_and_perform(left, right, monkeys, ops::Mul::mul)
        }
        Operation::Quotient(left, right) => {
            let left = monkeys.get(left).unwrap();
            let right = monkeys.get(right).unwrap();
            get_monkeys_and_perform(left, right, monkeys, ops::Div::div)
        }
    }
}

fn get_monkeys_and_perform(
    left: &Monkey,
    right: &Monkey,
    monkeys: &HashMap<&str, Monkey>,
    op: fn(i64, i64) -> i64,
) -> i64 {
    let left = get_and_perform_monkey(left, monkeys);
    let right = get_and_perform_monkey(right, monkeys);
    op(left, right)
}

fn get_and_perform_monkey(monkey: &Monkey, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match monkey {
        Monkey::Number(number) => *number,
        Monkey::Operation(operation) => perform_operation(operation, monkeys),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> HashMap<&'a str, Monkey<'a>> {
        HashMap::from([
            ("root", Monkey::Operation(Operation::Sum("pppw", "sjmn"))),
            ("dbpl", Monkey::Number(5)),
            ("cczh", Monkey::Operation(Operation::Sum("sllz", "lgvd"))),
            ("zczc", Monkey::Number(2)),
            (
                "ptdq",
                Monkey::Operation(Operation::Difference("humn", "dvpt")),
            ),
            ("dvpt", Monkey::Number(3)),
            ("lfqf", Monkey::Number(4)),
            ("humn", Monkey::Number(5)),
            ("ljgn", Monkey::Number(2)),
            (
                "sjmn",
                Monkey::Operation(Operation::Product("drzm", "dbpl")),
            ),
            ("sllz", Monkey::Number(4)),
            (
                "pppw",
                Monkey::Operation(Operation::Quotient("cczh", "lfqf")),
            ),
            (
                "lgvd",
                Monkey::Operation(Operation::Product("ljgn", "ptdq")),
            ),
            (
                "drzm",
                Monkey::Operation(Operation::Difference("hmdt", "zczc")),
            ),
            ("hmdt", Monkey::Number(32)),
        ])
    }

    #[test]
    fn calculates_root_number_correctly() {
        let input = get_input();

        let monkey = input.get("root").unwrap();
        let result = get_and_perform_monkey(monkey, &input);

        assert_eq!(result, 152);
    }

    #[test]
    fn parses_correctly() {
        let str_input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let expected = get_input();

        let parsed = read::parse_all_lines(str_input);

        assert_eq!(parsed, expected);
    }
}

mod read {
    // Sample input:
    // root: pppw + sjmn
    // dbpl: 5
    use super::Monkey;
    use super::Operation;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete as cc,
        combinator::{all_consuming, map},
        sequence::tuple,
        Finish, IResult,
    };
    use std::collections::HashMap;

    fn parse_operation(i: &str) -> IResult<&str, Operation> {
        let (i, (left, _, operator, _, right)) = tuple((
            cc::alpha1,
            tag(" "),
            cc::one_of("+-*/"),
            tag(" "),
            cc::alpha1,
        ))(i)?;
        match operator {
            '+' => Ok((i, Operation::Sum(left, right))),
            '-' => Ok((i, Operation::Difference(left, right))),
            '*' => Ok((i, Operation::Product(left, right))),
            '/' => Ok((i, Operation::Quotient(left, right))),
            _ => panic!("this shouldn't happen"),
        }
    }

    fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
        alt((
            map(cc::i64, Monkey::Number),
            map(parse_operation, Monkey::Operation),
        ))(i)
    }

    fn parse_line(i: &str) -> IResult<&str, (&str, Monkey)> {
        let (i, (key, _, monkey)) = tuple((cc::alpha1, tag(": "), parse_monkey))(i)?;
        Ok((i, (key, monkey)))
    }

    pub fn parse_all_lines<'a>(i: &'a str) -> HashMap<&'a str, Monkey<'a>> {
        i.lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect()
    }
}
