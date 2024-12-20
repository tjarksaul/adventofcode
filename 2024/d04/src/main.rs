use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            let letter = input[r][c];
            // forward
            if letter == 'X' {
                // possible XMAS found
                // check horizontally
                if c < input[r].len() - 3 {
                    if input[r][c+1] == 'M' && input[r][c+2] == 'A' && input[r][c+3] == 'S' { 
                        count += 1;
                    }
                }
                // check vertically
                if r < input.len() - 3 {
                    if input[r+1][c] == 'M' && input[r+2][c] == 'A' && input[r+3][c] == 'S' { 
                        count += 1;
                    }
                }
                // check diagonally right
                if c < input[r].len() - 3 && r < input.len() - 3 {
                    if input[r+1][c+1] == 'M' && input[r+2][c+2] == 'A' && input[r+3][c+3] == 'S' { 
                        count += 1;
                    }
                }
                // check diagonally left
                if c >= 3 && r < input.len() - 3 {
                    if input[r+1][c-1] == 'M' && input[r+2][c-2] == 'A' && input[r+3][c-3] == 'S' { 
                        count += 1;
                    }
                }
            }

            // backward
            if letter == 'S' {
                // possible XMAS found
                // check horizontally
                if c < input[r].len() - 3 {
                    if input[r][c+1] == 'A' && input[r][c+2] == 'M' && input[r][c+3] == 'X' { 
                        count += 1;
                    }
                }
                // check vertically
                if r < input.len() - 3 {
                    if input[r+1][c] == 'A' && input[r+2][c] == 'M' && input[r+3][c] == 'X' { 
                        count += 1;
                    }
                }
                // check diagonally
                if c < input[r].len() - 3 && r < input.len() - 3 {
                    if input[r+1][c+1] == 'A' && input[r+2][c+2] == 'M' && input[r+3][c+3] == 'X' { 
                        count += 1;
                    }
                }
                // check diagonally left
                if c >= 3 && r < input.len() - 3 {
                    if input[r+1][c-1] == 'A' && input[r+2][c-2] == 'M' && input[r+3][c-3] == 'X' { 
                        count += 1;
                    }
                }
                
            }
        }
    }

    count
}

fn part_2(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            let letter = input[r][c];
            // forward
            if letter == 'M' {
                if c < input[r].len() - 2 && r < input.len() - 2 {
                    if input[r+1][c+1] == 'A' && input[r+2][c+2] == 'S' { 
                        // we have a MAS down right, so we need to find an SAM | MAS crossing this one
                        if (input[r][c+2] == 'M' && input[r+2][c] == 'S') || 
                            (input[r][c+2] == 'S' && input[r+2][c] == 'M') {
                                count += 1;
                            }
                    }
                }
            }

            // backward
            if letter == 'S' {
                if c < input[r].len() - 2 && r < input.len() - 2 {
                    if input[r+1][c+1] == 'A' && input[r+2][c+2] == 'M' { 
                        // we have a MAS down right, so we need to find an SAM | MAS crossing this one
                        if (input[r][c+2] == 'M' && input[r+2][c] == 'S') || 
                            (input[r][c+2] == 'S' && input[r+2][c] == 'M') {
                                count += 1;
                            }
                    }
                }
            }   
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let input = vec![
            vec!['M','M','M','S','X','X','M','A','S','M'],
            vec!['M','S','A','M','X','M','S','M','S','A'],
            vec!['A','M','X','S','X','M','A','A','M','M'],
            vec!['M','S','A','M','A','S','M','S','M','X'],
            vec!['X','M','A','S','A','M','X','A','M','M'],
            vec!['X','X','A','M','M','X','X','A','M','A'],
            vec!['S','M','S','M','S','A','S','X','S','S'],
            vec!['S','A','X','A','M','A','S','A','A','A'],
            vec!['M','A','M','M','M','X','M','M','M','M'],
            vec!['M','X','M','X','A','X','M','A','S','X'],
        ];

        let result = part_1(&input);

        assert_eq!(18, result);
    }

    #[test]
    fn runs_part_2() {
        let input = vec![
            vec!['.','M','.','S','.','.','.','.','.','.'],
            vec!['.','.','A','.','.','M','S','M','S','.'],
            vec!['.','M','.','S','.','M','A','A','.','.'],
            vec!['.','.','A','.','A','S','M','S','M','.'],
            vec!['.','M','.','S','.','M','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
            vec!['S','.','S','.','S','.','S','.','S','.'],
            vec!['.','A','.','A','.','A','.','A','.','.'],
            vec!['M','.','M','.','M','.','M','.','M','.'],
            vec!['.','.','.','.','.','.','.','.','.','.'],
        ];

        let result = part_2(&input);

        assert_eq!(9, result);
    }
}

mod read {
    pub fn read_all_lines(i: String) -> Vec<Vec<char>> {
        i.lines()
            .map(|l| l.to_string().chars().collect())
            .collect()
    }
}
