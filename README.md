# Advent of Code 2022
My [Advent of Code 2022](https://adventofcode.com/2022)!

## Day 1 ~ 5
Create a new day
``` sh
cargo new --vcs=none day1
```

Run a day
``` sh
cargo run --bin day1 -- files/day1/sample1.txt
```

## Day 6 ~
Create a new day
``` sh
cargo run -p script -- --new-day 12
```

Test a day
```
cargo test -p day6
# for speed
cargo test --release -pday15
```