use crate::day_9::Direction::{Down, Left, Right, Up};

mod part_1;
mod part_2;
mod soluce;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct Coord {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Move {
    pub(crate) direction: Direction,
    pub(crate) qty: u64,
}

pub(crate) fn new_coords(start: &Coord, direction: &Direction) -> Coord {
    match direction {
        Up => Coord {
            x: start.x,
            y: start.y - 1,
        },
        Down => Coord {
            x: start.x,
            y: start.y + 1,
        },
        Left => Coord {
            x: start.x - 1,
            y: start.y,
        },
        Right => Coord {
            x: start.x + 1,
            y: start.y,
        },
    }
}

pub(crate) fn next_pos(origin: &Coord, head_pos: &Coord) -> Coord {
    let d_x = head_pos.x - origin.x;
    let d_y = head_pos.y - origin.y;
    if d_x.abs() > 1 || d_y.abs() > 1 {
        Coord {
            x: origin.x + d_x.signum(),
            y: origin.y + d_y.signum(),
        }
    } else {
        origin.clone()
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "R" => Ok(Right),
            "L" => Ok(Left),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(format!("Unknown direction {value}")),
        }
    }
}

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut splitted = value.split(' ');
        let direction: Direction = splitted
            .next()
            .ok_or(format!("Invalid move {value:?}"))?
            .try_into()?;
        let qty = splitted
            .next()
            .ok_or(format!("Invalid move {value:?}"))?
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        Ok(Move { direction, qty })
    }
}

#[cfg(test)]
mod tests {
    use crate::day_9::Direction::{Down, Left, Right, Up};
    use crate::day_9::{next_pos, Coord, Move};
    use crate::utils::{input_file_lines, FileLines};

    pub(crate) fn puzzle_input() -> Vec<Move> {
        input_file_lines("day_9.txt")
            .unwrap()
            .map(|line| {
                let line = line.unwrap();
                line.try_into().unwrap()
            })
            .collect()
    }

    pub(crate) fn sample() -> Vec<Move> {
        vec![
            Move {
                direction: Right,
                qty: 4,
            },
            Move {
                direction: Up,
                qty: 4,
            },
            Move {
                direction: Left,
                qty: 3,
            },
            Move {
                direction: Down,
                qty: 1,
            },
            Move {
                direction: Right,
                qty: 4,
            },
            Move {
                direction: Down,
                qty: 1,
            },
            Move {
                direction: Left,
                qty: 5,
            },
            Move {
                direction: Right,
                qty: 2,
            },
        ]
    }

    #[test]
    fn tail_doesnt_move_if_not_necessary() {
        let origin = Coord { x: 0, y: 0 };
        let tail_pos = next_pos(&origin, &origin);
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: 1, y: 0 });
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: 0, y: 1 });
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: 1, y: 1 });
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: -1, y: 0 });
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: 0, y: -1 });
        assert_eq!(tail_pos, origin);

        let tail_pos = next_pos(&origin, &Coord { x: -1, y: -1 });
        assert_eq!(tail_pos, origin);
    }

    #[test]
    fn tail_follows_head_along_col() {
        let origin = Coord { x: 0, y: 0 };
        let tail_pos = next_pos(&origin, &Coord { x: 2, y: 0 });
        assert_eq!(tail_pos, Coord { x: 1, y: 0 });

        let tail_pos = next_pos(&origin, &Coord { x: -2, y: 0 });
        assert_eq!(tail_pos, Coord { x: -1, y: 0 });
    }

    #[test]
    fn tail_follows_head_along_row() {
        let origin = Coord { x: 0, y: 0 };
        let tail_pos = next_pos(&origin, &Coord { x: 0, y: 2 });
        assert_eq!(tail_pos, Coord { x: 0, y: 1 });

        let tail_pos = next_pos(&origin, &Coord { x: 0, y: -2 });
        assert_eq!(tail_pos, Coord { x: 0, y: -1 });
    }

    #[test]
    fn tail_follows_head_in_diagonal() {
        let origin = Coord { x: 0, y: 0 };
        let tail_pos = next_pos(&origin, &Coord { x: 2, y: 2 });
        assert_eq!(tail_pos, Coord { x: 1, y: 1 });

        let tail_pos = next_pos(&origin, &Coord { x: -2, y: -2 });
        assert_eq!(tail_pos, Coord { x: -1, y: -1 });

        let tail_pos = next_pos(&origin, &Coord { x: -2, y: 2 });
        assert_eq!(tail_pos, Coord { x: -1, y: 1 });

        let tail_pos = next_pos(&origin, &Coord { x: 2, y: -2 });
        assert_eq!(tail_pos, Coord { x: 1, y: -1 });
    }

    #[test]
    fn parses_moves_correctly() {
        let mov: Result<Move, String> = "U 1".to_string().try_into();
        assert!(mov.is_ok());
        assert_eq!(
            Move {
                direction: Up,
                qty: 1
            },
            mov.unwrap()
        )
    }
}
