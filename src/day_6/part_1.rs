#[cfg(test)]
mod tests {
    use crate::day_6::tests::puzzle_input;
    use crate::day_6::{first_marker, Buffer};

    #[test]
    fn test_full_buffer() {
        let mut buff = Buffer::new(3);
        buff.add('a');
        assert!(!buff.is_full());
        buff.add('b');
        assert!(!buff.is_full());
        buff.add('c');
        assert!(buff.is_full())
    }

    #[test]
    fn test_add_buffer() {
        let mut buff = Buffer::new(10);
        buff.add('a');
        assert!(!buff.is_full());
        buff.add('b');
        assert!(!buff.is_full());
        buff.add('b');
        assert!(!buff.is_full());
        assert!(buff.contains(&'b'));
        assert!(!buff.contains(&'a'));
        assert_eq!(1, buff.size)
    }

    #[test]
    fn test_sample_0() {
        let sample = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let marker = first_marker(sample, 4);
        assert_eq!(7, marker);
    }

    #[test]
    fn test_sample_1() {
        let sample = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let marker = first_marker(sample, 4);
        assert_eq!(5, marker);
    }

    #[test]
    fn test_sample_2() {
        let sample = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let marker = first_marker(sample, 4);
        assert_eq!(6, marker);
    }

    #[test]
    fn test_sample_3() {
        let sample = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let marker = first_marker(sample, 4);
        assert_eq!(10, marker);
    }

    #[test]
    fn test_sample_4() {
        let sample = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let marker = first_marker(sample, 4);
        assert_eq!(11, marker);
    }

    #[test]
    fn solution() {
        let line = puzzle_input().next().unwrap().unwrap();
        let marker = first_marker(line, 4);
        assert_eq!(1896, marker);
    }
}
