use crate::day_3::priority;

/// Self-Note: got this one wrong the first time (was only building a global, per 3-lines frequency map)
/// Using tokens (per line) is easier although maybe not the most elegant solution

pub(crate) fn badge_sum<A, T: IntoIterator<Item = A>>(lines: T, mapping: fn(A) -> String) -> u64 {
    let mut sum = 0;
    let mut freq_map = [[3; 3]; 52];
    for (cpt_lines, line) in lines.into_iter().enumerate() {
        let line_in_grp = cpt_lines % 3;
        if line_in_grp == 0 {
            freq_map = [[3; 3]; 52];
        }
        for c in mapping(line).chars() {
            let priority = priority(c);
            let index = (priority - 1) as usize;
            let tokens = freq_map.get_mut(index).unwrap();
            let token = tokens.get_mut(line_in_grp).unwrap();
            *token = 0;
            if *tokens == [0; 3] {
                // encountered once per line so far
                sum += priority as u64;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day_3::part_2::badge_sum;
    use crate::day_3::tests::puzzle_input;

    #[test]
    fn sample_1() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        let badge = badge_sum(lines, str::to_string);
        assert_eq!(18, badge)
    }

    #[test]
    fn sample_2() {
        let lines = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let badge = badge_sum(lines, str::to_string);
        assert_eq!(52, badge)
    }

    #[test]
    fn combined_samples() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let badge = badge_sum(lines, str::to_string);
        assert_eq!(70, badge)
    }

    #[test]
    fn solution() {
        let total = badge_sum(puzzle_input(), Result::unwrap);
        assert_eq!(2415, total);
    }
}
