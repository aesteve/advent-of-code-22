use crate::day_11::Monkey;
use std::cmp;
use std::sync::Mutex;

fn exec_round(monkeys: &mut Vec<Monkey>) {
    let len = monkeys.len();
    for i in 0..len {
        let monkey = monkeys.get(i).unwrap();
        let thrown = monkey.eval_round();
        for t in thrown {
            monkeys.get_mut(t.to).unwrap().add_item(t.item)
        }
        let mut monkey = monkeys.get_mut(i).unwrap();
        monkey.items_inspected += monkey.items.len() as u64;
        monkey.items.clear();
    }
}

fn exec_rounds(rounds: usize, monkeys: &mut Vec<Monkey>) -> u64 {
    for _ in 0..rounds {
        exec_round(monkeys);
    }
    let mut items = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<_>>();
    items.sort_by_key(|&c| cmp::Reverse(c));
    items.iter().take(2).product()
}

#[cfg(test)]
mod tests {
    use crate::day_11::part_1::exec_rounds;
    use crate::day_11::tests::{puzzle_monkeys, sample_monkeys};
    use crate::day_11::{Monkey, Op, Operation, Term, Thrown};

    #[test]
    fn sample_round() {
        let monkeys = sample_monkeys();
        let monkey = monkeys.into_iter().next().unwrap();
        let thrown = monkey.eval_round();
        assert_eq!(
            thrown,
            vec![Thrown { item: 500, to: 3 }, Thrown { item: 620, to: 3 }]
        )
    }

    #[test]
    fn sample_1_round() {
        let mut monkeys = sample_monkeys();
        let _ = exec_rounds(1, &mut monkeys);
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
        let res = exec_rounds(20, &mut monkeys);
        assert_eq!(10, res);
    }
}
