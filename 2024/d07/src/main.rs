use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<(usize, VecDeque<usize>)>) -> usize {
    input.iter().fold(0, |prev, cur| {
        prev + if check_sum(cur.0, &cur.1, 0) {
            cur.0
        } else {
            0
        }
    })
}

fn check_sum(result: usize, operands: &VecDeque<usize>, previous: usize) -> bool {
    let mut new_operands = operands.clone();
    let operand = new_operands.pop_front();

    match operand {
        Some(op) => {
            if check_sum(result, &new_operands.clone(), previous + op) {
                return true;
            }

            if check_sum(result, &new_operands.clone(), previous * op) {
                return true;
            }
        }
        _ => {
            if previous == result {
                return true;
            }
        }
    }

    return false;
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let input = vec![
            (190, VecDeque::from([10, 19])),
            (3267, VecDeque::from([81, 40, 27])),
            (83, VecDeque::from([17, 5])),
            (156, VecDeque::from([15, 6])),
            (7290, VecDeque::from([6, 8, 6, 15])),
            (161011, VecDeque::from([16, 10, 13])),
            (192, VecDeque::from([17, 8, 14])),
            (21037, VecDeque::from([9, 7, 18, 13])),
            (292, VecDeque::from([11, 6, 16, 20])),
        ];

        let result = part_1(&input);

        assert_eq!(3749, result);
    }
}

mod read {
    use std::collections::VecDeque;

    pub fn read_all_lines(i: String) -> Vec<(usize, VecDeque<usize>)> {
        i.lines()
            .map(|l| {
                let (res, xs) = l.split_once(": ").unwrap();

                return (
                    res.parse().unwrap(),
                    xs.split(" ").map(|x| x.parse().unwrap()).collect(),
                );
            })
            .collect()
    }
}
