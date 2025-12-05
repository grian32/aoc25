use std::collections::HashSet;
use std::fs;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day5 {

}

pub struct Ingredients {
    ranges: Vec<(i64, i64)>,
    available: HashSet<i64>
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for (start, end) in ranges {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

impl Day<Ingredients, i64> for Day5 {
    fn get_input(&self) -> Ingredients {
        let file = fs::read_to_string("inputs/day5.txt").expect("read input");
        let split: Vec<_> = file.split("\n\n").collect();
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let mut available: HashSet<i64> = HashSet::new();

        for i in split[0].lines() {
            let split_range: Vec<_> = i.split("-").collect();
            let start = split_range[0].parse::<i64>().unwrap();
            let end = split_range[1].parse::<i64>().unwrap();
            ranges.push((start, end));
        }

        for i in split[1].lines() {
            let num = i.parse::<i64>().unwrap();
            available.insert(num);
        }

        let merged_ranges = merge_ranges(ranges.clone());
        Ingredients{
            ranges: merged_ranges,
            available: available,
        }
    }

    fn part1(&self, input: &Ingredients) -> i64 {
        let mut good_ingredients: i32 = 0;
        let ranges = &input.ranges.to_vec();
        let available = input.available.to_owned();


        for i in available {
            for (start, end) in ranges {
                let range = start..=end;

                if range.contains(&&i) {
                    good_ingredients += 1;
                    break
                }
            }
        }

        good_ingredients as i64
    }

    fn part2(&self, input: &Ingredients) -> i64 {
        let mut good_ingredients: i64 = 0;
        let ranges = &input.ranges.to_vec();

        for (start, end) in ranges {
            good_ingredients += end - start + 1;
        }

        good_ingredients
    }
}