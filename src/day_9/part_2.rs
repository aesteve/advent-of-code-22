use crate::day_9::{new_coords, next_pos, Coord, Direction, Move};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Rope {
    pub(crate) positions: VecDeque<Coord>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Rope {
            positions: VecDeque::from(vec![Coord { x: 0, y: 0 }; size]),
        }
    }

    fn tail_pos(&self) -> Coord {
        self.positions.back().unwrap().clone()
    }

    fn move_towards(&mut self, direction: &Direction) {
        let mut new_positions = VecDeque::with_capacity(self.positions.len());
        let head = new_coords(&self.positions.pop_front().unwrap(), direction);
        new_positions.push_back(head);
        while let Some(old_coords) = self.positions.pop_front() {
            let prev = new_positions.back().unwrap();
            let new_coords = next_pos(&old_coords, prev);
            new_positions.push_back(new_coords);
        }
        self.positions = new_positions;
    }

    fn mov(&mut self, mov: &Move, traversed: &mut HashSet<Coord>) {
        for _ in 0..mov.qty {
            self.move_towards(&mov.direction);
            traversed.insert(self.tail_pos());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_9::part_2::Rope;
    use crate::day_9::tests::{puzzle_input, sample};
    use std::collections::HashSet;

    #[test]
    fn same_sample_results_if_rope_size_is_2() {
        let mut rope = Rope::new(2);
        let mut traversed = HashSet::new();
        for mov in sample() {
            rope.mov(&mov, &mut traversed);
        }
        assert_eq!(13, traversed.len());
    }

    #[test]
    fn same_puzzle_results_if_rope_size_is_2() {
        let mut rope = Rope::new(2);
        let mut traversed = HashSet::new();
        for mov in puzzle_input() {
            rope.mov(&mov, &mut traversed);
        }
        assert_eq!(6563, traversed.len());
    }

    #[test]
    fn solution() {
        let mut rope = Rope::new(10);
        let mut traversed = HashSet::new();
        for mov in puzzle_input() {
            rope.mov(&mov, &mut traversed);
        }
        assert_eq!(2653, traversed.len());
    }
}
