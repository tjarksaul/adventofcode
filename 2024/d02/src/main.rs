use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

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

fn part_2(input: &Vec<Vec<usize>>) -> usize {
    let mut safe_amount = 0;
    for (j, report) in input.iter().enumerate() {
        let mut safe = false;
        let report = report.clone();
        for k in (0..(report.len() + 1)).rev() {
            let mut removed_safe = true;
            let mut report_copy = report.clone();
            if k < report.len() {
                report_copy.remove(k);
            }
            let mut previous = report_copy[0];
            let increasing = report_copy[1] > report_copy[0];
            for i in 1..report_copy.len() {
                let cur = report_copy[i];
                if (increasing && cur < previous) || (!increasing && cur > previous) {
                    removed_safe = false;
                    break;
                }
                if cur.abs_diff(previous) > 3 || cur.abs_diff(previous) == 0 {
                    removed_safe = false;
                    break;
                }
                previous = cur;
            }
            if removed_safe {
                safe = true;
                break;
            }
        }
        if safe {
            safe_amount += 1;
        }
    }
    safe_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<usize>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn runs_part_1() {
        let input = get_input();

        let result = part_1(&input);

        assert_eq!(2, result);
    }

    #[test]
    fn runs_part_2() {
        let input = get_input();

        let result = part_2(&input);

        assert_eq!(4, result);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<usize>> {
        i.lines()
            .map(|l| l.to_string().split(" ").map(|i| i.parse().unwrap()).collect())
            .collect()
    }
}
