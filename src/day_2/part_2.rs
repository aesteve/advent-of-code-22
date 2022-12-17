use crate::day_2::{Move, Outcome, Round};

// We won't re_use TryFrom here (to avoid conflicting implementations)

fn from_char(value: char) -> Result<Move, String> {
    match value {
        'A' => Ok(Move::Rock),
        'B' => Ok(Move::Paper),
        'C' => Ok(Move::Scissors),
        _ => Err(format!("Unrecognized character {value}")),
    }
}

// Another mistake is that Ord from part1 indeed seems like a non-useful model, we would rather model an oriented circular relationship so that we can go backwards here
// Let's try here (and reuse tests from previous section if necessary)

trait Node<T> {
    fn beaten(&self) -> T;
    fn beats(&self) -> T;
}

impl Node<Move> for Move {
    fn beaten(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

fn from_desired_outcome(opp_move: &Move, outcome: Outcome) -> Move {
    match outcome {
        Outcome::Won => opp_move.beaten(),
        Outcome::Draw => opp_move.clone(),
        Outcome::Lost => opp_move.beats(),
    }
}

fn from_line_p2(line: String) -> Result<Round, String> {
    let mut chars = line.chars();
    let c = chars.next().ok_or("Unexpected empty line")?;
    let opponent_move = from_char(c)?;
    chars.next().ok_or("No whitespace separator")?;
    let c = chars
        .next()
        .ok_or("Expected another char after whitespace")?;
    let desired_outcome = match c {
        'X' => Outcome::Lost,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Won,
        _ => return Err(format!("unknown char {c}")),
    };
    Ok(Round {
        self_move: from_desired_outcome(&opponent_move, desired_outcome),
        opponent_move,
    })
}

#[cfg(test)]
mod tests {
    use crate::day_2::part_2::{from_line_p2, Node};
    use crate::day_2::tests::puzzle_input;
    use crate::day_2::Move;

    #[test]
    fn check_rules() {
        assert_eq!(Move::Rock, Move::Paper.beats());
        assert_eq!(Move::Paper, Move::Scissors.beats());
        assert_eq!(Move::Scissors, Move::Rock.beats());

        assert_eq!(Move::Rock, Move::Scissors.beaten());
        assert_eq!(Move::Paper, Move::Rock.beaten());
        assert_eq!(Move::Scissors, Move::Paper.beaten());
    }

    #[test]
    fn fst_round_score() {
        let round = from_line_p2("A Y".to_string()).unwrap();
        assert_eq!(Move::Rock, round.opponent_move);
        assert_eq!(Move::Rock, round.self_move);
        assert_eq!(4, round.score())
    }

    #[test]
    fn snd_round_score() {
        let round = from_line_p2("B X".to_string()).unwrap();
        assert_eq!(Move::Paper, round.opponent_move);
        assert_eq!(Move::Rock, round.self_move);
        assert_eq!(1, round.score())
    }

    #[test]
    fn trd_round_score() {
        let round = from_line_p2("C Z".to_string()).unwrap();
        assert_eq!(Move::Scissors, round.opponent_move);
        assert_eq!(Move::Rock, round.self_move);
        assert_eq!(7, round.score())
    }

    #[test]
    fn solution() {
        let mut total = 0;
        for line in puzzle_input() {
            let round = from_line_p2(line.unwrap()).unwrap();
            total += round.score() as u64;
        }
        assert_eq!(16098, total);
    }
}
