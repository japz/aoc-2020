#![feature(str_split_once)]

use std::io;
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::string::ParseError;
use std::path::Path;
use std::fs::{File, read};


#[derive(Debug)]
struct Passport {
    byr: Option<String>,  // Birth Year
    iyr: Option<String>,  // Issue Year
    eyr: Option<String>,  // Expiration Year
    hgt: Option<String>,  // Height
    hcl: Option<String>,  // Hair Color
    ecl: Option<String>,  // Eye Color,
    pid: Option<String>,  // Passport ID,
    cid: Option<String>,  // Country ID
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
        for (k,v) in s.split_whitespace().map(|x| x.split_once(':').unwrap()) {
            match k {
                "byr" => passport.byr = Option::from(String::from(v)),
                "iyr" => passport.iyr = Option::from(String::from(v)),
                "eyr" => passport.eyr = Option::from(String::from(v)),
                "hgt" => passport.hgt = Option::from(String::from(v)),
                "hcl" => passport.hcl = Option::from(String::from(v)),
                "ecl" => passport.ecl = Option::from(String::from(v)),
                "pid" => passport.pid = Option::from(String::from(v)),
                "cid" => passport.cid = Option::from(String::from(v)),
                _ => {}
            }
        }
        Ok(passport)
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self.byr.is_none() || self.iyr.is_none() || self.eyr.is_none() || self.hgt.is_none() || self.hcl.is_none() || self.ecl.is_none() || self.pid.is_none() {
            return false;
        }
        true
    }
}

fn readfile() -> Result<String, io::Error> {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("couldn't read");

    Ok(s)
}

fn main() {
    // let input = BufReader::new(data_file("input.txt")?);
    let test_passport = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm").unwrap();
    println!("{:#?}", test_passport);
    if test_passport.is_valid() {
        println!("This passport is valid");
    } else {
        println!("This passport is invalid");
    }
    let passports: Vec<Passport> = readfile().unwrap().split("\n\n")
        .map(|s| Passport::from_str(s).unwrap())
        .filter(|x| x.is_valid())
        .collect();

    println!("We have {} valid passports", passports.len())
}
