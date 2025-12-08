use std::fs;
use fancy_regex::Regex;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day2 {

}

pub struct NumRange {
    start: i64,
    end: i64
}

impl Day<Vec<NumRange>, i64> for Day2 {
    fn get_input(&self) -> Vec<NumRange> {
        let file = fs::read_to_string("inputs/day2.txt").expect("read input");
        let mut vec: Vec<NumRange> = Vec::new();
        for line in file.split(",") {
            let range: Vec<_> = line.split("-").collect();
            let start: i64 = range[0].parse().unwrap();
            let end: i64 = range[1].parse().unwrap();

            vec.push(NumRange{
                start,
                end
            })
        }

        vec
    }

    fn part1(&self, input: &mut Vec<NumRange>) -> i64 {
        let mut bad_number_sum: i64 = 0;

        for line in input {
            for i in line.start..=line.end {
                let string = i.to_string();
                let (begin, end) = string.split_at(string.len() / 2);
                if begin == end {
                    bad_number_sum += i;
                }
            }
        }

        bad_number_sum
    }

    fn part2(&self, input: &mut Vec<NumRange>) -> i64 {
        let re = Regex::new(r"^(\d+)\1+$").unwrap();

        let mut bad_number_sum: i64 = 0;

        for line in input {
            for i in line.start..=line.end {
                let m = re.find_iter(&*i.to_string()).count();
                if m >= 1 {
                    bad_number_sum += i;
                }
            }
        }

        bad_number_sum
    }
}