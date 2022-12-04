/// Strong contract here though: it's an OrderedTuple
/// Self note: an ordered heap would be more generic, but that's probably the next question
/// Also: since we are only interested in the sum of quantities carried by top 3, a heap is definitely a good idea
type Elves = (u32, u32, u32);

fn add(elves: Elves, new: u32) -> Elves {
    let (max, middle, min) = elves;
    if new > max {
        (new, max, middle)
    } else if new > middle {
        (max, new, middle)
    } else if new > min {
        (max, middle, new)
    } else {
        elves
    }
}

pub(crate) fn three_most_calories_carried<Carried, Input: IntoIterator<Item = Carried>>(
    input: Input,
    mapping: fn(Carried) -> Option<u32>,
) -> u64 {
    let mut most_carried = (0, 0, 0);
    let mut acc = 0;
    for calories in input.into_iter() {
        let calories = mapping(calories);
        match calories {
            None => {
                most_carried = add(most_carried, acc);
                acc = 0;
            }
            Some(cal) => acc += cal,
        }
    }
    most_carried = add(most_carried, acc);
    most_carried.0 as u64 + most_carried.1 as u64 + most_carried.2 as u64
}

#[cfg(test)]
mod tests {
    use crate::day_1::cals_from_line;
    use crate::day_1::part_2::three_most_calories_carried;
    use crate::day_1::tests::read_from_input;
    use std::convert::identity;

    // 3 first tests might be replaced by PBT: no matter the calories carried, if we have <= 3 elves => it's the sum of what they carry

    #[test]
    fn a_single_one_must_be_the_max() {
        let sample = vec![Some(1_000)];
        let res = three_most_calories_carried(sample, identity);
        assert_eq!(1_000, res)
    }

    #[test]
    fn check_two_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500)];
        let res = three_most_calories_carried(sample, identity);
        assert_eq!(2_500, res)
    }

    #[test]
    fn check_three_values() {
        let sample = vec![Some(1_000), None, Some(1_000), Some(500), None, Some(500)];
        let res = three_most_calories_carried(sample, identity);
        assert_eq!(3_000, res)
    }

    #[test]
    fn check_more_values() {
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
        let res = three_most_calories_carried(sample, identity);
        assert_eq!(5_500, res)
    }

    #[test]
    fn website_solution() {
        let carried = three_most_calories_carried(read_from_input(), cals_from_line);
        // 67450 + 66474 + 65433 = 199357
        assert_eq!(199357, carried)
    }
}
