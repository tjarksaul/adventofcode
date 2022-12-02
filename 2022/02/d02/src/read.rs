use std::fs;

pub fn read_input<R>(fname: String, row_parser: fn(&str) -> R) -> Vec<R> {
    let contents = fs::read_to_string(fname)
        .expect("Should have been able to read the file");

    let splits: Vec<&str> = contents.split("\n").collect();

    return splits.iter().map(
        |row| row_parser(row)
    ).collect();
 }
