use crate::day_5::Crates;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct CratesV1 {
    stacks: Vec<VecDeque<char>>,
}

impl Crates for CratesV1 {
    fn move_crates(&mut self, qty: usize, from: usize, to: usize) {
        for _ in 0..qty {
            if let Some(item) = self.stacks.get_mut(from - 1).and_then(|s| s.pop_back()) {
                if let Some(dest) = self.stacks.get_mut(to - 1) {
                    dest.push_back(item)
                }
            }
        }
    }
    fn collect_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }

    fn init(stacks: Vec<VecDeque<char>>) -> Self {
        CratesV1 { stacks }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::part_1::CratesV1;
    use crate::day_5::tests::{puzzle_input, sample};
    use crate::day_5::{parse, Crates};
    use std::collections::VecDeque;

    #[test]
    fn sample_parsing() {
        let sample = sample();
        let crates = parse(sample, str::to_string);
        let expected = CratesV1 {
            stacks: vec![
                VecDeque::from(['Z', 'N']),
                VecDeque::from(['M', 'C', 'D']),
                VecDeque::from(['P']),
            ],
        };
        assert_eq!(expected, crates)
    }

    #[test]
    fn sample_move_1() {
        let mut sample = sample();
        sample.push("move 1 from 2 to 1");
        let crates = parse(sample, str::to_string);
        let expected = CratesV1 {
            stacks: vec![
                VecDeque::from(['Z', 'N', 'D']),
                VecDeque::from(['M', 'C']),
                VecDeque::from(['P']),
            ],
        };
        assert_eq!(expected, crates)
    }

    #[test]
    fn sample_move_2() {
        let mut sample = sample();
        sample.push("move 1 from 2 to 1");
        sample.push("move 3 from 1 to 3");
        let crates = parse(sample, str::to_string);
        let expected = CratesV1 {
            stacks: vec![
                VecDeque::from([]),
                VecDeque::from(['M', 'C']),
                VecDeque::from(['P', 'D', 'N', 'Z']),
            ],
        };
        assert_eq!(expected, crates)
    }

    #[test]
    fn solution() {
        let input = puzzle_input();
        let crates: CratesV1 = parse(input, Result::unwrap);
        assert_eq!("TBVFVDZPN".to_string(), crates.collect_crates())
    }
}
