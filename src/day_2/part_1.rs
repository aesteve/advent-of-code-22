use crate::day_2::{Move, Round};

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(format!("Unrecognized character {value}")),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().next() {
            None => Err("Could not transform an empty String into Rock|Paper|Scissors".to_string()),
            Some(c) => Move::try_from(c),
        }
    }
}

impl TryFrom<String> for Round {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splitted = value.split(' ');
        if let (Some(a), Some(b)) = (splitted.next(), splitted.next()) {
            Ok(Round {
                opponent_move: Move::try_from(a)?,
                self_move: Move::try_from(b)?,
            })
        } else {
            Err("Expecting two characters separated by a whitespace".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::tests::puzzle_input;
    use crate::day_2::{Move, Outcome, Round};

    #[test]
    fn parses_sample_properly() {
        for line in puzzle_input() {
            let line = line.unwrap();
            assert!(Round::try_from(line).is_ok())
        }
    }

    #[test]
    fn fst_round_from_description() {
        let round = Round {
            opponent_move: Move::Rock,
            self_move: Move::Paper,
        };
        assert_eq!(Outcome::Won, round.outcome());
        assert_eq!(8, round.score())
    }

    #[test]
    fn snd_round_from_description() {
        let round = Round {
            opponent_move: Move::Paper,
            self_move: Move::Rock,
        };
        assert_eq!(Outcome::Lost, round.outcome());
        assert_eq!(1, round.score())
    }

    #[test]
    fn trd_round_from_description() {
        let round = Round {
            opponent_move: Move::Scissors,
            self_move: Move::Scissors,
        };
        assert_eq!(Outcome::Draw, round.outcome());
        assert_eq!(6, round.score())
    }

    #[test]
    fn problem_solution() {
        let mut res = 0;
        for line in puzzle_input() {
            let round = Round::try_from(line.unwrap()).unwrap();
            res += round.score() as u64;
        }
        assert_eq!(15572, res)
    }
}
