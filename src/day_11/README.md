The hard part (in part 1) in Rust is that you can't both iterate over the monkeys and address the monkeys by index at the same time.
The most natural implementation would have been to make `Monkey::round(&mut self)` removing items.
But this leads to borrowing a Vec of Monkeys twice.
So one workaround is to make the evaluation of a round immutable, and then work on the mutable parts (removing items, etc.)