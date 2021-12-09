mod common;
use common::Solve;
mod day08;
use day08::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
