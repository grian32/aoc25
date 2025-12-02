use std::fs;
use fancy_regex::Regex;
use crate::day::Day;

pub struct Day2 {

}
impl Day for Day2 {
    fn get_input(&self) -> String {
        fs::read_to_string("inputs/day2.txt").expect("read input")
    }

    fn part1(&self) {
        let input = self.get_input();

        let mut bad_number_sum: i64 = 0;

        for line in input.split(",") {
            let range: Vec<_> = line.split("-").collect();
            let start: i64 = range[0].parse().unwrap();
            let end: i64 = range[1].parse().unwrap();

            for i in start..=end {
                let string = i.to_string();
                let (begin, end) = string.split_at(string.len() / 2);
                if begin == end {
                    bad_number_sum += i;
                }
            }
        }

        println!("Day 1 Part 1: {}", bad_number_sum);
    }

    fn part2(&self) {
        let input = self.get_input();
        let re = Regex::new(r"^(\d+)\1+$").unwrap();

        let mut bad_number_sum: i64 = 0;

        for line in input.split(",") {
            let range: Vec<_> = line.split("-").collect();
            let start: i64 = range[0].parse().unwrap();
            let end: i64 = range[1].parse().unwrap();

            for i in start..=end {
                let m = re.find_iter(&*i.to_string()).count();
                if m >= 1 {
                    bad_number_sum += i;
                }
            }
        }

        println!("Day 1 Part 2: {}", bad_number_sum);
    }
}