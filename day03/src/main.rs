use aocf::Aoc;

fn main() {
    //p1();
    p2();
}

fn p2() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(3)).init().unwrap();

    let input = aoc.get_input(false).unwrap().trim().to_string();

    let mut o2s = String::new();
    for n in 0..12 {
        let mut maj = 0;
        for line in input.lines() {
            if line.chars().nth(n).unwrap() == '1' {
                maj += 1;
            } else {
                maj -= 1;
            }
        }
        if maj >= 0 {
            o2s.push('1');
        } else {
            o2s.push('0');
        }
    }

    let o2v = isize::from_str_radix(&o2s, 2).unwrap();
    dbg!(&o2s, o2v);

    let mut co2s = String::new();
    for n in 0..12 {
        let mut maj = 0;
        for line in input.lines() {
            if line.chars().nth(n).unwrap() == '1' {
                maj -= 1;
            } else {
                maj += 1;
            }
        }
        if maj <= 0 {
            co2s.push('0');
        } else {
            co2s.push('1');
        }
    }

    let co2v = isize::from_str_radix(&co2s, 2).unwrap();

    dbg!(co2v, co2v * o2v);

    //
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
