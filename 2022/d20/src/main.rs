use std::collections::VecDeque;

fn main() {
    let data = read::parse_all_lines(include_str!("../input.txt"));

    for (mixes, multiplier) in [(1, 1), (10, 811589153)] {
        let coordinates = find_coordinates(&data, mixes, multiplier);

        dbg!(mixes, multiplier, coordinates);
    }
}

fn find_coordinates(data: &Vec<i64>, mixes: u64, multiplier: i64) -> i64 {
    let data: Vec<_> = data.iter().map(|&x| x * multiplier).enumerate().collect();
    let mut output = VecDeque::from(data);
    let len = (output.len() - 1) as i64; // -1 bc we always have 1 item removed

    for _ in 0..mixes {
        for i in 0..output.len() {
            while output[0].0 != i {
                // find curent location of `i`-th element
                let front = output.pop_front().unwrap();
                output.push_back(front);
            }

            let current = output.pop_front().unwrap();
            let amount = current.1.rem_euclid(len);

            for _ in 0..amount {
                // cue `amount` steps forward
                let front = output.pop_front().unwrap();
                output.push_back(front);
            }

            output.push_back(current);
        }
    }

    let index_0 = output
        .iter()
        .position(|&value| value.1 == 0)
        .expect("Should've found a 0");

    let real_len = output.len();

    [1000, 2000, 3000].iter().fold(0, |a, b| a + output[(b + index_0) % real_len].1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finds_coordinates_correctly() {
        let data = vec![1, 2, -3, 3, -2, 0, 4];

        let coordinates = find_coordinates(&data, 1, 1);

        assert_eq!(coordinates, 3);
    }

    #[test]
    fn test_finds_coordinates_correctly_part_2() {
        let data = vec![1, 2, -3, 3, -2, 0, 4];

        let coordinates = find_coordinates(&data, 10, 811589153);

        assert_eq!(coordinates, 1623178306);
    }

    #[test]
    fn test_read() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        let expected_data = vec![1, 2, -3, 3, -2, 0, 4];

        let data = read::parse_all_lines(input);

        assert_eq!(data, expected_data);
    }
}

mod read {
    pub fn parse_all_lines<'a>(i: &'a str) -> Vec<i64> {
        i.lines().map(|l| l.parse().unwrap()).collect()
    }
}
