mod common;
use common::Solve;
mod day11;
use day11::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
