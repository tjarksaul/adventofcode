use std::collections::HashMap;

fn main() {
    let input = read::read_input(String::from("input.txt"));

    let size_map = get_directory_sizes(input);

    let sizes: Vec<i32> = size_map.values().cloned().collect();

    let total_size = find_sum_of_small_directories(sizes.iter().cloned().collect());

    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = size_map[&String::from("/")];

    let required_to_delete = calculate_required_space(total_space, required_space, used_space);

    let deletable_size = find_smallest_directory_to_delete(sizes, required_to_delete);

    println!("Sum of small directory sizes: {}", total_size);
    println!("Size of smallest directory to delete: {}", deletable_size);
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

fn get_directory_sizes(input: HashMap<String, Vec<File>>) -> HashMap<String, i32> {
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

    return size_map;
}

fn find_sum_of_small_directories(sizes: Vec<i32>) -> i32 {
    return sizes
        .iter()
        .filter(|size| **size <= 100000)
        .fold(0, |a, b| a + b);
}

fn calculate_required_space(total_space: i32, required_space: i32, used_space: i32) -> i32 {
    return required_space - (total_space - used_space);
}

fn find_smallest_directory_to_delete(sizes: Vec<i32>, required_space: i32) -> i32 {
    let mut deletable_size = i32::MAX;

    for size in sizes {
        if size > required_space && size < deletable_size {
            deletable_size = size;
        }
    }

    return deletable_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> HashMap<String, Vec<File>> {
        return HashMap::from([
            (
                String::from("/"),
                vec![
                    File {
                        name: String::from("b.txt"),
                        size: 14848514,
                    },
                    File {
                        name: String::from("c.dat"),
                        size: 8504156,
                    },
                ],
            ),
            (
                String::from("/a"),
                vec![
                    File {
                        name: String::from("f"),
                        size: 29116,
                    },
                    File {
                        name: String::from("g"),
                        size: 2557,
                    },
                    File {
                        name: String::from("h.lst"),
                        size: 62596,
                    },
                ],
            ),
            (
                String::from("/a/e"),
                vec![File {
                    name: String::from("i"),
                    size: 584,
                }],
            ),
            (
                String::from("/d"),
                vec![
                    File {
                        name: String::from("j"),
                        size: 4060174,
                    },
                    File {
                        name: String::from("d.log"),
                        size: 8033020,
                    },
                    File {
                        name: String::from("d.ext"),
                        size: 5626152,
                    },
                    File {
                        name: String::from("k"),
                        size: 7214296,
                    },
                ],
            ),
        ]);
    }

    #[test]
    fn test_calculates_size_correctly() {
        let dirs = get_input();

        let sizes = get_directory_sizes(dirs);

        let total_size = find_sum_of_small_directories(sizes.values().cloned().collect());

        assert_eq!(total_size, 95437);
    }

    #[test]
    fn test_calculates_required_space_correctly() {
        let total_space = 70000000;
        let required_space = 30000000;
        let used_space = 48381165;

        let required_to_delete = calculate_required_space(total_space, required_space, used_space);

        assert_eq!(required_to_delete, 8381165);
    }

    #[test]
    fn test_finds_smallest_directory_to_delete_correctly() {
        let dirs = vec![584, 94853, 24933642, 48381165];
        let required_to_delete = 8381165;

        let deletable_size = find_smallest_directory_to_delete(dirs, required_to_delete);

        assert_eq!(deletable_size, 24933642);
    }
}

mod read {
    use super::Command;
    use super::File;
    use super::IO;
    use std::collections::HashMap;
    use std::fs;

    pub fn read_input(fname: String) -> HashMap<String, Vec<File>> {
        let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

        let mut splits: Vec<&str> = contents.split('$').collect();
        splits.drain(0..1);

        let mut commands: Vec<IO> = vec![];
        for split in splits {
            let mut lines = split.lines();

            let command = lines.next().unwrap().trim();

            if command == "ls" {
                commands.push(IO {
                    command: Command::Ls,
                    output: lines.map(|a| String::from(a)).collect(),
                });
            } else {
                let io: Vec<&str> = command.split(' ').collect();
                let argument = String::from(io[1]);
                commands.push(IO {
                    command: Command::Cd(argument),
                    output: vec![],
                });
            }
        }

        let mut current_path: Vec<String> = vec![];
        let mut current_entries: Vec<File> = vec![];
        let mut entry_map: HashMap<String, Vec<File>> = HashMap::from([]);

        for io in commands {
            if let Command::Cd(argument) = io.command {
                if current_path.len() > 0 && current_entries.len() > 0 {
                    entry_map.insert(current_path.join("/"), current_entries);
                }
                current_entries = vec![];
                if argument == ".." {
                    current_path.remove(current_path.len() - 1);
                } else {
                    current_path.push(argument);
                }
            } else {
                current_entries = io
                    .output
                    .iter()
                    .filter_map(|line| {
                        let splits: Vec<&str> = line.split(' ').collect();
                        let name = String::from(splits[1]);
                        if splits[0] == "dir" {
                            return None;
                        }
                        let size = splits[0].parse::<i32>().unwrap();
                        return Some(File { name, size });
                    })
                    .collect();
            }
        }

        entry_map.insert(current_path.join("/"), current_entries);

        return entry_map;
    }
}
