use crate::day_9::{new_coords, next_pos, Coord, Move};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct Rope {
    pub(crate) head: Coord,
    pub(crate) tail: Coord,
}

fn do_move(current: Rope, mov: &Move, traversed: &mut HashSet<Coord>) -> Rope {
    // move the head
    let mut state = current;
    for _ in 0..mov.qty {
        let new_head_pos = new_coords(&state.head, &mov.direction);
        let new_tail_pos = next_pos(&state.tail, &new_head_pos);
        traversed.insert(new_tail_pos.clone());
        state = Rope {
            head: new_head_pos,
            tail: new_tail_pos,
        };
    }
    state
}

#[cfg(test)]
mod tests {
    use crate::day_9::part_1::{do_move, Rope};
    use crate::day_9::tests::{puzzle_input, sample};
    use std::collections::HashSet;

    #[test]
    fn check_sample_positions() {
        let origin = Rope::default();
        let mut curr = origin;
        let mut traversed = HashSet::new();
        for mov in sample() {
            let state = do_move(curr.clone(), &mov, &mut traversed);
            curr = state;
        }
        assert_eq!(13, traversed.len());
    }

    fn valid_state(state: &Rope) -> bool {
        let dx = state.head.x - state.tail.x;
        let dy = state.head.y - state.tail.y;
        dx.abs() <= 1 && dy.abs() <= 1
    }

    #[test]
    fn solution() {
        let origin = Rope::default();
        let mut curr = origin;
        let mut traversed = HashSet::new();
        for mov in puzzle_input() {
            println!("{mov:?}");
            let state = do_move(curr.clone(), &mov, &mut traversed);
            println!("{state:?}");
            assert!(valid_state(&state));
            curr = state;
        }
        assert_eq!(6563, traversed.len());
    }
}
