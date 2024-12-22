use derive_more::Display;
use std::{cmp::Ordering, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &(Vec<Order>, Vec<Vec<usize>>)) -> usize {
    let orders = input.0.to_vec();
    let reports = input.1.to_vec();

    reports
        .iter()
        .filter(|ps| is_correctly_ordered(&ps, &orders))
        .map(|ps| ps[ps.len() / 2])
        .fold(0, |prev, cur| prev + cur)
}

fn part_2(input: &(Vec<Order>, Vec<Vec<usize>>)) -> usize {
    let orders = input.0.to_vec();
    let reports = input.1.to_vec();

    let incorrect_reports = reports
        .iter()
        .filter(|ps| !is_correctly_ordered(&ps, &orders));

    let correctly_ordered = incorrect_reports.clone().map(|ps| {
        let mut ps = ps.clone();
        ps.sort_by(|a, b| {
            for order in &orders {
                if order.0 == *a && order.1 == *b {
                    return Ordering::Greater;
                }
                if order.1 == *a && order.0 == *b {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        });
        ps
    });

    correctly_ordered
        .map(|ps| ps[ps.len() / 2])
        .fold(0, |prev, cur| prev + cur)
}

fn is_correctly_ordered(report: &Vec<usize>, orders: &Vec<Order>) -> bool {
    for i in 1..report.len() {
        let result = orders.iter().fold(true, |prev, order| {
            if report[i] == order.0 {
                // checking that all previous items are not the one that is supposed to come after this one
                return prev && (0..i).fold(true, |prev, j| prev && report[j] != order.1);
            }
            prev && true
        });
        if !result {
            return false;
        }
    }
    return true;
}

#[derive(Clone, Display)]
#[display("{}|{}", _0, _1)]
struct Order(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();

        let input = read::read_all_lines(s);

        let result = part_1(&input);

        assert_eq!(143, result);
    }

    #[test]
    fn runs_part_2() {
        let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();

        let input = read::read_all_lines(s);

        let result = part_2(&input);

        assert_eq!(123, result);
    }
}

mod read {
    use super::Order;

    pub fn read_all_lines(i: String) -> (Vec<Order>, Vec<Vec<usize>>) {
        let mut orders = vec![];
        let mut pages = vec![];

        i.lines().for_each(|l| {
            let l = l.to_string();
            if l.contains("|") {
                // we have an order
                let (i1, i2) = l.split_once("|").unwrap();
                let order = Order(i1.parse().unwrap(), i2.parse().unwrap());
                orders.push(order);
            } else if l.contains(",") {
                // we have a page list
                let items: Vec<usize> = l.split(",").map(|i| i.parse().unwrap()).collect();
                pages.push(items);
            }
        });

        (orders, pages)
    }
}
