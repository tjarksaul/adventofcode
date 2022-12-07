use std::collections::HashMap;

fn main() {
    let input = read::read_input(String::from("input.txt"));

    let sizes = get_directory_sizes(input);

    let total_size = find_sum_of_small_directories(sizes);

    println!("Sum of small directory sizes: {}", total_size);
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
pub struct IO {
    command: Command,
    output: Vec<String>,
}

pub struct File {
    name: String,
    size: i32,
}

fn get_directory_sizes(input: HashMap<String, Vec<File>>) -> Vec<i32> {
    let mut size_map: HashMap<String, i32> = HashMap::from([]);

    for key in input.keys() {
        let files = &input[key];
        let file_size = files.iter().fold(0, |a, b| a + b.size);
        let components: Vec<&str> = key.split('/').collect();
        let mut current_path = String::from("");
        for component in components {
            current_path = current_path.clone() + "/" + component;
            if !size_map.contains_key(&current_path) {
                size_map.insert(current_path.clone(), 0);
            }
            size_map.insert(current_path.clone(), size_map[&current_path] + file_size);
        }
    }

    return size_map.values().cloned().collect();
}

fn find_sum_of_small_directories(sizes: Vec<i32>) -> i32 {
    return sizes.iter().filter(|size| **size <= 100000).fold(0, |a, b| a + b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculates_size_correctly() {
        let dirs: HashMap<String, Vec<File>> = HashMap::from([
            (String::from("/"), vec![
                File { name: String::from("b.txt"), size: 14848514 },
                File { name: String::from("c.dat"), size: 8504156 },
            ]),
            (String::from("/a"), vec![
                File { name: String::from("f"), size: 29116 },
                File { name: String::from("g"), size: 2557 },
                File { name: String::from("h.lst"), size: 62596 },
            ]),
            (String::from("/a/e"), vec![
                File { name: String::from("i"), size: 584 },
            ]),
            (String::from("/d"), vec![
                File { name: String::from("j"), size: 4060174 },
                File { name: String::from("d.log"), size: 8033020 },
                File { name: String::from("d.ext"), size: 5626152 },
                File { name: String::from("k"), size: 7214296 },
            ])
        ]);

        let sizes = get_directory_sizes(dirs);

        let total_size = find_sum_of_small_directories(sizes);

        assert_eq!(total_size, 95437);
    }
}

mod read {
    use super::Command;
    use super::IO;
    use super::File;
    use std::fs;
    use std::collections::HashMap;

    pub fn read_input(fname: String) -> HashMap<String, Vec<File>> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let mut splits: Vec<&str> = contents.split('$').collect();
        splits.drain(0..1);

        let mut commands: Vec<IO> = vec![];
        for split in splits {
            let mut lines = split.lines();

            let command = lines.next().unwrap().trim();

            if command == "ls" {
                commands.push(IO { command: Command::Ls, output: lines.map(|a| String::from(a)).collect() });
            } else {
                let io: Vec<&str> = command.split(' ').collect();
                let argument = String::from(io[1]);
                commands.push(IO { command: Command::Cd(argument), output: vec![] });
            }
        }

        let mut current_path: Vec<String> = vec![];
        let mut current_entries: Vec<File> = vec![];
        let mut entry_map: HashMap<String, Vec<File>> = HashMap::from([]);

        for io in commands {
            if let Command::Cd(argument) = io.command {
                if current_path.len() > 0 && current_entries.len() > 0 {
                    entry_map.insert(
                        current_path.join("/"),
                        current_entries,
                    );
                }
                current_entries = vec![];
                if argument == ".." {
                    current_path.remove(current_path.len() - 1);
                } else {
                    current_path.push(argument);
                }
            } else {
                current_entries = io.output.iter().filter_map(|line| {
                    let splits: Vec<&str> = line.split(' ').collect();
                    let name = String::from(splits[1]);
                    if splits[0] == "dir" {
                        return None;
                    }
                    let size = splits[0].parse::<i32>().unwrap();
                    return Some(File { name: name, size: size });
                }).collect();
            }
        }

        entry_map.insert(
            current_path.join("/"),
            current_entries,
        );

        return entry_map;
    }

}
