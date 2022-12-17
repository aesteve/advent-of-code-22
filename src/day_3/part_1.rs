use crate::day_3::priority;
use std::collections::{BTreeSet, HashMap};

pub(crate) fn duplicated_priority(line: String) -> u8 {
    priority(find_duplicated(line))
}

pub(crate) fn find_duplicated(line: String) -> char {
    let middle = line.len() / 2;
    let mut indices_by_char: HashMap<char, BTreeSet<usize>> = HashMap::new();
    for (i, c) in line.chars().enumerate() {
        let indices = indices_by_char.entry(c).or_default();
        indices.insert(i);
        if i < middle {
            continue;
        }
        // we're on the right side of the String
        if let (Some(&smallest), Some(&biggest)) = (indices.first(), indices.last()) {
            if smallest < middle && biggest >= middle {
                return c;
            }
        }
    }
    panic!("No duplicated value found for {line}");
}

#[cfg(test)]
mod tests {
    use crate::day_3::part_1::{duplicated_priority, find_duplicated};
    use crate::day_3::tests::puzzle_input;

    #[test]
    fn check_sample_1() {
        let sample = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let dup = find_duplicated(sample);
        assert_eq!('p', dup);
    }

    #[test]
    fn check_sample_2() {
        let sample = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let dup = find_duplicated(sample);
        assert_eq!('L', dup);
    }

    #[test]
    fn check_sample_3() {
        let sample = "PmmdzqPrVvPwwTWBwg".to_string();
        let dup = find_duplicated(sample);
        assert_eq!('P', dup);
    }

    #[test]
    fn check_sample() {
        let sample = "CScCSPcPszFJWSMjGZHMpGMjvG".to_string();
        let dup = find_duplicated(sample);
        assert_eq!('S', dup);
    }

    #[test]
    fn solution() {
        let mut total = 0;
        for line in puzzle_input() {
            total += duplicated_priority(line.unwrap()) as u64;
        }
        assert_eq!(7766, total);
    }
}
