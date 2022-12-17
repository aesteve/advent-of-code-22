mod part_1;
mod part_2;

use regex::Regex;
use std::collections::VecDeque;

trait Crates {
    fn move_crates(&mut self, qty: usize, from: usize, to: usize);
    fn collect_crates(&self) -> String;
    fn init(stacks: Vec<VecDeque<char>>) -> Self;
}

fn parse<C: Crates, L, I: IntoIterator<Item = L>>(file: I, mapping: fn(L) -> String) -> C {
    let mut drawing_lines = VecDeque::new();
    let mut iter = file.into_iter();
    for l in iter.by_ref() {
        let line = mapping(l);
        if line.is_empty() {
            break;
        }
        drawing_lines.push_front(line)
    }

    let mut lines = drawing_lines.iter();
    let stack_count = lines.next().unwrap().split(' ').last().unwrap();
    let stack_count = stack_count.parse::<usize>().unwrap();
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(stack_count);
    for line in lines {
        let mut chars = line.chars();
        let mut idx = 1;
        for i in 0..stack_count {
            let c = chars.nth(idx);
            if let Some(c) = c {
                if !c.is_ascii_whitespace() {
                    if let Some(q) = stacks.get_mut(i) {
                        q.push_back(c)
                    } else {
                        let mut q = VecDeque::new();
                        q.push_back(c);
                        stacks.insert(i, q)
                    }
                }
            }
            idx = 3;
        }
    }
    let mut crates = C::init(stacks);

    let line_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in iter {
        let line = mapping(line);
        let capts = line_regex.captures(line.as_str()).unwrap();
        crates.move_crates(
            capts[1].parse::<usize>().unwrap(),
            capts[2].parse::<usize>().unwrap(),
            capts[3].parse::<usize>().unwrap(),
        );
    }
    crates
}

#[cfg(test)]
mod tests {
    use crate::utils::{input_file_lines, FileLines};

    pub(crate) fn sample() -> Vec<&'static str> {
        vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", "1   2   3", ""]
    }

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_5.txt").unwrap()
    }
}
//
// 1;5;9
