#[cfg(test)]
mod tests {
    use crate::day_10::tests::puzzle_input;
    use crate::day_10::{CpuState, Instruction};

    #[test]
    fn solution() {
        let mut state = CpuState::default();
        let lines = puzzle_input();
        let mut sum = 0;
        for line in lines {
            let instruction = Instruction::from(line.unwrap().as_str());
            if let Some((cycle, res)) = state.exec(&instruction) {
                sum += res * cycle as i64;
            }
        }
        assert_eq!(15260, sum);
    }
}
