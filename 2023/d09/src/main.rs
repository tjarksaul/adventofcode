fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let sum = calculate_sum_extrapolated_values(&input);

    dbg!(sum);
}

fn calculate_next_row(row: &Vec<i64>) -> Vec<i64> {
    let mut result = vec![];
    for i in 1..row.len() {
        result.push(row[i] - row[i-1]);
    }
    return result;
}

fn is_final_row(row: &Vec<i64>) -> bool {
    row.iter().all(|n| *n == 0)
}

fn calculate_final_values(rows: &Vec<Vec<i64>>) -> i64 {
    let mut rows = rows.to_vec();
    let mut new_value = 0;
    for i in (0..rows.len()).rev() {
        rows[i].push(new_value);
        if i == 0 {
            return new_value;
        } else {
            new_value = rows[i-1].last().copied().unwrap() + new_value;
        }
    }
    -1
}

fn calculate_final_value(row: &Vec<i64>) -> i64 {
    let mut current_row = row.to_vec();
    let mut rows = vec![row.to_vec()];
    while !is_final_row(&current_row) {
        let next_row = calculate_next_row(&current_row);
        current_row = next_row.to_vec();
        rows.push(next_row.to_vec());
    }
    calculate_final_values(&rows)
}

fn calculate_sum_extrapolated_values(rows: &Vec<Vec<i64>>) -> i64 {
    rows.iter().fold(0, |acc, cur| acc + calculate_final_value(&cur))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_sum_correctly() {
        let input = vec![
            vec![0, 3, 6, 9, 12, 15,],
            vec![1, 3, 6, 10, 15, 21,],
            vec![10, 13, 16, 21, 30, 45,],
        ];

        let sum = calculate_sum_extrapolated_values(&input);

        assert_eq!(sum, 114);
    }
}

mod read {
    use nom::{
        bytes::complete::{tag}, character::complete as cc, combinator::all_consuming,
        multi::{separated_list1}, Finish, IResult,
    };

    // Sample input
    // 0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45
    pub fn read_all_lines(i: &'static str) -> Vec<Vec<i64>> {
        i.lines()
            .map(|l| all_consuming(parse_row)(l).finish().unwrap().1)
            .collect()
    }

    fn parse_row(i: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(tag(" "), cc::i64)(i)
    }
}
