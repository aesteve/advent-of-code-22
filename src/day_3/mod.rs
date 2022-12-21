mod part_1;
mod part_2;

pub(crate) fn priority(item: char) -> u8 {
    if item.is_ascii_lowercase() {
        (item as u8) - b'a' + 1
    } else if item.is_ascii_uppercase() {
        (item as u8) - b'A' + 27
    } else {
        panic!("Non ascii character {item:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::day_3::priority;
    use crate::utils::io::{input_file_lines, FileLines};

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_3.txt").unwrap()
    }

    #[test]
    fn check_priorities() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));

        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
        assert_eq!(19, priority('s'));
    }
}
