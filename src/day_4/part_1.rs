use crate::day_4::RangePair;

pub(crate) fn count_contained<A, T: IntoIterator<Item = A>>(
    lines: T,
    mapping: fn(A) -> RangePair,
) -> u64 {
    let mut total = 0;
    for line in lines {
        let mapped = mapping(line);
        if mapped.fully_contained() {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::day_4::part_1::count_contained;
    use crate::day_4::tests::{parse_line, puzzle_input};
    use crate::day_4::{parse_ranges, RangePair};

    #[test]
    fn solution() {
        let lines = puzzle_input();
        let result = count_contained(lines, parse_line);
        assert_eq!(580, result)
    }
}
