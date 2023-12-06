use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location(isize, isize);

impl Location {
    fn get_neighbors(&self) -> HashSet<Location> {
        let mut neighbors: HashSet<Location> = HashSet::new();

        [-1, 0, 1].into_iter().for_each(|xdiff| {
            [-1, 0, 1].into_iter().for_each(|ydiff| {
                neighbors.insert(Location(self.0 + xdiff, self.1 + ydiff));
            })
        });

        neighbors
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
    location: HashSet<Location>,
}

/// struct to hold the schematic, for easier addressing of specific locations.
/// positive X = right, positive Y = down
struct Schematic {
    contents: Vec<String>,
}

impl Schematic {
    fn read_file(path: &str) -> Result<Schematic, String> {
        let input = File::open(path).map_err(|_| "Couldn't open file")?;
        let reader = BufReader::new(input);

        let schem: Vec<String> = reader
            .lines()
            .map(|line| line.expect("Line could not be read"))
            .collect();

        Ok(Schematic { contents: schem })
    }

    fn symbol_at(&self, loc: Location) -> Option<char> {
        if loc.0 < 0 || loc.1 < 0 {
            return None;
        }

        self.contents
            .get(loc.1 as usize)?
            .chars()
            .nth(loc.0 as usize)
    }

    fn find_numbers(&self) -> Vec<Number> {
        self.contents
            .iter()
            .enumerate()
            .map(|(y, line)| line.chars().enumerate().map(|(x, c)| {}));

        vec![]
    }
}

fn main() -> Result<(), String> {
    let schem = Schematic::read_file("input.txt")?;

    let numbers: Vec<Number> = schem.find_numbers();

    Ok(())
}
