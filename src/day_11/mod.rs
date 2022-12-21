use std::cmp;
use std::collections::VecDeque;

mod parse;
mod part_1;
mod part_2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Monkey {
    pub(crate) identifier: u64,
    pub(crate) items: Vec<u64>,
    pub(crate) items_inspected: u64,
    pub(crate) operation: Operation,
    pub(crate) test_divisible_by: u64,
    pub(crate) monkey_if_true: u64,
    pub(crate) monkey_if_false: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Op {
    Mul,
    Add,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Term {
    Constant(u64),
    Old,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Operation {
    lhs: Term,
    op: Op,
    rhs: Term,
}

impl Term {
    fn value(&self, old: u64) -> u64 {
        match self {
            Term::Constant(value) => *value,
            Term::Old => old,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Thrown {
    item: u64,
    to: usize,
}

impl Monkey {
    // What makes a problem extremely hard in Rust is that we can't mutate an iterator while trying to index it
    // so we need to reset self.items after having called this method
    pub(crate) fn eval_round(&self, worry_concern: &impl Fn(u64) -> u64) -> Vec<Thrown> {
        let mut res = vec![];
        for item in &self.items {
            res.push(self.inspect(item, worry_concern));
        }
        res
    }

    pub(crate) fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn inspect(&self, item: &u64, worry_concern: &impl Fn(u64) -> u64) -> Thrown {
        let mut new_value = self.exec_op(*item);
        new_value = worry_concern(new_value);
        if self.test_outcome(new_value) {
            Thrown {
                item: new_value,
                to: self.monkey_if_true as usize,
            }
        } else {
            Thrown {
                item: new_value,
                to: self.monkey_if_false as usize,
            }
        }
    }

    fn exec_op(&self, item: u64) -> u64 {
        let lhs = self.operation.lhs.value(item);
        let rhs = self.operation.rhs.value(item);
        match self.operation.op {
            Op::Mul => lhs * rhs,
            Op::Add => lhs + rhs,
        }
    }

    fn test_outcome(&self, value: u64) -> bool {
        value % self.test_divisible_by == 0
    }
}

pub(crate) fn exec_round(monkeys: &mut Vec<Monkey>, worry_concern: &impl Fn(u64) -> u64) {
    let len = monkeys.len();
    for i in 0..len {
        let monkey = monkeys.get(i).unwrap();
        let thrown = monkey.eval_round(worry_concern);
        for t in thrown {
            monkeys.get_mut(t.to).unwrap().add_item(t.item)
        }
        let mut monkey = monkeys.get_mut(i).unwrap();
        monkey.items_inspected += monkey.items.len() as u64;
        monkey.items.clear();
    }
}

pub(crate) fn exec_rounds(
    rounds: usize,
    monkeys: &mut Vec<Monkey>,
    worry_concern: &impl Fn(u64) -> u64,
) -> u64 {
    for _ in 0..rounds {
        exec_round(monkeys, &worry_concern);
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
    use crate::day_11::parse::parse_all_monkeys_def;
    use crate::day_11::Monkey;
    use crate::utils::io::input_string;

    pub(crate) const SAMPLE_DEF: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1\n";

    pub(crate) fn sample_monkeys() -> Vec<Monkey> {
        parse_all_monkeys_def(SAMPLE_DEF).unwrap().1
    }

    fn puzzle_input() -> String {
        input_string("day_11.txt")
    }

    pub(crate) fn puzzle_monkeys() -> Vec<Monkey> {
        let str = puzzle_input();
        parse_all_monkeys_def(str.as_str()).unwrap().1
    }
}
