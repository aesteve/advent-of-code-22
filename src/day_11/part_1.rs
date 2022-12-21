#[cfg(test)]
mod tests {
    use crate::day_11::tests::{puzzle_monkeys, sample_monkeys};
    use crate::day_11::{exec_rounds, Monkey, Op, Operation, Term, Thrown};

    fn part_1_worry_concern(i: u64) -> u64 {
        i / 3
    }

    #[test]
    fn sample_round() {
        let monkeys = sample_monkeys();
        let monkey = monkeys.into_iter().next().unwrap();
        let thrown = monkey.eval_round(&part_1_worry_concern);
        assert_eq!(
            thrown,
            vec![Thrown { item: 500, to: 3 }, Thrown { item: 620, to: 3 }]
        )
    }

    #[test]
    fn sample_1_round() {
        let mut monkeys = sample_monkeys();
        let _ = exec_rounds(1, &mut monkeys, &part_1_worry_concern);
        assert_eq!(
            monkeys.get(0).unwrap(),
            &Monkey {
                identifier: 0,
                items: vec![20, 23, 27, 26],
                items_inspected: 2,
                operation: Operation {
                    lhs: Term::Old,
                    op: Op::Mul,
                    rhs: Term::Constant(19),
                },
                test_divisible_by: 23,
                monkey_if_true: 2,
                monkey_if_false: 3,
            }
        );
        assert_eq!(
            monkeys.get(1).unwrap(),
            &Monkey {
                identifier: 1,
                items: vec![2080, 25, 167, 207, 401, 1046],
                items_inspected: 4,
                operation: Operation {
                    lhs: Term::Old,
                    op: Op::Add,
                    rhs: Term::Constant(6),
                },
                test_divisible_by: 19,
                monkey_if_true: 2,
                monkey_if_false: 0,
            }
        );
    }

    #[test]
    fn solution() {
        let mut monkeys = puzzle_monkeys();
        let res = exec_rounds(20, &mut monkeys, &part_1_worry_concern);
        assert_eq!(56120, res);
    }
}
