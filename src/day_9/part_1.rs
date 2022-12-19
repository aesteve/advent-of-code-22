use crate::day_9::{eval_tail, new_coords, Coord, Move, State};
use std::collections::HashSet;

fn do_move(current: State, mov: &Move, traversed: &mut HashSet<Coord>) -> State {
    // move the head
    let mut state = current;
    for _ in 0..mov.qty {
        let new_head_pos = new_coords(&state.head, &mov.direction);
        let new_tail_pos = eval_tail(&state.tail, &new_head_pos);
        traversed.insert(new_tail_pos.clone());
        state = State {
            head: new_head_pos,
            tail: new_tail_pos,
        };
    }
    state
}

#[cfg(test)]
mod tests {
    use crate::day_9::part_1::do_move;
    use crate::day_9::tests::{puzzle_input, sample};
    use crate::day_9::State;
    use std::collections::HashSet;

    #[test]
    fn check_sample_positions() {
        let origin = State::default();
        let mut curr = origin;
        let mut traversed = HashSet::new();
        for mov in sample() {
            let state = do_move(curr.clone(), &mov, &mut traversed);
            curr = state;
        }
        assert_eq!(13, traversed.len());
    }

    fn valid_state(state: &State) -> bool {
        let dx = state.head.x - state.tail.x;
        let dy = state.head.y - state.tail.y;
        dx.abs() <= 1 && dy.abs() <= 1
    }

    #[test]
    fn solution() {
        let origin = State::default();
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
