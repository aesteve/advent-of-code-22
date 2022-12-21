mod part_1;

use linked_hash_set::LinkedHashSet;
use std::cmp;
use std::collections::{HashMap, HashSet}; // just for the sake of keeping the coordinates in order (debugging purposes)

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Grid {
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn new(inner: Vec<Vec<char>>) -> Self {
        Grid { inner }
    }

    fn width(&self) -> usize {
        self.inner.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.inner.len()
    }

    fn at(&self, coord: &Coord) -> char {
        *self.inner.get(coord.x).unwrap().get(coord.y).unwrap()
    }

    fn find(&self, target: char) -> Coord {
        self.inner
            .iter()
            .enumerate()
            .find_map(|(i, vec)| {
                vec.iter().enumerate().find_map(|(j, c)| {
                    if *c == target {
                        Some(Coord { x: i, y: j })
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn end(&self) -> Coord {
        self.find('E')
    }

    fn start(&self) -> Coord {
        self.find('S')
    }
}

impl Coord {
    fn left(&self) -> Option<Coord> {
        self.y
            .checked_add_signed(-1)
            .map(|y| Coord { x: self.x, y })
    }
    fn up(&self) -> Option<Coord> {
        self.x
            .checked_add_signed(-1)
            .map(|x| Coord { x, y: self.y })
    }
    fn right(&self, size: usize) -> Option<Coord> {
        if self.y + 1 >= size {
            return None;
        }
        Some(Coord {
            x: self.x,
            y: self.y + 1,
        })
    }
    fn bottom(&self, size: usize) -> Option<Coord> {
        if self.x + 1 >= size {
            return None;
        }
        Some(Coord {
            x: self.x + 1,
            y: self.y,
        })
    }
}

fn valid_directions(grid: &Grid, origin: &Coord) -> Vec<Coord> {
    let height = grid.height();
    let width = grid.width();
    vec![
        origin.left(),
        origin.up(),
        origin.right(width),
        origin.bottom(height),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn valid_elevation(origin: char, new: char) -> bool {
    if new == 'E' {
        return origin == 'z';
    }
    if new == 'S' {
        return false;
    }
    if origin == 'S' {
        return new == 'a';
    }
    new as u8 <= origin as u8 + 1
}

fn valid_elevation_reverse(origin: char, new: char) -> bool {
    if origin == 'E' {
        return new == 'z';
    }
    if origin == 'S' {
        return true;
    }
    if origin == 'a' {
        return new == 'S';
    }
    new as u8 >= origin as u8 - 1
}

fn allowed_directions(grid: &Grid, origin: &Coord) -> Vec<Coord> {
    let origin_elevation = grid.at(origin);
    valid_directions(grid, origin)
        .into_iter()
        .filter(|coord| {
            let elevation = grid.at(coord);
            valid_elevation(origin_elevation, elevation)
        })
        .collect()
}

fn allowed_directions_reverse(grid: &Grid, origin: &Coord) -> Vec<Coord> {
    let origin_elevation = grid.at(origin);
    println!("elevation: {origin_elevation:?}");
    valid_directions(grid, origin)
        .into_iter()
        .filter(|coord| {
            let elevation = grid.at(coord);
            valid_elevation_reverse(origin_elevation, elevation)
        })
        .collect()
}

fn walk_to(
    grid: &Grid,
    origin: &Coord,
    visited: &mut HashMap<Coord, usize>,
    traversed: &mut LinkedHashSet<Coord>,
    solutions: &mut Vec<LinkedHashSet<Coord>>,
) {
    let mut directions_possible = allowed_directions(grid, origin);
    directions_possible.sort_by_key(|f| cmp::Reverse(grid.at(f)));
    for coord in directions_possible {
        let mut traversed_so_far = traversed.clone();
        if traversed_so_far.contains(&coord) {
            continue; // next direction
        }
        if let Some(v) = visited.get(&coord) {
            if *v < traversed.len() {
                continue; // we already visited it with a shorter path
            }
        }
        if grid.at(&coord) == 'E' {
            solutions.push(traversed_so_far.clone());
            solutions.sort_by_key(|f| f.len());
            return;
        }
        traversed_so_far.insert(coord.clone());
        let v = visited.entry(coord.clone()).or_insert(usize::MAX);
        *v = cmp::min(*v, traversed.len());
        if let Some(found) = solutions.first() {
            if traversed_so_far.len() > found.len() {
                break;
            }
        }
        walk_to(grid, &coord, visited, &mut traversed_so_far, solutions);
    }
}

fn walk_from(
    grid: &Grid,
    origin: &Coord,
    traversed: &mut LinkedHashSet<Coord>,
    solutions: &mut Vec<LinkedHashSet<Coord>>,
) {
    let mut directions_possible = allowed_directions_reverse(grid, origin);
    directions_possible.sort_by_key(|f| grid.at(f));
    for coord in directions_possible {
        let mut traversed_so_far = traversed.clone();
        if traversed_so_far.contains(&coord) {
            continue; // next direction
        }
        if grid.at(&coord) == 'S' {
            solutions.push(traversed_so_far.clone());
            solutions.sort_by_key(|f| f.len());
            return;
        }
        traversed_so_far.insert(coord.clone());
        if let Some(found) = solutions.first() {
            if traversed_so_far.len() > found.len() {
                break;
            }
        }
        walk_from(grid, &coord, &mut traversed_so_far, solutions);
    }
}

pub(crate) fn paths(grid: &Grid) -> Vec<LinkedHashSet<Coord>> {
    let mut solutions = vec![];
    let mut traversed = LinkedHashSet::new();
    walk_to(
        grid,
        &grid.start(),
        &mut HashMap::new(),
        &mut traversed,
        &mut solutions,
    );
    solutions
}

pub(crate) fn paths_rev(grid: &Grid) -> Vec<LinkedHashSet<Coord>> {
    let mut solutions = vec![];
    let mut traversed = LinkedHashSet::new();
    walk_from(grid, &grid.end(), &mut traversed, &mut solutions);
    solutions
}

#[cfg(test)]
mod tests {
    use crate::day_12::{
        allowed_directions, allowed_directions_reverse, paths, paths_rev, valid_elevation, walk_to,
        Coord, Grid,
    };
    use crate::utils::io::input_file_lines;

    pub(crate) fn puzzle_input() -> Grid {
        Grid::new(
            input_file_lines("day_12.txt")
                .unwrap()
                .map(|line| line.unwrap().chars().collect::<Vec<_>>())
                .collect(),
        )
    }

    fn sample() -> Grid {
        let raw = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        let vec = raw.lines().map(|c| c.chars().collect()).collect();
        Grid::new(vec)
    }

    #[test]
    fn check_valid_elevation() {
        assert!(valid_elevation('a', 'b'));
        assert!(valid_elevation('a', 'a'));
        assert!(valid_elevation('c', 'd'));
        assert!(valid_elevation('d', 'c'));
        assert!(!valid_elevation('c', 't'));
        assert!(!valid_elevation('x', 'E'));
        assert!(valid_elevation('z', 'E'));
    }

    #[test]
    fn check_few_valid_directions() {
        let grid = sample();
        let directions = allowed_directions(&grid, &Coord { x: 0, y: 2 });
        assert_eq!(
            vec![Coord { x: 0, y: 1 }, Coord { x: 1, y: 2 },],
            directions
        );
        let directions = allowed_directions(&grid, &Coord { x: 2, y: 2 });
        assert_eq!(
            vec![
                Coord { x: 2, y: 1 },
                Coord { x: 1, y: 2 },
                Coord { x: 3, y: 2 }
            ],
            directions
        );
        let directions = allowed_directions(&grid, &Coord { x: 1, y: 2 });
        println!("{directions:?}")
    }

    #[test]
    fn check_few_valid_directions_reverse() {
        let grid = sample();
        let directions = allowed_directions_reverse(&grid, &Coord { x: 2, y: 3 });
        assert_eq!(
            vec![
                Coord { x: 1, y: 3 },
                Coord { x: 2, y: 4 },
                Coord { x: 3, y: 3 }
            ],
            directions
        );
    }

    #[test]
    fn check_start_end() {
        let grid = sample();
        assert_eq!(Coord { x: 0, y: 0 }, grid.start());
        assert_eq!(Coord { x: 2, y: 5 }, grid.end());
    }

    #[test]
    fn check_walk_to() {
        let grid = sample();
        let solutions = paths(&grid);
        let s = solutions.first().unwrap();
        let collected = s.iter().map(|coord| grid.at(coord)).collect::<String>();
        assert_eq!("abcccdefghijklmnopqrstuvwxxxyz".to_string(), collected);
        let len = s.len() + 1;
        assert_eq!(31, len);
    }

    #[test]
    fn check_walk_from() {
        let grid = sample();
        let solutions = paths_rev(&grid);
        let nb_solutions = solutions.len();
        println!("{nb_solutions:?}");
        let s = solutions.first().unwrap();
        println!("{}", s.len());
        let collected = s
            .iter()
            .map(|coord| grid.at(coord))
            .rev()
            .collect::<String>();
        println!("{collected:?}");
        assert_eq!("abcccdefghijklmnopqrstuvwxxxyz".to_string(), collected);
        let len = s.len() + 1;
        assert_eq!(31, len);
    }
}
