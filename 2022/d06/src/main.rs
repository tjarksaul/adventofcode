use std::collections::HashSet;

fn main() {
    let input = read::read_input("input.txt".to_string());
    let packet_position = find_marker_position(input.as_str(), 4);
    let message_position = find_marker_position(input.as_str(), 14);

    println!("Found packet marker at position {}", packet_position);
    println!("Found message marker at position {}", message_position);
}

fn find_marker_position(input: &str, distinct_marker_length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();

    for i in distinct_marker_length..chars.len() {
        let slice = &chars[(i-distinct_marker_length)..i];

        let set_len = HashSet::<&char>::from_iter(slice).len();

        if set_len == distinct_marker_length {
            return i;
        }
    }

    return chars.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finds_packet_markers_correctly() {
        let input_1 = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        let input_2 = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        let input_3 = ("nppdvjthqldpwncqszvftbrmjlhg", 6);
        let input_4 = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        let input_5 = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

        assert_eq!(find_marker_position(input_1.0, 4), input_1.1);
        assert_eq!(find_marker_position(input_2.0, 4), input_2.1);
        assert_eq!(find_marker_position(input_3.0, 4), input_3.1);
        assert_eq!(find_marker_position(input_4.0, 4), input_4.1);
        assert_eq!(find_marker_position(input_5.0, 4), input_5.1);
    }

    #[test]
    fn test_finds_message_markers_correctly() {
        let input_1 = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        let input_2 = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        let input_3 = ("nppdvjthqldpwncqszvftbrmjlhg", 23);
        let input_4 = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        let input_5 = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);

        assert_eq!(find_marker_position(input_1.0, 14), input_1.1);
        assert_eq!(find_marker_position(input_2.0, 14), input_2.1);
        assert_eq!(find_marker_position(input_3.0, 14), input_3.1);
        assert_eq!(find_marker_position(input_4.0, 14), input_4.1);
        assert_eq!(find_marker_position(input_5.0, 14), input_5.1);
    }
}

mod read {
    use std::fs;

    pub fn read_input(fname: String) -> String {
        return fs::read_to_string(fname).expect("Should have been able to read the file");
    }
}
