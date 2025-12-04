use crate::days::day4::Day4;
use crate::days::day::Day;

pub mod days;

fn main() {
    let day = Day4 {};
    let input= day.get_input();
    println!("Part 1: {}", day.part1(&input));
    println!("Part 2: {}", day.part2(&input));
}
