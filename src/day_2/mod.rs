mod part_1;
mod part_2;

use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub(crate) enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub(crate) enum Outcome {
    Won,
    Draw,
    Lost,
}

trait Scored {
    fn score(&self) -> u8;
}

impl Scored for Move {
    fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Scored for Outcome {
    fn score(&self) -> u8 {
        match self {
            Outcome::Won => 6,
            Outcome::Draw => 3,
            Outcome::Lost => 0,
        }
    }
}

// FIXME: looks weird to have repetition in the patterns, what am I missing?
impl PartialOrd<Self> for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Move::Rock => match other {
                Move::Rock => Ordering::Equal,
                Move::Paper => Ordering::Less,
                Move::Scissors => Ordering::Greater,
            },
            Move::Paper => match other {
                Move::Rock => Ordering::Greater,
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
            },
            Move::Scissors => match other {
                Move::Rock => Ordering::Less,
                Move::Paper => Ordering::Greater,
                Move::Scissors => Ordering::Equal,
            },
        })
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub(crate) struct Round {
    pub(crate) opponent_move: Move,
    pub(crate) self_move: Move,
}

impl Round {
    pub(crate) fn outcome(&self) -> Outcome {
        match self.self_move.cmp(&self.opponent_move) {
            Ordering::Less => Outcome::Lost,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Won,
        }
    }

    pub(crate) fn score(&self) -> u8 {
        self.self_move.score() + self.outcome().score()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::Move;
    use crate::utils::{input_file_lines, FileLines};

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_2.txt").unwrap()
    }

    #[test]
    fn test_ordering() {
        assert!(Move::Rock > Move::Scissors);
        assert!(Move::Rock < Move::Paper);
        assert!(Move::Paper > Move::Rock);
        assert!(Move::Paper < Move::Scissors);
        assert!(Move::Scissors > Move::Paper);
        assert!(Move::Scissors < Move::Rock);
    }
}
