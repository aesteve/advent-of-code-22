mod part_1;
mod part_2;

use std::ops::RangeInclusive;

pub(crate) struct RangePair {
    fst: RangeInclusive<u32>,
    snd: RangeInclusive<u32>,
}

impl RangePair {
    /// Returns true if one range is fully contained in the other
    pub(crate) fn fully_contained(&self) -> bool {
        (self.fst.contains(self.snd.start()) && self.fst.contains(self.snd.end()))
            || self.snd.contains(self.fst.start()) && self.snd.contains(self.fst.end())
    }
}

pub(crate) fn parse_ranges(line: String) -> RangePair {
    let mut blocks = line.split(',');
    let (fst, snd) = (blocks.next().unwrap(), blocks.next().unwrap());
    let (fst, snd) = (parse_range(fst), parse_range(snd));
    RangePair { fst, snd }
}
fn parse_range(str: &str) -> RangeInclusive<u32> {
    let mut blocks = str.split('-');
    let (start, end) = (blocks.next().unwrap(), blocks.next().unwrap());
    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();
    start..=end
}

#[cfg(test)]
mod tests {
    use crate::day_4::{parse_ranges, RangePair};
    use crate::utils::io::{input_file_lines, FileLines};

    pub(crate) fn parse_line(l: std::io::Result<String>) -> RangePair {
        parse_ranges(l.unwrap())
    }

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_4.txt").unwrap()
    }

    #[test]
    fn can_parse_range() {
        let RangePair { fst, snd } = parse_ranges("2-6,4-8".to_string());
        assert_eq!(2..=6, fst);
        assert_eq!(4..=8, snd);
        assert!(fst.contains(&2)); // inclusive range
        assert!(fst.contains(&6)) // inclusive range
    }

    #[test]
    fn can_parse_range_double_digit() {
        let RangePair { fst, snd } = parse_ranges("2-62,4-8".to_string());
        assert_eq!(2..=62, fst);
        assert_eq!(4..=8, snd);
    }

    #[test]
    fn test_range_inclusion() {
        let larger = 2..=8;
        let smaller = 4..=6;
        let larger_fst = RangePair {
            fst: larger.clone(),
            snd: smaller.clone(),
        };
        assert!(larger_fst.fully_contained());
        let smaller_fst = RangePair {
            fst: smaller,
            snd: larger,
        };
        assert!(smaller_fst.fully_contained());
    }

    #[test]
    fn test_range_overlap() {
        let larger = 2..=5;
        let smaller = 4..=6;
        let larger_fst = RangePair {
            fst: larger.clone(),
            snd: smaller.clone(),
        };
        assert!(!larger_fst.fully_contained());
        let smaller_fst = RangePair {
            fst: smaller,
            snd: larger,
        };
        assert!(!smaller_fst.fully_contained());
    }

    #[test]
    fn test_range_exclusion() {
        let larger = 2..=3;
        let smaller = 4..=6;
        let larger_fst = RangePair {
            fst: larger.clone(),
            snd: smaller.clone(),
        };
        assert!(!larger_fst.fully_contained());
        let smaller_fst = RangePair {
            fst: smaller,
            snd: larger,
        };
        assert!(!smaller_fst.fully_contained());
    }

    #[test]
    fn contains_itself() {
        let range = 2..=3;
        let pair = RangePair {
            fst: range.clone(),
            snd: range,
        };
        assert!(pair.fully_contained());
    }
}
