use std::fs;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day1 {}

impl Day for Day1 {
    fn get_input(&self) -> String {
        fs::read_to_string("inputs/day1.txt").expect("read input")
    }

    fn part1(&self) {
        let input = self.get_input();

        let mut dial_pos = 50;
        let mut sum = 0;

        for line in input.lines()  {
            let direction = line.chars().nth(0).unwrap();
            let n: i32 = *&line[1..].parse().unwrap();
            match direction {
                'L' => {
                    dial_pos -= n;
                }
                'R' => {
                    dial_pos += n;
                }
                _ => { println!("invalid first char") }
            }

            dial_pos = dial_pos.rem_euclid(100);

            if dial_pos == 0 {
                sum += 1;
            }
        }

        println!("Day 1 Part 1: {}", sum);
    }

    fn part2(&self) {
        let input = self.get_input();

        let mut dial_pos: i32 = 50;
        let mut sum= 0;

        for line in input.lines()  {
            let direction = line.chars().nth(0).unwrap();
            let n: i32 = *&line[1..].parse().unwrap();

            for _ in 0..n {
                match direction {
                    'L' => dial_pos-= 1,
                    'R' => dial_pos+= 1,
                    _ => { println!("invalid first char") }
                }

                dial_pos = dial_pos.rem_euclid(100);

                if dial_pos == 0 {
                    sum += 1
                }
            }
        }

        println!("Day 1 Part 2: {}", sum)
    }
}