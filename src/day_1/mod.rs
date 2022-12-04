use std::str::FromStr;

mod part_1;
mod part_2;
mod top_k;

pub(crate) fn cals_from_line(line: std::io::Result<String>) -> Option<u32> {
    line.ok().and_then(|str| u32::from_str(&str).ok())
}

#[cfg(test)]
mod tests {
    use crate::utils::{input_file_lines, FileLines};

    /// In order to focus on the algorithm / implementation and test it while abstracting from storage
    pub(crate) type Inventories = Vec<Option<u32>>;

    pub(crate) fn example_from_doc() -> (Inventories, u64) {
        let sample = vec![
            Some(1_000),
            Some(2_000),
            Some(3_000),
            None,
            Some(4_000),
            None,
            Some(5_000),
            Some(6_000),
            None,
            Some(7_000),
            Some(8_000),
            Some(9_000),
            None,
            Some(10_000),
        ];
        (sample, 24_000)
    }

    pub(crate) fn last_is_biggest() -> (Inventories, u64) {
        (vec![Some(1_000), None, Some(1_000), Some(1_000)], 2_000)
    }

    pub(crate) fn first_is_biggest() -> (Inventories, u64) {
        (vec![Some(3_000), None, Some(1_000), Some(1_000)], 3_000)
    }

    pub(crate) fn read_from_input() -> FileLines {
        input_file_lines("day_1.txt").unwrap()
    }
}
