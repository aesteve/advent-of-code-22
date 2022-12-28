mod part_1;
mod soluce_serde;

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Item {
    Int(u64),
    List(Vec<Item>),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Packet {
    inner: VecDeque<Item>,
}

impl Packet {
    fn pop(&mut self) -> Option<Item> {
        self.inner.pop_front()
    }

    fn back(&self) -> Option<&Item> {
        self.inner.back()
    }

    fn push(&mut self, item: Item) {
        self.inner.push_back(item)
    }
}

impl From<Vec<Item>> for Packet {
    fn from(value: Vec<Item>) -> Self {
        Packet {
            inner: VecDeque::from(value),
        }
    }
}

pub(crate) fn is_ordered(lhs: &mut Packet, rhs: &mut Packet) -> bool {
    match (lhs.pop(), rhs.pop()) {
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (None, None) => true,
        (Some(Item::Int(left)), Some(Item::Int(right))) => match left.cmp(&right) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => is_ordered(lhs, rhs),
        },
        (Some(Item::List(l)), Some(Item::Int(i))) => {
            println!("list compare int");
            lhs.push(Item::List(l));
            let mut rhs = rhs.clone();
            rhs.push(Item::List(vec![Item::Int(i)]));
            is_ordered(lhs, &mut rhs)
        }
        (Some(Item::Int(i)), Some(Item::List(l))) => {
            let mut lhs = lhs.clone();
            lhs.push(Item::List(vec![Item::Int(i)]));
            rhs.push(Item::List(l));
            is_ordered(&mut lhs, rhs)
        }
        (Some(Item::List(list_left)), Some(Item::List(list_right))) => {
            println!("left: {list_left:?}");
            println!("right: {list_right:?}");
            let mut new_lhs = list_left.into();
            let mut new_rhs = list_right.into();
            is_ordered(&mut new_lhs, &mut new_rhs) && is_ordered(lhs, rhs)
        }
    }
}

fn parse_list(chars: &mut Chars) -> Vec<Item> {
    let d = chars.clone().collect::<String>();
    let mut items = Vec::new();
    while let Some(c) = chars.next() {
        if c == ']' {
            break;
        }
        if let Some((closed, item)) = parse_item(c, chars) {
            items.push(item);
            if closed {
                break;
            }
        }
    }
    items
}

fn parse_item(c: char, chars: &mut Chars) -> Option<(bool, Item)> {
    match c {
        ',' => None,
        '[' => {
            let list = parse_list(chars);
            Some((false, Item::List(list)))
        }
        value => {
            let mut str = format!("{value}");
            let mut last = ',';
            let rest = chars
                .take_while(|c| {
                    last = *c;
                    c.is_ascii_digit()
                })
                .collect::<String>();
            str += rest.as_str();
            match str.parse() {
                Err(_) => {
                    println!("Invalid {str}");
                    None
                }
                Ok(val) => Some((last == ']', Item::Int(val))),
            }
        }
    }
}

