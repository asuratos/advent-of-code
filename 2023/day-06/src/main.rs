fn distance_travelled(hold: &i32, racetime: &i32) -> i32 {
    hold * (racetime - hold)
}

fn count_wins(racetime: i32, record: i32) -> i32 {
    (0..racetime as usize)
        .filter(|&hold| distance_travelled(&(hold as i32), &racetime) > record)
        .count() as i32
}

fn main() {
    const GAMES: [(i32, i32); 4] = [(58, 434), (81, 1041), (96, 2219), (76, 1218)];

    let product = GAMES
        .iter()
        .map(|&(racetime, record)| count_wins(racetime, record))
        .product::<i32>();

    println!("{}", product);
}
