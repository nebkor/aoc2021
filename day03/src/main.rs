use aocf::Aoc;

fn main() {
    p1();
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

    dbg!(gamma * epsilon);
}
