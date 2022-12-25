fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));

    let result = calculate_snafu_result(&input);

    dbg!(result);
}

fn calculate_snafu_result(input: &Vec<String>) -> String {
    dec_to_snafu(
        input
            .iter()
            .map(|it| snafu_to_dec(&it))
            .fold(0, |a, b| a + b),
    )
}

fn snafu_to_dec(number: &str) -> i64 {
    let mut dec = 0;

    let number_len = number.len() as u32;
    for (i, digit) in number.chars().enumerate() {
        let exponent = number_len - (i + 1) as u32;
        let multiplier = match digit {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Digit doesn't exist"),
        };

        let value = 5_i64.pow(exponent);

        dec += value * multiplier;
    }

    dec
}

fn dec_to_snafu<'a>(number: i64) -> String {
    let mut highest_power = 0;
    let mut power = 0;
    loop {
        let result = number / 5_i64.pow(power);

        if result >= 1 {
            highest_power = power;
        } else {
            break;
        }

        power += 1;
    }

    let mut remaining = number;
    let mut powers = vec![0; highest_power as usize + 10];
    for power in (0..=highest_power).rev() {
        let multiplier = remaining / 5_i64.pow(power);

        if multiplier <= 2 {
            // normal multiplier
            powers[power as usize] = multiplier;
        } else {
            // we have 3 or 4, need to shift left
            let mut carry = 1;
            let mut add = 1;
            while carry > 0 {
                let idx = (power + add) as usize;
                powers[idx] += carry;
                if powers[idx] > 2 {
                    carry = powers[idx] - 2;
                    let old_value = powers[idx];
                    powers[idx] = old_value - 5;
                } else {
                    carry = 0;
                }
                add += 1;
            }
            powers[power as usize] = multiplier - 5;
        }

        remaining -= multiplier * 5_i64.pow(power);
    }

    let highest_power = powers.iter().enumerate().fold(0, |high, (idx, &cur)| {
        if cur > 0 {
            return idx;
        }
        return high;
    });

    let mut output: String = "".to_owned();

    for idx in (0..=highest_power).rev() {
        let chr = match powers[idx] {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => panic!("invalid number"),
        };
        output.push_str(chr);
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_snafu_result() {
        let input = vec![
            "1=-0-2".to_owned(),
            "12111".to_owned(),
            "2=0=".to_owned(),
            "21".to_owned(),
            "2=01".to_owned(),
            "111".to_owned(),
            "20012".to_owned(),
            "112".to_owned(),
            "1=-1=".to_owned(),
            "1-12".to_owned(),
            "12".to_owned(),
            "1=".to_owned(),
            "122".to_owned(),
        ];

        let result = calculate_snafu_result(&input);

        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn test_snafu_to_dec() {
        let tests = vec![
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];

        for (snafu, dec) in tests {
            let result = snafu_to_dec(snafu);

            assert_eq!(result, dec);
        }
    }

    #[test]
    fn test_dec_to_snafu() {
        let tests = vec![
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];

        for (snafu, dec) in tests {
            let result = dec_to_snafu(dec);

            assert_eq!(result, snafu);
        }
    }
}

mod read {
    pub fn read_all_lines(input: &str) -> Vec<String> {
        input.lines().map(|l| l.to_owned()).collect()
    }
}
