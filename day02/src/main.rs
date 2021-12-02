use aocf::Aoc;

#[derive(Debug)]
struct SPath {
    pub h: Vec<i32>,
    pub v: Vec<i32>,
    aim: i32,
}

impl SPath {
    pub fn fs(s: &str) -> Self {
        let mut h = Vec::new();
        let mut v = Vec::new();
        let mut aim = 0;

        for l in s.split('\n') {
            match l.split(' ').collect::<Vec<_>>().as_slice() {
                ["forward", x] => {
                    let x = x.parse::<i32>().unwrap();
                    h.push(x);
                    let d = aim * x;
                    v.push(d);
                }
                ["up", x] => {
                    aim -= x.parse::<i32>().unwrap();
                }
                ["down", x] => {
                    aim += x.parse::<i32>().unwrap();
                }
                _ => {}
            }
        }

        Self { h, v, aim }
    }
}

fn main() {
    p1();
}

fn p1() {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(2)).init().unwrap();

    let input = aoc.get_input(false).unwrap();

    let s = SPath::fs(&input);

    let ans: i32 = s.h.iter().sum::<i32>() * s.v.iter().sum::<i32>();

    dbg!(ans);
}
