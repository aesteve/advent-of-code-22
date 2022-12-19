use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl Direction {
    fn rx_cx(self) -> (i32, i32) {
        match self {
            Direction::L(_) => (0, -1),
            Direction::R(_) => (0, 1),
            Direction::U(_) => (1, 0),
            Direction::D(_) => (-1, 0),
        }
    }

    fn steps(self) -> i32 {
        match self {
            Direction::L(steps) => steps,
            Direction::R(steps) => steps,
            Direction::U(steps) => steps,
            Direction::D(steps) => steps,
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s[2..]
            .parse()
            .map_err(|_| format!("cannot parse steps: {}", s))?;

        match &s[..1] {
            "L" => Ok(Direction::L(steps)),
            "R" => Ok(Direction::R(steps)),
            "U" => Ok(Direction::U(steps)),
            "D" => Ok(Direction::D(steps)),
            _ => Err(format!("invalid direction: {}", s)),
        }
    }
}

pub fn part_one(input: &[Direction]) -> usize {
    simulate::<2>(input)
}

pub fn part_two(input: &[Direction]) -> usize {
    simulate::<10>(input)
}

fn simulate<const N: usize>(input: &[Direction]) -> usize {
    let mut rope = [(0, 0); N];

    let mut visited = HashSet::default();
    visited.insert((0, 0));

    for direction in input.iter().copied() {
        let (rx, cx) = direction.rx_cx();
        for _ in 0..direction.steps() {
            rope[0].0 += rx;
            rope[0].1 += cx;
            move_rope(&mut rope, &mut visited);
        }
    }

    visited.len()
}

fn move_rope<const N: usize>(rope: &mut [(i32, i32); N], visited: &mut HashSet<(i32, i32)>) {
    let old_tail = rope[N - 1];

    for h in 0..N - 1 {
        let (hr, hc) = rope[h + 0];
        let (tr, tc) = rope[h + 1];

        let (rx, cx) = move_tail(hr, hc, tr, tc);
        if (rx, cx) == (tr, tc) {
            break;
        }

        rope[h + 1] = (rx, cx);
    }

    if old_tail != rope[N - 1] {
        visited.insert(rope[N - 1]);
    }
}

fn move_tail(hr: i32, hc: i32, mut tr: i32, mut tc: i32) -> (i32, i32) {
    if hc.abs_diff(tc) > 1 || hr.abs_diff(tr) > 1 {
        tr += (hr - tr).signum();
        tc += (hc - tc).signum();
    }

    (tr, tc)
}

#[cfg(test)]
mod tests {
    use crate::day_9::soluce::part_one;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use std::path::Path;
    use std::str::FromStr;

    pub fn parse_line_delimited<I, R>(input: I) -> Vec<R>
    where
        I: AsRef<str>,
        R: FromStr,
        <R as FromStr>::Err: Debug,
    {
        input
            .as_ref()
            .lines()
            .map(|l| l.trim())
            .filter(|&l| !l.is_empty())
            .map(|l| l.parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }

    pub fn load_text_input_from_autodetect() -> String {
        load_text_input(auto_select_input())
    }

    pub fn load_line_delimited_input_from_autodetect<O: FromStr<Err = impl Debug>>() -> Vec<O> {
        parse_line_delimited(load_text_input_from_autodetect())
    }

    pub fn load_line_delimited_input_from_file<O: FromStr<Err = impl Debug>, P>(path: P) -> Vec<O>
    where
        P: AsRef<Path>,
    {
        let input = load_text_input_from_file(path);
        parse_line_delimited(input)
    }

    pub fn load_text_input_from_stdin() -> String {
        load_text_input(std::io::stdin().lock())
    }

    pub fn load_text_input_from_file<P: AsRef<Path>>(path: P) -> String {
        load_text_input(File::open(path).unwrap())
    }

    pub fn auto_select_input() -> Box<dyn BufRead> {
        match std::env::args().skip(1).next() {
            None => Box::new(BufReader::new(std::io::stdin())),
            Some(path) => Box::new(BufReader::new(
                File::open(&path).expect(&format!("file path: {}", &path)),
            )),
        }
    }

    pub fn load_text_input<R: Read>(mut input: R) -> String {
        let mut buffer = String::new();
        input.read_to_string(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("website_inputs/day_9.txt");
        let answer = part_one(&input);

        assert_eq!(6197, answer);
    }
}
