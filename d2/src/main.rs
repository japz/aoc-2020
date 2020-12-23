#![feature(str_split_once)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::string::ParseError;
use std::str::FromStr;

struct PasswordEntry {
    first: usize,
    second: usize,
    letter: char,
    password: String
}

impl PasswordEntry {
    fn verify(&self) -> bool {
        let mut matches: usize = 0;
        for character in self.password.chars() {
            if character == self.letter {
                matches += 1;
            }
        };
        self.first <= matches && matches <= self.second
    }
    fn verify2(&self) -> bool {
        let mut matches: u32 = 0;
        if self.password.chars().nth(self.first-1).unwrap() == self.letter {
            matches += 1;
        }
        if self.password.chars().nth(self.second-1).unwrap() == self.letter {
            matches += 1;
        }
        matches == 1
    }
}


impl FromStr for PasswordEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule, password) = s.split_once(": ").unwrap();
        let (range, letter) = rule.split_once(" ").unwrap();
        let (min, max) = range.split_once("-").unwrap();
        Ok(PasswordEntry {
            first: min.parse().unwrap(),
            second: max.parse().unwrap(),
            letter: letter.chars().next().unwrap(),
            password: String::from(password)
        })
    }
}

fn readfile() -> Result<Vec<String>, io::Error> {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read");
    let return_values: Vec<String> = s.lines().map(|s| String::from(s)).collect();

    Ok(return_values)
}

fn main() {
    let lines = readfile().expect("Couldn't read lines from file");
    let password_entries: Vec<PasswordEntry> = lines.iter().map(|s| PasswordEntry::from_str(s).unwrap()).collect();

    let valid_entries: Vec<&PasswordEntry> = password_entries.iter().filter(|x| x.verify()).collect();
    let valid_entries2: Vec<&PasswordEntry> = password_entries.iter().filter(|x| x.verify2()).collect();


    println!("There are {} valid passwords according to the first policy", valid_entries.len());
    println!("There are {} valid passwords according to the second policy", valid_entries2.len());

}