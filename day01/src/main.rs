use aocf::Aoc;
use itertools::Itertools;

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(1)).init().unwrap();

    let input = aoc.get_input(false).unwrap();

    let mut input = input
        .split('\n')
        .map(|d| d.parse::<i32>().unwrap_or(std::i32::MIN));
    let mut pd = input.next().unwrap();
    let mut ans = 0;
    for d in input {
        if d > pd {
            ans += 1;
        }
        pd = d;
    }

    dbg!(ans);
}

fn p2() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(1)).init().unwrap();

    let input = aoc.get_input(false).unwrap();

    let input = input
        .split('\n')
        .map(|d| d.parse::<i32>().unwrap_or(std::i32::MIN));

    let mut pd = std::i32::MAX;
    let mut ans = 0;
    for (x, y, z) in input.tuple_windows() {
        let d = x + y + z;
        if d > pd {
            ans += 1;
        }
        pd = d;
    }

    dbg!(ans);
}
