use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

static WORDS: OnceLock<HashMap<&str, u32>> = OnceLock::new();

fn init_words() -> () {
    let _ = WORDS.set(HashMap::from([
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
}

/// A struct containing the value of a number, its length, and its location
/// within the larger string
#[derive(Eq, PartialEq, Debug)]
struct Number {
    value: u32,
    len: usize,
    loc: usize,
}

impl Number {
    fn new(value: u32, len: usize, loc: usize) -> Number {
        Number {
            value: value,
            len: len,
            loc: loc,
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.loc == other.loc {
            return Some(other.len.cmp(&self.len));
        }

        Some(self.loc.cmp(&other.loc))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.loc == other.loc {
            return other.len.cmp(&self.len);
        }
        self.loc.cmp(&other.loc)
    }
}

/// A struct to hold the translated input lines
#[derive(Debug, PartialEq)]
struct NumberLine(Vec<Number>);

fn find_word(string: &str, word: &str) -> Option<Vec<Number>> {
    let indices: Vec<usize> = string
        .match_indices(word)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    Some(
        indices
            .into_iter()
            .filter_map(|i| {
                Some(Number {
                    value: WORDS.get()?.get(word)?.clone(),
                    len: word.len(),
                    loc: i,
                })
            })
            .collect(),
    )
}

fn translate_line(line: &str) -> Option<Vec<Number>> {
    // Get vector for words
    let mut found_words: Vec<Number> = WORDS
        .get()?
        .keys()
        .filter_map(|word| find_word(line, word))
        .flatten()
        .collect();

    // get vector for numbers
    let mut found_numbers: Vec<Number> = line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .filter_map(|(i, n)| {
            Some(Number {
                value: n.to_digit(10)?,
                loc: i,
                len: 1,
            })
        })
        .collect();

    found_words.append(&mut found_numbers);
    found_words.sort();
    Some(found_words)
}

fn get_first_and_last_numerals(line: &str) -> Result<u32, String> {
    // First get the first number moving forward
    let numbers: String = line.chars().filter(|c| c.is_digit(10)).collect();

    let first = numbers.chars().next().ok_or("Couldn't get first number")?;
    let last = numbers.chars().last().ok_or("Couldn't get last number")?;

    let num = format!("{}{}", first, last)
        .parse::<u32>()
        .map_err(|_| "Couldn't parse number")?;

    Ok(num)
}

fn main() -> Result<(), String> {
    init_words();
    // println!("{:?}", WORDS.get());

    let input = File::open("input").map_err(|_| "Couldn't read file")?;
    let reader = BufReader::new(input);

    // Part 1: only if we count the numerals
    // println!(
    //     "If we count only numerals, the sum is: {:?}",
    //     reader
    //         .lines()
    //         .map(|line| get_first_and_last_numerals(&line.map_err(|_| "Reader sent a wrong line")?))
    //         .sum::<Result<u32, String>>()?
    // );

    // Part 2: ...but what if we had to count the words too?
    for line in reader.lines() {
        println!("{:?}", find_word(&line.map_err(|_| "line")?, "eight"))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_word_in_string() {
        init_words();
        assert_eq!(
            find_word("somesix", "six"),
            Some(vec![Number::new(6, 3, 4)])
        );
    }

    #[test]
    fn find_same_word_in_string() {
        init_words();
        assert_eq!(
            find_word("sixsix", "six"),
            Some(vec![Number::new(6, 3, 0), Number::new(6, 3, 3)])
        );
    }
}
