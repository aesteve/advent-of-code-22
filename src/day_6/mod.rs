use std::collections::{HashSet, VecDeque};

mod part_1;
mod part_2;

#[derive(Debug)]
pub(crate) struct Buffer {
    q: VecDeque<char>,
    check: HashSet<char>,
    size: usize,
    max: usize,
}

impl Buffer {
    pub(crate) fn new(capacity: usize) -> Self {
        Buffer {
            q: VecDeque::with_capacity(capacity + 1),
            check: HashSet::with_capacity(capacity + 1),
            size: 0,
            max: capacity,
        }
    }

    pub(crate) fn add(&mut self, c: char) {
        if self.contains(&c) {
            self.remove(c)
        }
        self.q.push_back(c);
        self.check.insert(c);
        self.size += 1;
    }

    pub(crate) fn is_full(&self) -> bool {
        self.size >= self.max
    }

    fn contains(&self, c: &char) -> bool {
        self.check.contains(c)
    }

    fn remove(&mut self, c: char) {
        while let Some(removed) = self.q.pop_front() {
            self.check.remove(&removed);
            self.size -= 1;
            if removed == c {
                break;
            }
        }
    }
}

fn first_marker(input: String, size: usize) -> usize {
    let mut buffer = Buffer::new(size);
    for (i, c) in input.chars().enumerate() {
        if buffer.is_full() {
            return i;
        }
        buffer.add(c);
    }
    usize::MAX
}

#[cfg(test)]
mod tests {
    use crate::utils::io::{input_file_lines, FileLines};

    pub(crate) fn puzzle_input() -> FileLines {
        input_file_lines("day_6.txt").unwrap()
    }
}
