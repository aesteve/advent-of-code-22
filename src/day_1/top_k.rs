use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// OK, so implementation for part_2 was a bit cheating (using a 3-tuple)
/// Let's try a "Top K" implementation so that we can't use a Tuple
/// We would like to store the "Top K elves" in a data structure:
/// * allowing us to insert fast
/// * ordered, so that we can `pop` from it if the current elf is carrying more than the 1 carrying the least
/// Sounds like a priority queue (ordered heap)
/// Careful: we want a min heap (to pop the minimum), so inversing in Reverse order

pub struct Elves {
    inner: BinaryHeap<Reverse<u32>>, // The way to implement a reversed heap in Rust => Storing Reverse
    capacity: usize,
    calories: u64,
}

impl Elves {
    pub fn with_capacity(capacity: usize) -> Self {
        Elves {
            inner: BinaryHeap::with_capacity(capacity),
            capacity,
            calories: 0,
        }
    }

    pub fn push(&mut self, new: u32) {
        if self.inner.len() >= self.capacity {
            if let Some(min) = self.inner.peek() {
                if min.0 < new {
                    self.calories -= self.inner.pop().map(|u| u.0).unwrap_or(0) as u64;
                } else {
                    // every top K is greater than the new value => don't act
                    return;
                }
            }
        }
        self.inner.push(Reverse(new));
        self.calories += new as u64;
    }
}

fn top_k_most_calories_carried<Carried, Input: IntoIterator<Item = Carried>>(
    k: usize,
    input: Input,
    mapping: fn(Carried) -> Option<u32>,
) -> u64 {
    let mut elves = Elves::with_capacity(k);
    let mut acc = 0;
    for calories in input.into_iter() {
        let calories = mapping(calories);
        match calories {
            None => {
                elves.push(acc);
                acc = 0;
            }
            Some(cal) => acc += cal,
        }
    }
    elves.push(acc);
    elves.calories
}

#[cfg(test)]
mod tests {
    use crate::day_1::cals_from_line;
    use crate::day_1::part_1::most_calories_carried;
    use crate::day_1::part_2::three_most_calories_carried;
    use crate::day_1::tests::read_from_input;
    use crate::day_1::top_k::top_k_most_calories_carried;
    use std::convert::identity;

    // When using k = 1 we can re-use the tests from part 1

    #[test]
    fn part_1_a_single_one_must_be_the_max() {
        let sample = vec![Some(1_000)];
        let res = top_k_most_calories_carried(1, sample.clone(), identity);
        assert_eq!(most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_1_check_two_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500)];
        let res = top_k_most_calories_carried(1, sample.clone(), identity);
        assert_eq!(most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_1_check_three_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500), None, Some(500)];
        let res = top_k_most_calories_carried(1, sample.clone(), identity);
        assert_eq!(most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_1_check_more_values() {
        let sample = vec![
            Some(1_000),
            None,
            Some(1_000),
            Some(500),
            None,
            Some(500),
            None,
            Some(200),
            Some(100),
            None,
            Some(2_000),
            Some(1_000),
            None,
            Some(100),
            None,
        ];
        let res = top_k_most_calories_carried(1, sample.clone(), identity);
        assert_eq!(most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_1_website_solution() {
        let carried = top_k_most_calories_carried(1, read_from_input(), cals_from_line);
        assert_eq!(
            most_calories_carried(read_from_input(), cals_from_line),
            carried
        )
    }

    // We can re-use the previous test for k = 3
    #[test]
    fn part_2_a_single_one_must_be_the_max() {
        let sample = vec![Some(1_000)];
        let res = top_k_most_calories_carried(3, sample.clone(), identity);
        assert_eq!(three_most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_2_check_two_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500)];
        let res = top_k_most_calories_carried(3, sample.clone(), identity);
        assert_eq!(three_most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_2_check_three_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500), None, Some(500)];
        let res = top_k_most_calories_carried(3, sample.clone(), identity);
        assert_eq!(three_most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_2_check_more_values() {
        let sample = vec![
            Some(1_000),
            None,
            Some(1_000),
            Some(500),
            None,
            Some(500),
            None,
            Some(200),
            Some(100),
            None,
            Some(2_000),
            Some(1_000),
            None,
            Some(100),
            None,
        ];
        let res = top_k_most_calories_carried(3, sample.clone(), identity);
        assert_eq!(three_most_calories_carried(sample, identity), res)
    }

    #[test]
    fn part_2_website_solution() {
        let carried = top_k_most_calories_carried(3, read_from_input(), cals_from_line);
        // 67450 + 66474 + 65433 = 199357
        assert_eq!(
            three_most_calories_carried(read_from_input(), cals_from_line),
            carried
        )
    }
}
