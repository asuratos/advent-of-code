use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), String> {
    let input = File::open("input").map_err(|_| "Couldn't read file")?;
    let reader = BufReader::new(input);

    let cards: u32 = reader
        .lines()
        .filter_map(|line| {
            let unwrappedline = line.expect("Couldn't read lines");
            let (_cardnum, card) = unwrappedline.split_once(':').expect("Bad Card Format");
            card.split('|')
                .into_iter()
                .map(|section| {
                    HashSet::from_iter(
                        section
                            .split_whitespace()
                            .map(|numstr| numstr.parse::<u32>().expect("Unable to parse number")),
                    )
                })
                .reduce(|acc: HashSet<u32>, set: HashSet<u32, _>| {
                    acc.intersection(&set).cloned().collect::<HashSet<u32, _>>()
                })
                .map(|map: HashSet<u32, _>| {
                    if map.len() == 0 {
                        0
                    } else {
                        2_u32.pow(map.len() as u32 - 1)
                    }
                })
        })
        .sum();

    println!("{:?}", cards);

    Ok(())
}
