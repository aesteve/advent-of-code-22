use crate::day_4::RangePair;

fn overlap(range: &RangePair) -> bool {
    range.fst.contains(range.snd.start())
        || range.fst.contains(range.snd.end())
        || range.snd.contains(range.fst.start())
        || range.snd.contains(range.fst.end())
}

pub(crate) fn count_overlaps<A, T: IntoIterator<Item = A>>(
    lines: T,
    mapping: fn(A) -> RangePair,
) -> u64 {
    let mut total = 0;
    for line in lines {
        let mapped = mapping(line);
        if overlap(&mapped) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::day_4::part_2::{count_overlaps, overlap};
    use crate::day_4::tests::{parse_line, puzzle_input};
    use crate::day_4::RangePair;

    #[test]
    fn check_overlaps() {
        let range = RangePair {
            fst: 1..=3,
            snd: 2..=4,
        };
        assert!(overlap(&range));
        let range = RangePair {
            fst: 1..=2,
            snd: 2..=4,
        };
        assert!(overlap(&range));
        let range = RangePair {
            fst: 2..=3,
            snd: 2..=2,
        };
        assert!(overlap(&range));
        let range = RangePair {
            fst: 6..=8,
            snd: 2..=7,
        };
        assert!(overlap(&range));
    }

    #[test]
    fn check_no_overlaps() {
        let range = RangePair {
            fst: 1..=3,
            snd: 4..=4,
        };
        assert!(!overlap(&range));

        let range = RangePair {
            fst: 2..=3,
            snd: 5..=7,
        };
        assert!(!overlap(&range));
        let range = RangePair {
            fst: 6..=8,
            snd: 2..=3,
        };
        assert!(!overlap(&range));
    }

    #[test]
    fn solution() {
        let lines = puzzle_input();
        let result = count_overlaps(lines, parse_line);
        assert_eq!(895, result)
    }
}
