#[cfg(test)]
mod tests {
    use crate::day_12::tests::puzzle_input;
    use crate::day_12::{paths, paths_rev};

    #[test]
    fn solution() {
        let grid = puzzle_input();
        println!("grid: {grid:?}");
        // TODO: explodes in complexity... why? :'(

        // let solutions = paths(&grid);
        // let best = solutions.first().unwrap();
        // let collected = best.iter().map(|c| grid.at(c)).collect::<String>();
        // let len = best.len() + 1;
        // println!("{collected:?}");
        // assert_eq!(12345, len);
    }
}
