use crate::day_11::Monkey;

// The tip here is that we don't need to work with the level, but only the value divided by the common divisor to every monkey operation
fn worry_level_reducer(monkeys: &Vec<Monkey>) -> impl Fn(u64) -> u64 {
    let pgcm: u64 = monkeys.iter().map(|m| m.test_divisible_by).product();
    move |i: u64| -> u64 { i % pgcm }
}

#[cfg(test)]
mod tests {
    use crate::day_11::exec_rounds;
    use crate::day_11::part_2::worry_level_reducer;
    use crate::day_11::tests::{puzzle_monkeys, sample_monkeys};

    #[test]
    fn check_sample() {
        let mut monkeys = sample_monkeys();
        let reducer = worry_level_reducer(&monkeys);
        let res = exec_rounds(10_000, &mut monkeys, &reducer);
        assert_eq!(2713310158, res);
    }

    #[test]
    fn solution() {
        let mut monkeys = puzzle_monkeys();
        let reducer = worry_level_reducer(&monkeys);
        let res = exec_rounds(10_000, &mut monkeys, &reducer);
        assert_eq!(24389045529, res);
    }
}