pub(crate) fn parse(str: String) -> Packet {
    let mut chars = str.chars();
    chars.next(); // [
    let list = parse_list(&mut chars);
    list.into()
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        "[".to_string()
            + self
                .inner
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .as_str()
            + "]"
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Int(u) => u.to_string(),
            Item::List(l) => {
                "[".to_string()
                    + l.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                        .as_str()
                    + "]"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_13::{is_ordered, parse, Item, Packet};
    use crate::utils::io::input_file_lines;

    pub(crate) fn sample_pairs() -> Vec<(Packet, Packet)> {
        vec![
            (
                vec![
                    Item::Int(1),
                    Item::Int(1),
                    Item::Int(3),
                    Item::Int(1),
                    Item::Int(1),
                ]
                .into(),
                vec![
                    Item::Int(1),
                    Item::Int(1),
                    Item::Int(5),
                    Item::Int(1),
                    Item::Int(1),
                ]
                .into(),
            ),
            (
                vec![
                    Item::List(vec![Item::Int(1)]),
                    Item::List(vec![Item::Int(2), Item::Int(3), Item::Int(4)]),
                ]
                .into(),
                vec![Item::List(vec![Item::Int(1)]), Item::Int(4)].into(),
            ),
            (
                vec![Item::Int(9)].into(),
                vec![Item::List(vec![Item::Int(8), Item::Int(7), Item::Int(6)])].into(),
            ),
            (
                vec![
                    Item::List(vec![Item::Int(4), Item::Int(4)]),
                    Item::Int(4),
                    Item::Int(4),
                ]
                .into(),
                vec![
                    Item::List(vec![Item::Int(4), Item::Int(4)]),
                    Item::Int(4),
                    Item::Int(4),
                    Item::Int(4),
                ]
                .into(),
            ),
            (
                vec![Item::Int(7), Item::Int(7), Item::Int(7), Item::Int(7)].into(),
                vec![Item::Int(7), Item::Int(7), Item::Int(7)].into(),
            ),
            (vec![].into(), vec![Item::Int(3)].into()),
            (
                vec![Item::List(vec![Item::List(vec![])])].into(),
                vec![Item::List(vec![])].into(),
            ),
            (
                vec![
                    Item::Int(1),
                    Item::List(vec![
                        Item::Int(2),
                        Item::List(vec![
                            Item::Int(3),
                            Item::List(vec![
                                Item::Int(4),
                                Item::List(vec![Item::Int(5), Item::Int(6), Item::Int(7)]),
                            ]),
                        ]),
                    ]),
                    Item::Int(8),
                    Item::Int(9),
                ]
                .into(),
                vec![
                    Item::Int(1),
                    Item::List(vec![
                        Item::Int(2),
                        Item::List(vec![
                            Item::Int(3),
                            Item::List(vec![
                                Item::Int(4),
                                Item::List(vec![Item::Int(5), Item::Int(6), Item::Int(0)]),
                            ]),
                        ]),
                    ]),
                    Item::Int(8),
                    Item::Int(9),
                ]
                .into(),
            ),
        ]
    }

    fn sample_pair_n_order(n: usize) -> bool {
        let mut samples = sample_pairs();
        let (lhs, rhs) = samples.get_mut(n).unwrap();
        is_ordered(lhs, rhs)
    }

    #[test]
    fn check_sample_pair_1() {
        assert!(sample_pair_n_order(0));
    }

    #[test]
    fn check_sample_pair_2() {
        assert!(sample_pair_n_order(1));
    }
    #[test]
    fn check_sample_pair_3() {
        assert!(!sample_pair_n_order(2));
    }

    #[test]
    fn check_sample_pair_4() {
        assert!(sample_pair_n_order(3));
    }

    #[test]
    fn check_sample_pair_5() {
        assert!(!sample_pair_n_order(4));
    }

    #[test]
    fn check_sample_pair_6() {
        assert!(sample_pair_n_order(5));
    }

    #[test]
    fn check_sample_pair_7() {
        assert!(!sample_pair_n_order(6));
    }

    #[test]
    fn check_sample_pair_8() {
        assert!(!sample_pair_n_order(7));
    }

    fn check_parsing_n(idx: usize, input: (&str, &str)) {
        let (l, r) = input;
        let l = parse(l.to_string());
        let r = parse(r.to_string());
        let samples = sample_pairs();
        let (el, er) = samples.get(idx).unwrap();
        assert_eq!((el, er), (&l, &r));
        assert_eq!(input, (l.to_string().as_str(), r.to_string().as_str()))
    }

    #[test]
    fn check_parsing_sample_pair_1() {
        check_parsing_n(0, ("[1,1,3,1,1]", "[1,1,5,1,1]"));
    }

    #[test]
    fn check_parsing_sample_pair_2() {
        check_parsing_n(1, ("[[1],[2,3,4]]", "[[1],4]"));
    }

    #[test]
    fn check_parsing_sample_pair_3() {
        check_parsing_n(2, ("[9]", "[[8,7,6]]"));
    }

    #[test]
    fn check_parsing_sample_pair_4() {
        check_parsing_n(3, ("[[4,4],4,4]", "[[4,4],4,4,4]"));
    }

    #[test]
    fn check_parsing_sample_pair_5() {
        check_parsing_n(4, ("[7,7,7,7]", "[7,7,7]"));
    }

    #[test]
    fn check_parsing_sample_pair_6() {
        check_parsing_n(5, ("[]", "[3]"));
    }

    #[test]
    fn check_parsing_sample_pair_7() {
        check_parsing_n(6, ("[[[]]]", "[[]]"));
    }

    #[test]
    fn check_parsing_sample_pair_8() {
        check_parsing_n(
            7,
            ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
        );
    }

    #[test]
    fn check_nested() {
        let a = parse("[[0]]".to_string());
        assert_eq!(Packet::from(vec![Item::List(vec![Item::Int(0)])]), a);

        let a = parse("[[1],4]".to_string());
        assert_eq!(
            Packet::from(vec![Item::List(vec![Item::Int(1)]), Item::Int(4)]),
            a
        );
    }

    pub(crate) fn puzzle_input() -> Vec<(Packet, Packet)> {
        input_file_lines("day_13.txt")
            .unwrap()
            .map(Result::unwrap)
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|chunk| {
                let fst = &chunk[0];
                let snd = &chunk[1];
                (parse(fst.clone()), parse(snd.clone()))
            })
            .collect()
    }
}
