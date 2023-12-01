use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_first_and_last_numbers(line: &str) -> Result<i32, String> {
    // First get the first number moving forward
    let numbers: String = line.chars().filter(|c| c.is_digit(10)).collect();

    let first = numbers.chars().next().ok_or("Couldn't get first number")?;
    let last = numbers.chars().last().ok_or("Couldn't get last number")?;

    let num = format!("{}{}", first, last)
        .parse::<i32>()
        .map_err(|_| "Couldn't parse number")?;

    Ok(num)
}

fn main() -> Result<(), String> {
    let input = File::open("input").map_err(|_| "Couldn't read file")?;
    let reader = BufReader::new(input);

    println!("{:?}",
        reader
        .lines()
        .map(|line| get_first_and_last_numbers(&line.map_err(|_| "Reader sent a wrong line")?))
        .sum::<Result<i32, String>>());

    Ok(())
}
