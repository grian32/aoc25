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

        let input_len = input.len();
        let input_line_len = input[0].len();

        stack.push((x, y));
        while !stack.is_empty() {
            let curr_node = stack.pop().unwrap();
            if visited.contains(&curr_node) {
                continue;
            }
            visited.insert(curr_node);

            let left_search = curr_node.1 - 1;
            let right_search = curr_node.1 + 1;

            if left_search < input_line_len {
                let mut search = curr_node.0;
                while search < input_len && input[search][left_search] != b'^' {
                    search += 1;
                }

                if search < input_len && !visited.contains(&(search, left_search)) {
                    stack.push((search, left_search));
                }
            }

            if right_search < input_line_len {
                let mut search = curr_node.0;
                while search < input_len && input[search][right_search] != b'^' {
                    search += 1;
                }

                if search < input_len && !visited.contains(&(search, right_search)){
                    stack.push((search, right_search));
                }
            }
        }

        visited.len() as i64
    }

    fn part2(&self, input: &Vec<Vec<u8>>) -> i64 {
        0
    }
}