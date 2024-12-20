use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Instruction>) -> usize {
    input.iter().fold(0, |prev, cur| prev + match cur {
        &Instruction::Mul(op) => op.run(),
        _ => 0,
    })
}

fn part_2(input: &Vec<Instruction>) -> usize {
    let mut disabled = false;
    let mut result = 0;

    for &instruction in input {
        match instruction {
            Instruction::Do => {
                disabled = false;
            },
            Instruction::Dont => {
                disabled = true;
            },
            Instruction::Mul(op) => {
                if !disabled {
                    result += op.run();
                }
            }
        }
    }

    result
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Do,
    Dont,
    Mul(Mul),
}

#[derive(Debug, Clone, Copy)]
struct Mul(usize, usize);

impl Mul {
    fn run(self) -> usize {
        self.0 * self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

        let data = read::read_all_lines(input);

        let result = part_1(&data);

        assert_eq!(161, result);
    }

    #[test]
    fn runs_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

        let data = read::read_all_lines(input);

        let result = part_2(&data);

        assert_eq!(48, result);
    }
}

mod read {
    use super::{Mul, Instruction};

    #[derive(Debug, Clone)]
    #[derive(PartialEq)]
    enum ParseState {
        ClosingParen,
        InNumber(Vec<char>),
        Comma,
        OpeningParen,
    }

    pub fn read_all_lines(input: String) -> Vec<Instruction> {
        let verbose = false;
        let very_verbose = false;
        let mut instructions = vec![];

        let mut i = 3;
        if input.len() < i {
            return vec![];
        }

        let input = input.as_bytes();
        while i < input.len() {
            // println!("{i}, {}", input[i] as char);
            if very_verbose {
                print!("{}", input[i] as char);
            }
            if input[i - 3] == 'd' as u8 
                && input[i - 2] == 'o' as u8 
                && input[i - 1] == '(' as u8 
                && input[i] == ')' as u8 {
                    if verbose {
                        println!("Found instruction do() at index {}", i - 3);
                    }
                    instructions.push(Instruction::Do);
            }

            if  i >= 6
                && input[i - 6] == 'd' as u8
                && input[i - 5] == 'o' as u8
                && input[i - 4] == 'n' as u8
                && input[i - 3] == '\'' as u8 
                && input[i - 2] == 't' as u8 
                && input[i - 1] == '(' as u8 
                && input[i] == ')' as u8 {
                    if verbose {
                        println!("Found instruction don't() at index {}", i - 3);
                    }
                    instructions.push(Instruction::Dont);
            }

            if i >= 8 && input[i] == ')' as u8 {
                let mut parsing_state = ParseState::ClosingParen;
                let mut operands: Vec<usize> = vec![];
                if very_verbose {
                    println!("Found possible instruction start at idx {i}");
                }

                for j in (0..i).rev() {
                    if very_verbose {
                        println!("{j}, {} Parsing state: {parsing_state:?}", input[j] as char);
                        println!("Operands: {:?}", operands);
                    }
                    if parsing_state == ParseState::ClosingParen {
                        match input[j] {
                            // '0'..'9'
                            48..58 => {
                                if very_verbose {
                                    println!("Found number {}", input[j] as char);
                                }
                                // parse to usize and add to ParseState
                                let num = input[j] as char;
                                parsing_state = ParseState::InNumber(vec![num]);
                                continue;
                            }
                            _ => { break; }
                        }
                    }
                    if let ParseState::InNumber(nums) = parsing_state.clone() {
                        match input[j] {
                            // '0'..'9'
                            48..58 => {
                                // parse to usize and add to ParseState
                                if very_verbose {
                                    println!("Found number {}", input[j] as char);
                                }
                                let num = input[j] as char;
                                let mut nums = nums;
                                nums.push(num);
                                parsing_state = ParseState::InNumber(nums);
                                continue;
                            }
                            // ','
                            44 => {
                                // check if we only have one number, else break
                                if very_verbose {
                                    println!("Found comma");
                                }
                                if operands.len() > 0 {
                                    break;
                                }

                                let s: String = nums.into_iter().rev().collect();
                                let operand = s.parse().unwrap();
                                operands.push(operand);
                                parsing_state = ParseState::Comma;
                                continue;
                            }
                            // '('
                            40 => {
                                // check if we have two numbers, else break
                                if very_verbose {
                                    println!("Found opening paren");
                                }
                                if operands.len() != 1 {
                                    break;
                                }

                                let s: String = nums.into_iter().rev().collect();
                                let operand = s.parse().unwrap();
                                operands.push(operand);
                                parsing_state = ParseState::OpeningParen;
                                continue;
                            }
                            _ => { break; }
                        }
                    }

                    if parsing_state == ParseState::Comma { 
                        match input[j] {
                            // '0'..'9'
                            48..58 => {
                                // parse to usize and add to ParseState
                                if very_verbose {
                                    println!("Found number {}", input[j] as char);
                                }
                                let num = input[j] as char;
                                parsing_state = ParseState::InNumber(vec![num]);
                                continue;
                            }
                            _ => { break; }
                        }
                    }

                    if parsing_state == ParseState::OpeningParen && j >= 2 {
                        if input[j - 2] == 109 // 'm'
                            && input[j - 1] == 117 // 'u'
                            && input[j] == 108 { // 'l'
                                // we found an actual mul instruction and can push it to our instruction set
                                if verbose {
                                    println!("Found instruction mul({}, {}) at index {}", operands[1], operands[0], i - 2);
                                }
                                instructions.push(Instruction::Mul(Mul(operands[1], operands[0])));
                        }
                        break;
                    }
                }
            }


            i += 1;
        }

        let mut counts = (0, 0, 0);
        for instruction in &instructions {
            match instruction {
                Instruction::Do => { counts.0 += 1 },
                Instruction::Dont => { counts.1 += 1 },
                Instruction::Mul(_) => { counts.2 += 1 },
            }
        }

        println!("Found {} Do, {} Don't, {} Mul instructions", counts.0, counts.1, counts.2);

        instructions
    }
}
