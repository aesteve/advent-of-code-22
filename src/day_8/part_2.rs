use crate::day_8::{Forest, Tree};
use crate::utils::geom::{Coord, Direction, DIRECTIONS};
use std::cmp::max;

fn coords_towards(forest: &Forest, from: &Coord, towards: Direction) -> Vec<Coord> {
    let forest_len = forest.len() as i64; // always a square
    match towards {
        Direction::Up => (0..from.x).rev().map(|x| Coord { x, y: from.y }).collect(),
        Direction::Down => (from.x + 1..forest_len)
            .map(|x| Coord { x, y: from.y })
            .collect(),
        Direction::Left => (0..from.y).rev().map(|y| Coord { x: from.x, y }).collect(),
        Direction::Right => ((from.y + 1)..forest_len)
            .map(|y| Coord { x: from.x, y })
            .collect(),
    }
}

fn viewing_distance(forest: &Vec<Vec<Tree>>, from: &Tree, towards: Direction) -> usize {
    let coords_to_watch: Vec<Coord> = coords_towards(
        forest,
        &Coord {
            x: from.x as i64,
            y: from.y as i64,
        },
        towards,
    );
    let mut visible_trees = 0;
    for coord in coords_to_watch {
        visible_trees += 1;
        let tree = forest
            .get(coord.x as usize)
            .unwrap()
            .get(coord.y as usize)
            .unwrap();
        if tree.height >= from.height {
            break;
        }
    }
    visible_trees
}

fn scenic_score(forest: &Forest, from: &Tree) -> u64 {
    let mut total = 1;
    for direction in DIRECTIONS {
        total *= viewing_distance(forest, from, direction) as u64;
    }
    total
}

fn highest_scenic_score(forest: &Forest) -> u64 {
    let mut max_scenic = 0;
    for row in forest {
        for tree in row {
            max_scenic = max(max_scenic, scenic_score(forest, tree))
        }
    }
    max_scenic
}

#[cfg(test)]
mod tests {
    use crate::day_8::part_2::{
        coords_towards, highest_scenic_score, scenic_score, viewing_distance,
    };
    use crate::day_8::tests::puzzle_input_forest;
    use crate::day_8::tests::sample_forest;
    use crate::day_8::Tree;
    use crate::utils::geom::{Coord, Direction};

    #[test]
    fn check_trees_coords() {
        let forest = sample_forest();
        let top_left = Coord { x: 0, y: 0 };
        assert!(coords_towards(&forest, &top_left, Direction::Up).is_empty());
        assert!(coords_towards(&forest, &top_left, Direction::Left).is_empty());
        assert_eq!(
            4,
            coords_towards(&forest, &top_left, Direction::Right).len()
        );
        assert_eq!(4, coords_towards(&forest, &top_left, Direction::Down).len());
        let bottom_right = Coord { x: 4, y: 4 };
        assert!(coords_towards(&forest, &bottom_right, Direction::Down).is_empty());
        assert!(coords_towards(&forest, &bottom_right, Direction::Right).is_empty());
        assert_eq!(
            4,
            coords_towards(&forest, &bottom_right, Direction::Up).len()
        );
        assert_eq!(
            4,
            coords_towards(&forest, &bottom_right, Direction::Left).len()
        );
        let middle_tree = Coord { x: 2, y: 2 };
        assert_eq!(
            2,
            coords_towards(&forest, &middle_tree, Direction::Up).len()
        );
        assert_eq!(
            2,
            coords_towards(&forest, &middle_tree, Direction::Left).len()
        );
        assert_eq!(
            2,
            coords_towards(&forest, &middle_tree, Direction::Down).len()
        );
        assert_eq!(
            2,
            coords_towards(&forest, &middle_tree, Direction::Right).len()
        );
        // Coords are returned in the right order
        assert_eq!(
            vec![Coord { x: 1, y: 2 }, Coord { x: 0, y: 2 },],
            coords_towards(&forest, &middle_tree, Direction::Up)
        );
        assert_eq!(
            vec![Coord { x: 2, y: 3 }, Coord { x: 2, y: 4 },],
            coords_towards(&forest, &middle_tree, Direction::Right)
        );
        assert_eq!(
            vec![Coord { x: 2, y: 1 }, Coord { x: 2, y: 0 },],
            coords_towards(&forest, &middle_tree, Direction::Left)
        );
        assert_eq!(
            vec![Coord { x: 3, y: 2 }, Coord { x: 4, y: 2 },],
            coords_towards(&forest, &middle_tree, Direction::Down)
        );
    }

    #[test]
    fn check_viewing_distance() {
        let forest = sample_forest();
        let sample_tree = Tree {
            x: 3,
            y: 2,
            height: 5,
        };
        let visible_top = viewing_distance(&forest, &sample_tree, Direction::Up);
        assert_eq!(2, visible_top);
        let visible_left = viewing_distance(&forest, &sample_tree, Direction::Left);
        assert_eq!(2, visible_left);
        let visible_right = viewing_distance(&forest, &sample_tree, Direction::Right);
        assert_eq!(2, visible_right);
        let visible_down = viewing_distance(&forest, &sample_tree, Direction::Down);
        assert_eq!(1, visible_down);
    }

    #[test]
    fn check_scenic_score() {
        let forest = sample_forest();
        let sample_tree = Tree {
            x: 1,
            y: 2,
            height: 5,
        };
        assert_eq!(4, scenic_score(&forest, &sample_tree))
    }

    #[test]
    fn check_max_scenic_score() {
        let forest = sample_forest();
        assert_eq!(8, highest_scenic_score(&forest));
    }

    #[test]
    fn solution() {
        let forest = puzzle_input_forest();
        assert_eq!(486540, highest_scenic_score(&forest))
    }
}
