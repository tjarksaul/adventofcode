use std::error::Error;
use pathfinding::prelude::astar;

fn main() -> Result<(), Box<dyn Error>> {
    let str_data = aoc::get_input()?;
    let input = read::read_all_lines(str_data);

    let part1 = part_1(&input);

    let part2 = part_2();

    dbg!(part1, part2);

    Ok(())
}

fn part_1(input: &Vec<Machine>) -> usize {
    input.iter().map(|machine| {
        let GOAL = machine.prize;
        let result = astar(&Pos(0, 0), |p| p.successors(&machine), |p| p.distance(&GOAL) / 3,
                   |p| *p == GOAL);

        if let Some(res) = result {
            res.1
        } else {
            0
        }

    })
    .fold(0, |prev, cur| prev + cur)
}

fn part_2() -> usize {
    0
}

#[derive(Debug)]
struct Button(usize, usize);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        if self.0 > other.0 || self.1 > other.1 {
            return usize::MAX;
        }
      (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
  
    fn successors(&self, machine: &Machine) -> Vec<(Pos, usize)> {
      let &Pos(x, y) = self;
      if x > machine.prize.0 || y > machine.prize.1 {
        // we can't go back in this machine
        return vec![];
      }
      vec![
        (Pos(x + machine.btn_a.0, y + machine.btn_a.1), 3),
        (Pos(x + machine.btn_b.0, y + machine.btn_b.1), 1),
      ]
    }
  }
  
#[derive(Debug)]
struct Machine {
    btn_a: Button,
    btn_b: Button,
    prize: Pos,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_part_1() {
        let s = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279".to_string();

        let input = read::read_all_lines(s);

        dbg!(&input);

        let result = part_1(&input);

        assert_eq!(result, 480);
    }
}

mod read {
    use super::{Button, Pos, Machine};

    pub fn read_all_lines(i: String) -> Vec<Machine> {
        i.split("\n\n")
            .map(|l| {
                let splits: Vec<_> = l.split("\n").collect();

                let (_, btn_a) = splits[0].split_once(": ").unwrap();
                let (btn_a_x_term, btn_a_y_term) = btn_a.split_once(", ").unwrap();
                let (_, btn_a_x_str) = btn_a_x_term.split_once("+").unwrap();
                let btn_a_x = btn_a_x_str.parse().unwrap();
                let (_, btn_a_y_str) = btn_a_y_term.split_once("+").unwrap();
                let btn_a_y = btn_a_y_str.parse().unwrap();

                let (_, btn_b) = splits[1].split_once(": ").unwrap();
                let (btn_b_x_term, btn_b_y_term) = btn_b.split_once(", ").unwrap();
                let (_, btn_b_x_str) = btn_b_x_term.split_once("+").unwrap();
                let btn_b_x = btn_b_x_str.parse().unwrap();
                let (_, btn_b_y_str) = btn_b_y_term.split_once("+").unwrap();
                let btn_b_y = btn_b_y_str.parse().unwrap();

                let (_, prize) = splits[2].split_once(": ").unwrap();
                let (prize_x_term, prize_y_term) = prize.split_once(", ").unwrap();
                let (_, prinze_x_str) = prize_x_term.split_once("=").unwrap();
                let prize_x = prinze_x_str.parse().unwrap();
                let (_, prinze_y_str) = prize_y_term.split_once("=").unwrap();
                let prize_y = prinze_y_str.parse().unwrap();

                Machine{
                    btn_a: Button(btn_a_x, btn_a_y),
                    btn_b: Button(btn_b_x, btn_b_y),
                    prize: Pos(prize_x, prize_y),
                }
            })
            .collect()
    }
}
