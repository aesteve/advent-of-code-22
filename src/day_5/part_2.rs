use crate::day_5::Crates;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct CratesV2 {
    stacks: Vec<VecDeque<char>>,
}

impl Crates for CratesV2 {
    fn move_crates(&mut self, qty: usize, from: usize, to: usize) {
        // This time, we can't for_each(pop)
        let mut temp = VecDeque::new();
        if let Some(source) = self.stacks.get_mut(from - 1) {
            for _ in 0..qty {
                if let Some(item) = source.pop_back() {
                    temp.push_front(item)
                }
            }
        }
        if let Some(dest) = self.stacks.get_mut(to - 1) {
            for item in temp {
                dest.push_back(item)
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
        CratesV2 { stacks }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::part_2::CratesV2;
    use crate::day_5::tests::{puzzle_input, sample};
    use crate::day_5::{parse, Crates};
    use std::collections::VecDeque;

    #[test]
    fn move_1_doesnt_change() {
        let mut sample = sample();
        sample.push("move 1 from 2 to 1");
        let crates = parse(sample, str::to_string);
        let expected = CratesV2 {
            stacks: vec![
                VecDeque::from(['Z', 'N', 'D']),
                VecDeque::from(['M', 'C']),
                VecDeque::from(['P']),
            ],
        };
        assert_eq!(expected, crates)
    }

    #[test]
    fn move_2_changes_crates() {
        let mut sample = sample();
        sample.push("move 1 from 2 to 1");
        sample.push("move 3 from 1 to 3");
        let crates = parse(sample, str::to_string);
        let expected = CratesV2 {
            stacks: vec![
                VecDeque::from([]),
                VecDeque::from(['M', 'C']),
                VecDeque::from(['P', 'Z', 'N', 'D']),
            ],
        };
        assert_eq!(expected, crates)
    }

    #[test]
    fn move_3_changes_crates() {
        let input = puzzle_input();
        let crates: CratesV2 = parse(input, Result::unwrap);
        assert_eq!("VLCWHTDSZ".to_string(), crates.collect_crates())
    }
}
