use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
    let mut safe_amount = 0;
    for report in input {
        let mut safe = true;
        let mut report = report.clone();
        if report[1] < report[0] {
            report = report.iter().copied().rev().collect();
        }
        let mut previous = report[0];
        for i in 1..report.len() {
            let cur = report[i];
            if cur < previous {
                safe = false;
                break;
            }
            if cur - previous > 3 || cur - previous == 0 {
                safe = false;
                break;
            }
            previous = cur;
        }
        if safe {
            safe_amount += 1;
        }
    }
    safe_amount
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
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let result = part_1(&input);

        assert_eq!(2, result);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<usize>> {
        i.lines()
            .map(|l| l.to_string().split(" ").map(|i| i.parse().unwrap()).collect())
            .collect()
    }
}
