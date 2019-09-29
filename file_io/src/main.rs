static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    let filepath = Path::new("out/output.txt");
    let display = filepath.display();

    let mut file = match File::create(&filepath) {
        Err(why) => panic!("couldnt create {}: {}", 
                            display,
                            why.description()),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("Couldnt write: {}: {}",
                            display,
                            why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    };

    let contents = fs::read_to_string("out/output.txt")
        .expect("Something went wrong reading the file");

    println!("{}", contents);
}
