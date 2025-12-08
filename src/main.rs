use crate::days::day8::Day8;
use crate::days::day::Day;

pub mod days;

fn main() {
    let day = Day8 {};
    let mut input = day.get_input();
    println!("Part 1: {}", day.part1(&mut input));
    println!("Part 2: {}", day.part2(&mut input));
}
