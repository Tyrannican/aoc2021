mod common;
use common::Solve;
mod day09;
use day09::solution::Solution;

fn main() {
    let mut s = Solution::new();
    s.part1();
    s.part2();
}
