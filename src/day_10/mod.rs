mod part_1;

#[derive(Debug)]
pub(crate) struct CpuState {
    cycle: u64,
    register: i64,
}

#[derive(Debug)]
pub(crate) enum Instruction {
    Noop,
    Addx(i64),
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            cycle: 0,
            register: 1,
        }
    }
}

impl CpuState {
    pub(crate) fn exec(&mut self, instruction: &Instruction) -> Option<(u64, i64)> {
        let before_value = self.register;
        let passed_threshold = match instruction {
            Instruction::Noop => self.cycle_through(),
            Instruction::Addx(value) => {
                let passed = self.cycle_through();
                self.cycle_through();
                self.register += *value;
                passed
            }
        };
        if let Some(passed) = passed_threshold {
            Some((passed, before_value))
        } else if let Some(threshold) = self.at_threshold() {
            Some((threshold, self.register))
        } else {
            None
        }
    }

    fn cycle_through(&mut self) -> Option<u64> {
        self.cycle += 1;
        self.at_threshold()
    }

    fn at_threshold(&self) -> Option<u64> {
        if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
            Some(self.cycle)
        } else {
            None
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "noop" => Instruction::Noop,
            _ => {
                let mut splitted = value.split(' ');
                splitted.next();
                let qty = splitted.next().unwrap();
                Instruction::Addx(qty.parse().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_10::{CpuState, Instruction};
    use crate::utils::{input_file_lines, FileLines};

    pub(crate) fn sample() -> Vec<Instruction> {
        vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ]
        .into_iter()
        .map(Instruction::from)
        .collect()
    }

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_10.txt").unwrap()
    }

    #[test]
    fn check_sample() {
        let mut state = CpuState::default();
        let mut sum = 0;
        for instruction in sample() {
            if let Some((cycle, res)) = state.exec(&instruction) {
                sum += res * cycle as i64;
            }
        }
        assert_eq!(13360, sum);
    }
}
