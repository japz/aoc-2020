use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read");

    let input: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let input_copy = input.clone();
    let input_copy2 = input.clone();


    let counter = 0;
    'outer: for input1 in input.iter() {
        for input2 in input_copy.iter() {
            for input3 in input_copy2.iter() {
                if input1 + input2 + input3 == 2020 {
                    println!("{} + {} + {} = 2020! The answer is {}", input1, input2, input3, input1 * input2 * input3);
                    break 'outer;
                }
            }
        }
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
