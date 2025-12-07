use crate::days::day7::Day7;
use crate::days::day::Day;

pub mod days;

fn main() {
    let day = Day7 {};
    let input= day.get_input();
    println!("Part 1: {}", day.part1(&input));
    println!("Part 2: {}", day.part2(&input));
}
