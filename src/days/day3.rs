use std::fs;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day3 {

}

impl Day<Vec<Vec<i32>>, i64> for Day3 {
    fn get_input(&self) -> Vec<Vec<i32>> {
        let file = fs::read_to_string("inputs/day3.txt").expect("read input");
        let mut vec: Vec<Vec<i32>> = Vec::new();
        for line in file.lines() {
            let numbers = line
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as i32)
                    .collect();
            vec.push(
                numbers
            );
        }

        vec
    }

    fn part1(&self, input: &Vec<Vec<i32>>) -> i64 {
        let mut curr_max_joltage = 0;

        for line in input {
            let mut numbers = line.to_vec();

            let mut max_idx= 0;
            let mut max = -1;

            for i in 0..numbers.len() {
                if numbers[i] > max {
                    max = numbers[i];
                    max_idx = i;
                }
            }

            if max_idx == numbers.len() - 1 {
                numbers.sort();
                curr_max_joltage += numbers[numbers.len() - 2] * 10 + max;
            } else {
                let mut second_max = -1;
                for i in max_idx + 1..numbers.len() {
                    if numbers[i] > second_max {
                        second_max = numbers[i];
                    }
                }
                curr_max_joltage += max * 10 + second_max;
            }
        }

        curr_max_joltage as i64
    }

    fn part2(&self, input: &Vec<Vec<i32>>) -> i64 {
        let mut curr_max_joltage = 0;

        for line in input {
            let numbers: Vec<_> = line.to_vec();
            let num_len = numbers.len();

            let mut joltage_nums: Vec<i32> = vec![];
            let mut rem_count = num_len - 12;

            for i in numbers {
                while joltage_nums.len() > 0 && joltage_nums[joltage_nums.len() - 1] < i {
                    if rem_count == 0 {
                        break;
                    }
                    joltage_nums.pop();
                    rem_count -= 1;
                }
                joltage_nums.push(i);
            }

            curr_max_joltage += joltage_nums[0..12].iter().map(|n| n.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
        }

        curr_max_joltage
    }
}