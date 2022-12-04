use std::cmp::max;

pub(crate) fn most_calories_carried<Carried, Input: IntoIterator<Item = Carried>>(
    input: Input,
    mapping: fn(Carried) -> Option<u32>,
) -> u64 {
    let mut most_carried: u64 = 0;
    let mut current = 0;
    for calories in input.into_iter() {
        let calories = mapping(calories);
        match calories {
            None => {
                most_carried = max(current, most_carried);
                current = 0;
            }
            Some(cal) => current += cal as u64,
        }
    }
    max(current, most_carried)
}

#[cfg(test)]
mod tests {
    use crate::day_1::cals_from_line;
    use crate::day_1::part_1::most_calories_carried;
    use crate::day_1::tests::{
        example_from_doc, first_is_biggest, last_is_biggest, read_from_input, Inventories,
    };
    use std::convert::identity;

    #[test]
    fn passes_example_from_doc() {
        let (inventories, expected) = example_from_doc();
        let res = most_calories_carried(inventories, identity);
        assert_eq!(res, expected)
    }

    #[test]
    fn works_if_biggest_is_last() {
        let (inventories, expected) = last_is_biggest();
        let res = most_calories_carried(inventories, identity);
        assert_eq!(res, expected)
    }

    #[test]
    fn works_if_biggest_is_first() {
        let (inventories, expected) = first_is_biggest();
        let res = most_calories_carried(inventories, identity);
        assert_eq!(res, expected)
    }

    #[test]
    fn website_solution() {
        let res = most_calories_carried(read_from_input(), cals_from_line);
        assert_eq!(res, 67_450)
    }

    // PBT
    // ==> useful to check overflows (that are not handled atm)
    #[quickcheck]
    fn a_single_element_inventory_must_return_itself(
        carried: Vec<u32>,
        before_nones: u8,
        after_nones: u8,
    ) {
        // one single elf
        let mut inventories: Inventories = std::iter::repeat(None)
            .take(before_nones as usize)
            .collect();
        let after: Inventories = std::iter::repeat(None).take(after_nones as usize).collect();
        let inventory: Inventories = carried.iter().map(|x| Some(*x)).collect();
        inventories.extend_from_slice(&inventory);
        inventories.extend_from_slice(&after);
        let res = most_calories_carried(inventories, identity);
        let elf_sum = carried.iter().fold(0_u64, |acc, &x| acc + x as u64);
        assert_eq!(elf_sum, res)
    }
}
