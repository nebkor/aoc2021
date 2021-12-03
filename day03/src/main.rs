use aocf::Aoc;

fn main() {
    p1();
    p2();
}

fn p2() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(3)).init().unwrap();
    let input = aoc.get_input(false).unwrap();
    let mut olines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    // for the co2 calculation
    let mut clines = olines.clone();

    // o2 calculation
    for n in 0..12 {
        if olines.len() < 2 {
            break;
        }

        let mut bal = 0;
        for line in olines.iter() {
            if line.chars().nth(n).unwrap() == '1' {
                bal += 1;
            } else {
                bal -= 1;
            }
        }
        if bal < 0 {
            olines.retain(|l| l.chars().nth(n).unwrap() == '0');
        } else {
            olines.retain(|l| l.chars().nth(n).unwrap() == '1');
        }
    }
    let o2s = olines[0];
    let o2v = s2n(o2s);
    dbg!(o2s, o2v);

    // co2 rating
    for n in 0..12 {
        if clines.len() < 2 {
            break;
        }
        let mut bal = 0;
        for line in clines.iter() {
            if line.chars().nth(n).unwrap() == '1' {
                bal += 1;
            } else {
                bal -= 1;
            }
        }

        if bal < 0 {
            clines.retain(|l| l.chars().nth(n).unwrap() == '1');
        } else {
            clines.retain(|l| l.chars().nth(n).unwrap() == '0');
        }
    }
    let co2s = clines[0];
    let co2v = s2n(co2s);
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

    let gamma = s2n(&bs);

    let mut eps = String::new();

    for c in bs.chars() {
        if c == '1' {
            eps.push('0');
        } else {
            eps.push('1');
        }
    }

    let epsilon = s2n(&eps);

    // part 1
    dbg!(gamma * epsilon);
}

fn s2n(s: &str) -> isize {
    isize::from_str_radix(s, 2).unwrap()
}
