use std::fs;
use crate::days::day::Day;

pub struct DialTurn {
    direction: char,
    turns: i32,
}

#[allow(dead_code)]
pub struct Day1 {}

impl Day<Vec<DialTurn>, i32> for Day1 {
    fn get_input(&self) -> Vec<DialTurn> {
        let file = fs::read_to_string("inputs/day1.txt").expect("read input");
        let lines = file.lines();
        let mut vec: Vec<DialTurn> = Vec::new();
        for line in lines {
            let direction = line.chars().nth(0).unwrap();
            let turns: i32 = *&line[1..].parse().unwrap();
            let turn: DialTurn = DialTurn{
                direction,
                turns
            };
            vec.push(turn);
        }
        
        vec
    }

    fn part1(&self, input: &Vec<DialTurn>) -> i32 {
        let mut dial_pos = 50;
        let mut sum = 0;

        for line in input  {
            match line.direction {
                'L' => {
                    dial_pos -= line.turns;
                }
                'R' => {
                    dial_pos += line.turns;
                }
                _ => { println!("invalid first char") }
            }

            dial_pos = dial_pos.rem_euclid(100);

            if dial_pos == 0 {
                sum += 1;
            }
        }

        sum
    }

    fn part2(&self, input: &Vec<DialTurn>) -> i32{
        let mut dial_pos: i32 = 50;
        let mut sum= 0;

        for line in input{
            for _ in 0..line.turns {
                match line.direction {
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

        sum
    }
}