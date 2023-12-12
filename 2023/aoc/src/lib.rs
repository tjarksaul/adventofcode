use std::env;
use std::error::Error;
use std::fs::read_to_string;

pub fn get_input() -> Result<String, Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        Err("Missing filename")?
    }
    let name = &args[1];

    Ok(read_to_string(name)?)
}
