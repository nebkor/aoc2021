use std::process::exit;

use aocf::Aoc;

fn main() {
    //p1();
    p2();
}

fn p2() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(3)).init().unwrap();

    let input = aoc.get_input(false).unwrap();

    let mut lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    for n in 0..12 {
        let mut maj = 0;
        for line in lines.iter().filter(|l| !l.is_empty()) {
            if line.chars().nth(n).unwrap() == '1' {
                maj += 1;
            } else {
                maj -= 1;
            }
        }
        if maj < 0 {
            lines.retain(|l| l.chars().nth(n).unwrap() == '0');
        } else {
            lines.retain(|l| l.chars().nth(n).unwrap() == '1');
        }
    }

    dbg!(&lines);
    let o2s = lines[0];
    let o2v = isize::from_str_radix(o2s, 2).unwrap();
    dbg!(o2s, o2v);

    //exit(0);

    let mut lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    for n in 0..12 {
        if lines.len() < 2 {
            break;
        }
        let mut maj = 0;
        for line in lines.iter().filter(|l| !l.is_empty()) {
            if line.chars().nth(n).unwrap() == '1' {
                maj += 1;
            } else {
                maj -= 1;
            }
        }
        dbg!(n, maj, &lines);
        if maj >= 0 {
            lines.retain(|l| l.chars().nth(n).unwrap() == '0');
        } else {
            lines.retain(|l| l.chars().nth(n).unwrap() == '1');
        }
    }

    dbg!(&lines);
    let co2s = lines[0];
    let co2v = isize::from_str_radix(co2s, 2).unwrap();
    dbg!(co2v, o2v, co2v * o2v);
}

fn p1() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(3)).init().unwrap();

    let input = aoc.get_input(false).unwrap();

    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let mut gammas = Vec::with_capacity(first.len());

    for c in first.chars() {
        match c {
            '1' => gammas.push(1),
            _ => gammas.push(-1),
        }
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => gammas[i] += 1,
                _ => gammas[i] -= 1,
            }
        }
    }

    let mut bs: String = String::new();

    for n in gammas {
        if n > 0 {
            bs.push('1');
        } else if n < 0 {
            bs.push('0');
        } else {
            println!("shit, I got a zero: {}", &bs);
        }
    }

    let gamma = isize::from_str_radix(&bs, 2).unwrap();

    let mut eps = String::new();

    for c in bs.chars() {
        if c == '1' {
            eps.push('0');
        } else {
            eps.push('1');
        }
    }

    let epsilon = isize::from_str_radix(&eps, 2).unwrap();

    // part 1
    dbg!(gamma * epsilon);

    let mut input = input.clone();
    for (i, n) in bs.chars().enumerate() {
        dbg!(&input, i, n, &bs);
        let v = input
            .lines()
            .filter(|l| l.chars().nth(i).unwrap() == n)
            .collect::<Vec<_>>();
        input = v.join("\n");
    }

    //assert_eq!(input.len(), 12;
    let o2_level = isize::from_str_radix(&input, 2).unwrap();
    dbg!(o2_level);
}
