mod part_1;
mod part_2;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

pub(crate) type Forest = Vec<Vec<Tree>>; // squared

fn visible_trees(forest: &Vec<Vec<char>>) -> HashSet<Tree> {
    let len = forest.len();
    let mut visible = HashSet::new();
    for i in 0..len {
        let mut tallest_from_left = -1;
        let mut tallest_from_right = -1;

        let mut tallest_from_top = -1;
        let mut tallest_from_bottom = -1;

        let row = forest.get(i).unwrap();
        let row_len = row.len();

        for j in 0..row_len {
            // Row by Row
            let h_left = row.get(j).unwrap();
            let left_tree = Tree {
                x: i,
                y: j,
                height: h_left.to_digit(10).unwrap(),
            };
            let h_right = row.get(row_len - j - 1).unwrap();
            let right_tree = Tree {
                x: i,
                y: row_len - j - 1,
                height: h_right.to_digit(10).unwrap(),
            };
            if left_tree.height as i64 > tallest_from_left {
                tallest_from_left = left_tree.height as i64;
                visible.insert(left_tree);
            }
            if right_tree.height as i64 > tallest_from_right {
                tallest_from_right = right_tree.height as i64;
                visible.insert(right_tree);
            }

            // Col by Col
            let h_top = forest.get(j).unwrap().get(i).unwrap();
            let top_tree = Tree {
                x: j,
                y: i,
                height: h_top.to_digit(10).unwrap(),
            };
            let h_bottom = forest.get(len - j - 1).unwrap().get(i).unwrap();
            let bottom_tree = Tree {
                x: len - j - 1,
                y: i,
                height: h_bottom.to_digit(10).unwrap(),
            };
            if top_tree.height as i64 > tallest_from_top {
                tallest_from_top = top_tree.height as i64;
                visible.insert(top_tree);
            }
            if bottom_tree.height as i64 > tallest_from_bottom {
                tallest_from_bottom = bottom_tree.height as i64;
                visible.insert(bottom_tree);
            }
        }
    }
    visible
}

#[cfg(test)]
mod tests {
    use crate::day_8::{visible_trees, Forest, Tree};
    use crate::utils::io::input_file_lines;

    pub(crate) fn sample() -> Vec<Vec<char>> {
        vec![
            vec!['3', '0', '3', '7', '3'],
            vec!['2', '5', '5', '1', '2'],
            vec!['6', '5', '3', '3', '2'],
            vec!['3', '3', '5', '4', '9'],
            vec!['3', '5', '3', '9', '0'],
        ]
    }

    fn create_forest(origin: Vec<Vec<char>>) -> Forest {
        origin
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| Tree {
                        x: i,
                        y: j,
                        height: c.to_digit(10).unwrap(),
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect()
    }

    pub(crate) fn sample_forest() -> Forest {
        create_forest(sample())
    }

    pub(crate) fn puzzle_input() -> Vec<Vec<char>> {
        input_file_lines("day_8.txt")
            .unwrap()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect()
    }

    pub(crate) fn puzzle_input_forest() -> Forest {
        input_file_lines("day_8.txt")
            .unwrap()
            .enumerate()
            .map(|(i, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(j, c)| Tree {
                        x: i,
                        y: j,
                        height: c.to_digit(10).unwrap(),
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn sample_visible_trees() {
        let visible = visible_trees(&sample());
        assert_eq!(21, visible.len());
    }
}
