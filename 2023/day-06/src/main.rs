fn distance_travelled(hold: &i64, racetime: &i64) -> i64{
    hold * (racetime - hold)
}

fn count_wins(racetime: i64, record: i64) -> i64 {
    (0..racetime as usize)
        .filter(|&hold| distance_travelled(&(hold as i64), &racetime) > record)
        .count() as i64
}

fn main() {
    const GAMES: [(i64, i64); 4] = [(58, 434), (81, 1041), (96, 2219), (76, 1218)];

    let product = GAMES
        .iter()
        .map(|&(racetime, record)| count_wins(racetime, record))
        .product::<i64>();

    println!("{}", product);

    println!("{}", count_wins( 58819676, 434104122191218));
}
