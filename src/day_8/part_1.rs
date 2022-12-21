#[cfg(test)]
mod tests {
    use crate::day_8::tests::puzzle_input;
    use crate::day_8::visible_trees;

    #[test]
    fn solution() {
        let trees = visible_trees(&puzzle_input());
        assert_eq!(1684, trees.len())
    }
}
