use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<u8>>) -> usize {
    let dropped = drop_stones(&input);
    // print_result(&dropped);
    calculate_weight(&dropped)
}

fn drop_stones(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    result.push(input[0].to_vec());
    for i in 1..input.len() {
        result.push(vec![]);
        for j in 0..input[i].len() {
            if input[i][j] == b'O' {
                let mut target = i;
                for k in (0..i).rev() {
                    if result[k][j] == b'.' {
                        target = k;
                    } else if result[k][j] == b'#' || result[k][j] == b'O' {
                        break;
                    }
                }
                if result[i].len() <= j {
                    result[i].push(b'.')
                } else {
                    result[i][j] = b'.';
                }
                result[target][j] = b'O';
            } else {
                result[i].push(input[i][j]);
            }
        }
    }
    result
}

fn calculate_weight(input: &Vec<Vec<u8>>) -> usize {
    input.iter().rev().enumerate().fold(0, |acc, (i, cur)| 
        acc + ((i + 1) * cur.iter().fold(0, |inner_acc, chr| inner_acc + if *chr == b'O' { 1 } else { 0 }))
    )
}

fn print_result(input: &Vec<Vec<u8>>) {
    for row in input {
        for chr in row {
            print!("{}", *chr as char);
        }
        print!("\n");
    }
}

fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        assert_eq!(0, 1);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<u8>> {
        i.lines()
            .map(|l| l.chars().map(|c| c as u8).collect())
            .collect()
    }
}
