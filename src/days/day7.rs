use std::collections::HashSet;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::days::day::Day;

pub struct Day7 {

}

fn find_next_left_right(curr_node: (usize, usize), input: &Vec<Vec<u8>>) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut ret: (Option<(usize, usize)>, Option<(usize, usize)>) = (None, None);
    let left_search = curr_node.1 - 1;
    let right_search = curr_node.1 + 1;

    let input_len = input.len();
    let input_line_len = input[0].len();

    if left_search < input_line_len {
        let mut search = curr_node.0;
        while search < input_len && input[search][left_search] != b'^' {
            search += 1;
        }

        if search < input_len {
            ret.0 = Some((search, left_search));
        }
    }

    if right_search < input_line_len {
        let mut search = curr_node.0;
        while search < input_len && input[search][right_search] != b'^' {
            search += 1;
        }

        if search < input_len {
            ret.1 = Some((search, right_search));
        }
    }

    ret
}

fn dfs(curr_node: (usize, usize),
       input: &Vec<Vec<u8>>,
       left_right: &Vec<Vec<(Option<(usize, usize)>, Option<(usize, usize)>)>>,
       paths: &mut Vec<Vec<i64>>
) -> i64 {
    let (left, right) = left_right[curr_node.0][curr_node.1];

    if paths[curr_node.0][curr_node.1] != -1 {
        return paths[curr_node.0][curr_node.1];
    }

    let curr_col = curr_node.1;
    let mut total: i64 = 0;

    if left.is_none() && curr_col > 0 {
        total += 1;
    }
    if right.is_none() && curr_col + 1 < input[0].len() {
        total += 1;
    }

    if let Some(left) = left {
        total += dfs(left, input, left_right, paths);
    }
    if let Some(right) = right {
        total += dfs(right, input, left_right, paths);
    }

    paths[curr_node.0][curr_node.1] = total;
    total
}

impl Day<Vec<Vec<u8>>, i64> for Day7 {
    fn get_input(&self) -> Vec<Vec<u8>> {
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
        let mut x = 0;
        let y = input[0].iter().position(|x| *x == b'S').unwrap();

        let mut paths: Vec<Vec<i64>> = vec![vec![-1; input[0].len()]; input.len()];

        while input[x][y] != b'^' {
            x += 1;
        }

        let mut left_right = vec![vec![(None, None); input[0].len()]; input.len() + 1];

        for x in 0..input.len() {
            for y in 0..input[0].len() {
                left_right[x][y] = find_next_left_right((x, y), input)
            }
        }

        dfs((x, y), input, &left_right, &mut paths)
    }
}