#[cfg(test)]
mod tests {
    use crate::day_6::first_marker;
    use crate::day_6::tests::puzzle_input;

    const BUF_SIZE: usize = 14;

    #[test]
    fn test_sample_1() {
        let sample = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        let marker = first_marker(sample, BUF_SIZE);
        assert_eq!(19, marker);
    }

    #[test]
    fn test_sample_2() {
        let sample = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let marker = first_marker(sample, BUF_SIZE);
        assert_eq!(23, marker);
    }

    #[test]
    fn test_sample_3() {
        let sample = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let marker = first_marker(sample, BUF_SIZE);
        assert_eq!(23, marker);
    }

    #[test]
    fn test_sample_4() {
        let sample = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let marker = first_marker(sample, BUF_SIZE);
        assert_eq!(29, marker);
    }

    #[test]
    fn test_sample_5() {
        let sample = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        let marker = first_marker(sample, BUF_SIZE);
        assert_eq!(26, marker);
    }

    #[test]
    fn solution() {
        let line = puzzle_input().next().unwrap().unwrap();
        let marker = first_marker(line, BUF_SIZE);
        assert_eq!(3452, marker);
    }
}
