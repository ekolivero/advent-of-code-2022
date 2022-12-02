# Advent of code 2022 in RUST.

Sharing the solutions for the AoC 2022. The code is written in rust and in this file I'll share my learning during the journey.

## How to run the code

You need to first install [rust](https://www.rust-lang.org/learn/get-started).

Once installed clone the repo and from the root run:

```bash
cargo run --bin day<number>
```

Where number is the day of AoC you want to execute.

Apart from the day1 all the other days has tests, you can easily run them via:

```bash
cargo test --bin day<number>

```

If you have any advice please let me know :)

## Learnings

### Day 1

Not much to say. It was a warm up challange, showing how to parse raw text file and perform easy computation on it.

### Day 2

I really had fun with struct and the [match pattern](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html). I've applied TDD using the samples provided by AoC.
