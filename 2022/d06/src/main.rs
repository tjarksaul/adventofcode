fn main() {
    let input = read::read_input("input.txt".to_string());
    let position = find_marker_position(input.as_str());

    println!("Found marker at position {}", position);
}

fn find_marker_position(input: &str) -> usize {
    let mut chars = input.chars();
    let mut pos_0: char;
    let mut pos_1 = chars.next().unwrap();
    let mut pos_2 = chars.next().unwrap();
    let mut pos_3 = chars.next().unwrap();

    let mut pos = 3;
    while let Some(cur) = chars.next() {
        pos += 1;
        pos_0 = pos_1;
        pos_1 = pos_2;
        pos_2 = pos_3;
        pos_3 = cur;

        if pos_0 != pos_1
            && pos_0 != pos_2
            && pos_0 != pos_3
            && pos_1 != pos_2
            && pos_1 != pos_3
            && pos_2 != pos_3
        {
            return pos;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finds_markers_correctly() {
        let input_1 = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        let input_2 = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        let input_3 = ("nppdvjthqldpwncqszvftbrmjlhg", 6);
        let input_4 = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        let input_5 = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

        assert_eq!(find_marker_position(input_1.0), input_1.1);
        assert_eq!(find_marker_position(input_2.0), input_2.1);
        assert_eq!(find_marker_position(input_3.0), input_3.1);
        assert_eq!(find_marker_position(input_4.0), input_4.1);
        assert_eq!(find_marker_position(input_5.0), input_5.1);
    }
}

mod read {
    use std::fs;

    pub fn read_input(fname: String) -> String {
        return fs::read_to_string(fname).expect("Should have been able to read the file");
    }
}
