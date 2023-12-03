use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Draw {
    red: usize,
    blue: usize,
    green: usize,
}

impl Draw {
    fn new(red: usize, blue: usize, green: usize) -> Draw {
        Draw { red, blue, green }
    }
}

#[derive(Debug)]
struct Game {
    number: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn init(string: &str) -> Result<Game, String> {
        Ok(Game {
            number: string
                .split(' ')
                .into_iter()
                .rev()
                .next()
                .ok_or("Unable to get Game number")?
                .parse::<usize>()
                .map_err(|_| "Unable to parse game number")?,
            draws: vec![],
        })
    }

    fn is_possible(&self, other: Draw) -> bool {
        self.draws.iter().all(|draw| {
            (draw.red <= other.red) && (draw.blue <= other.blue) && (draw.green <= other.green)
        })
    }

    fn power(&self) -> usize {
        let mut highest = Draw::new(0, 0, 0);
        self.draws.iter().for_each(|draw| {
            if highest.red < draw.red {
                highest.red = draw.red
            }
            if highest.blue < draw.blue {
                highest.blue = draw.blue
            }
            if highest.green < draw.green {
                highest.green = draw.green
            }
        });

        highest.red * highest.blue * highest.green
    }
}

fn parse_draw(draw: &str) -> Result<Draw, String> {
    let mut output = Draw::new(0, 0, 0);
    // println!("{}", draw);
    draw.split(", ").into_iter().for_each(|colorstr| {
        let mut numcolor_iter = colorstr.split_whitespace().into_iter();
        let num = numcolor_iter
            .next()
            .expect("Wrong number format")
            .parse::<usize>()
            .expect("Couldn't parse number");
        let color = numcolor_iter.next().expect("Wrong number format");
        // println!("{}: {},{}", colorstr, num, color);
        match color {
            "red" => output.red += num,
            "blue" => output.blue += num,
            "green" => output.green += num,
            c => panic!("Unknown color {}", c),
        };
    });

    Ok(output)
}

fn parse_line(line: &str) -> Result<Game, String> {
    let mut line_iter = line.split(':');
    let mut game = Game::init(line_iter.next().ok_or("Line could not be parsed")?)?;
    game.draws = line_iter
        .next()
        .ok_or("Unable to parse draws")?
        .split(";")
        .map(|draw| parse_draw(draw).expect("Unable to parse draws"))
        .collect();

    // println!("{:?}", game);

    Ok(game)
}

fn main() -> Result<(), String> {
    let input = File::open("input").map_err(|_| "Couldn't read file")?;
    let reader = BufReader::new(input);

    let games = reader
        .lines()
        .map(|line| parse_line(&line.expect("Could not read line")).expect("Could not parse game"))
        .collect::<Vec<Game>>();

    let total = games
        .iter()
        .filter(|game| {
            game.is_possible(Draw {
                red: 12,
                blue: 14,
                green: 13,
            })
        })
        .map(|game| game.number)
        .sum::<usize>();
    println!("The total is {}", total);

    let power: usize = games.iter().map(|game| game.power()).sum();

    println!("The total power is {}", power);

    Ok(())
}
