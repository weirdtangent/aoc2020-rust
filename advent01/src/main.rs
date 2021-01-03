use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let (a, b) = include_str!("input.txt")
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .expect("no pair had a sum of 2020");

    println!("Part 1:");
    dbg!(a + b);
    dbg!(a * b);

    let (a, b, c) = include_str!("input.txt")
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .expect("no tuple of length 3 had a sum of 2020");

    println!("Part 2:");
    dbg!(a + b + c);
    dbg!(a * b * c);

    Ok(())
}
