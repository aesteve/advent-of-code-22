use crate::day_11::{Monkey, Op, Operation, Term};
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::{line_ending, multispace0, u64 as u64p};
use nom::combinator::{map, value};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

fn parse_term(def: &str) -> IResult<&str, Term> {
    let parse_old = value(Term::Old, tag("old"));
    let parse_constant = map(u64p, Term::Constant);
    alt((parse_old, parse_constant))(def)
}

fn parse_op(def: &str) -> IResult<&str, Op> {
    let parse_mul = value(Op::Mul, tag(" * "));
    let parse_add = value(Op::Add, tag(" + "));
    alt((parse_add, parse_mul))(def)
}

fn parse_operation(def: &str) -> IResult<&str, Operation> {
    let (rest, (_, lhs, op, rhs)) =
        tuple((tag("Operation: new = "), parse_term, parse_op, parse_term))(def)?;
    Ok((rest, Operation { lhs, op, rhs }))
}

fn parse_items(def: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(", "), u64p)(def)
}

pub(crate) fn parse_monkey_def(def: &str) -> IResult<&str, Monkey> {
    let (rest, (_, identifier, _, _)) = tuple((tag("Monkey "), u64p, tag(":"), line_ending))(def)?;
    let (rest, (_, _, items, _)) = tuple((
        multispace0,
        tag("Starting items: "),
        parse_items,
        line_ending,
    ))(rest)?;
    let (rest, (_, operation, _)) = tuple((multispace0, parse_operation, line_ending))(rest)?;
    let (rest, (_, _, test_divisible_by, _)) =
        tuple((multispace0, tag("Test: divisible by "), u64p, line_ending))(rest)?;
    let (rest, (_, _, monkey_if_true, _)) = tuple((
        multispace0,
        tag("If true: throw to monkey "),
        u64p,
        line_ending,
    ))(rest)?;
    let (rest, (_, _, monkey_if_false, _)) = tuple((
        multispace0,
        tag("If false: throw to monkey "),
        u64p,
        line_ending,
    ))(rest)?;
    Ok((
        rest,
        Monkey {
            identifier,
            items,
            items_inspected: 0,
            operation,
            test_divisible_by,
            monkey_if_true,
            monkey_if_false,
        },
    ))
}

pub(crate) fn parse_all_monkeys_def(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(line_ending, parse_monkey_def)(input)
}

#[cfg(test)]
mod tests {
    use crate::day_11::parse::{parse_monkey_def, parse_operation};
    use crate::day_11::tests::{puzzle_monkeys, sample_monkeys};
    use crate::day_11::{Op, Operation, Term};

    #[test]
    fn can_parse_operation() {
        let sample = "Operation: new = old * 19\n";
        let res = parse_operation(sample);
        assert!(res.is_ok());
        let (_, op) = res.unwrap();
        assert_eq!(
            Operation {
                lhs: Term::Old,
                op: Op::Mul,
                rhs: Term::Constant(19),
            },
            op
        )
    }

    #[test]
    fn can_parse_monkey() {
        let monkey_def = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3\n";
        let res = parse_monkey_def(monkey_def);
        res.unwrap();
    }

    #[test]
    fn can_parse_sample() {
        let monkeys = sample_monkeys();
        assert_eq!(4, monkeys.len())
    }

    #[test]
    fn can_parse_puzzle_input() {
        let monkeys = puzzle_monkeys();
        assert_eq!(8, monkeys.len());
    }
}
