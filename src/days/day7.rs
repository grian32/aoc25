use std::collections::HashSet;
use std::fs;
use crate::days::day::Day;

pub struct Day7 {

}

impl Day<Vec<Vec<u8>>, i64> for Day7 {
    fn get_input(&self) -> Vec<Vec<u8>>{
        let file = fs::read_to_string("inputs/day7.txt").expect("read input");

        file.lines().collect::<Vec<&str>>().iter().map(|x|x.bytes().collect::<Vec<u8>>()).collect()
    }

    fn part1(&self, input: &Vec<Vec<u8>>) -> i64 {
        let mut x = 0;
        let y = input[0].iter().position(|x| *x == b'S').unwrap();

        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while input[x][y] != b'^' {
            x += 1;
        }

        stack.push((x, y));
        while !stack.is_empty() {
            let curr_node = stack.pop().unwrap();
            if visited.contains(&curr_node) {
                continue;
            }
            visited.insert(curr_node);

            let mut left_search_pos = curr_node;
            let mut right_search_pos = curr_node;
            left_search_pos.1 = left_search_pos.1 - 1;
            right_search_pos.1 = right_search_pos.1 + 1;

            if left_search_pos.1 < input[0].len() {
                while left_search_pos.0 < input.len() && input[left_search_pos.0][left_search_pos.1] != b'^' {
                    left_search_pos.0 += 1;
                }

                if left_search_pos.0 < input.len() && left_search_pos.1 < input[0].len() && !visited.contains(&left_search_pos) {
                    stack.push((left_search_pos.0, left_search_pos.1));
                }
            }

            if right_search_pos.1 < input[0].len() {
                while right_search_pos.0 < input.len() && input[right_search_pos.0][right_search_pos.1] != b'^' {
                    right_search_pos.0 += 1;
                }

                if right_search_pos.0 < input.len() && right_search_pos.1 < input[0].len() && !visited.contains(&right_search_pos){
                    stack.push((right_search_pos.0, right_search_pos.1));
                }
            }
        }

        visited.len() as i64
    }

    fn part2(&self, input: &Vec<Vec<u8>>) -> i64 {
        0
    }
}