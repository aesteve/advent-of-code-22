#[cfg(test)]
mod tests {
    use crate::day_12::tests::puzzle_input;
    use crate::day_12::{paths, paths_rev, walk_to_iter};

    #[test]
    fn solution() {
        let grid = puzzle_input();
        let solutions = walk_to_iter(&grid, &grid.start(), &grid.end());
        for s in solutions.iter() {
            let collected = s.iter().map(|c| grid.at(c)).collect::<String>();
            let l = collected.len();
            println!("{collected:?}|{l}");
        }
        let best = solutions.first().unwrap();
        let collected = best.iter().map(|c| grid.at(c)).collect::<String>();
        let len = best.len();
        println!("{collected:?}");
        assert_eq!(481, len);
    }
}
