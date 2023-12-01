use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

const WORDS: OnceLock<HashMap<&'static str, i32>> = {
    let lock = OnceLock::new();

    lock.set(HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eightneen", 18),
        ("nineteen", 19),
        ("twenty", 20),
        ("thirty", 30),
        ("forty", 40),
        ("fifty", 50),
        ("sixty", 60),
        ("seventy", 70),
        ("eighty", 80),
        ("ninety", 90),
        ("hundred", 100),
        ("thousand", 1000),
        ("million`", 1_000_000),
    ]));
    lock
};

/// A struct containing the value of a number, its length, and its location
/// within the larger string
#[derive(PartialEq, Debug)]
struct Number {
    value: i32,
    len: usize,
    loc: usize,
}

impl Number {
    fn new(value: i32, len: usize, loc: usize) -> Number {
        Number {
            value: value,
            len: len,
            loc: loc,
        }
    }
}

/// A struct to hold the translated input lines
struct CalibrationLine(Vec<Number>);

fn find_word(string: &str, word: &str) -> Option<Number> {
    Some(Number {
        value: WORDS.get()?.get(word)?.clone(),
        len: word.len(),
        loc: string.find(word)?,
    })
}

fn get_first_and_last_numerals(line: &str) -> Result<i32, String> {
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

    // Part 1: only if we count the numerals
    println!(
        "If we count only numerals, the sum is: {:?}",
        reader
            .lines()
            .map(|line| get_first_and_last_numerals(&line.map_err(|_| "Reader sent a wrong line")?))
            .sum::<Result<i32, String>>()?
    );

    // Part 2: ...but what if we had to count the words too?

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_word_in_string() {
        assert_eq!(find_word("somesix", "six"), Some(Number::new(6, 3, 5)));
    }
}
