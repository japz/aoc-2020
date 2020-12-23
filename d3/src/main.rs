#![feature(extend_one)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::string::ParseError;
use std::str::FromStr;

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Clone)]
struct Map {
    x_size: usize,
    y_size: usize,
    trees: Vec<Vec<bool>>,
    position: Coord
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<bool>> = Vec::new();
        let mut x_size = 0;
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                x_size = line.len();
            }
            map.extend_one(line.chars().map(|s| s == '#').collect());
        }

        Ok(Map{
            x_size,
            y_size: map.len(),
            trees: map,
            position: Coord{x:0, y:0}
        })
    }
}

impl Map{
    fn current_position_has_tree(&self) -> bool {
        // Wrap-around the map in the X-direction
        self.trees[self.position.y][self.position.x % self.x_size]
    }
    fn step(&mut self, vector: (usize, usize)) {
        self.position.x += vector.0;
        self.position.y += vector.1
    }
    fn out_of_bounds(&self) -> bool {
        self.position.y >= self.y_size
    }
}

struct Scenario {
    map: Map,
    slope: (usize, usize)
}

impl Scenario {
    fn how_many_trees(&mut self) -> u32 {
        let mut trees_encountered = 0;
        loop {
            // println!("I am on position ({}, {})", self.map.position.x, self.map.position.y);
            if self.map.current_position_has_tree() {
                trees_encountered += 1;
            }
            self.map.step(self.slope);
            if self.map.out_of_bounds() {
                break;
            }
        }
        trees_encountered
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
    let lines = readfile().expect("Couldn't read lines from file");
    let mut map = Map::from_str(&lines[..]).unwrap();
    let slope: (usize, usize) = (3, 1);

    println!("Loaded map of size X={}, Y={}", map.x_size, map.y_size);

    let mut scenarios: [Scenario; 5] = [
        Scenario {map: map.clone(), slope: (1,1)},
        Scenario {map: map.clone(), slope: (3,1)},
        Scenario {map: map.clone(), slope: (5,1)},
        Scenario {map: map.clone(), slope: (7,1)},
        Scenario {map: map.clone(), slope: (1,2)},
    ];

    let mut trees_multiplied = 1;
    for (i, s) in scenarios.iter_mut().enumerate() {
        let num_trees = s.how_many_trees();
        trees_multiplied = trees_multiplied * num_trees;
        println!("We encountered {} trees in scenario {}", num_trees, i+1);
    }
    println!("We multiplied the trees for some reason, and found {}", trees_multiplied)
}