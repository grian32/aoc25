use std::fs;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day4 {

}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // top-left
    (-1,  0), // top
    (-1,  1), // top-right
    ( 0, -1), // left
    ( 0,  1), // right
    ( 1, -1), // bottom-left
    ( 1,  0), // bottom
    ( 1,  1), // bottom-right
];

impl Day<Vec<Vec<char>>, i32> for Day4 {
    fn get_input(&self) -> Vec<Vec<char>>{
        let file = fs::read_to_string("inputs/day4.txt").expect("read input");
        file.lines().map(|x| x.chars().collect()).collect()
    }

    fn part1(&self, input: &Vec<Vec<char>>) -> i32 {
        let grid_len = input.len() as i32;
        let grid_line_len = input[0].len() as i32;
        let mut valid_rolls = 0;

        for i in 0..grid_len{
            for j in 0..grid_line_len {
                let mut roll_count = 0;
                if input[i as usize][j as usize] == '.' {
                    continue;
                }
                for (x, y) in DIRECTIONS {
                    let dx = i + x;
                    let dy = j + y;
                    if dx >= 0 && dx < grid_len && dy >= 0 && dy < grid_line_len {
                        if input[dx as usize][dy as usize] == '@' {
                            roll_count += 1;
                        }
                    }
                }
                if roll_count < 4 {
                    valid_rolls += 1;
                }
            }
        }

        valid_rolls
    }

    fn part2(&self, input: &Vec<Vec<char>>) -> i32 {
        let mut grid = input.to_vec();
        let grid_len = grid.len() as i32;
        let grid_line_len = grid[0].len() as i32;
        let mut total_valid_rolls = 0;

        loop {
            let mut curr_removed_rolls = 0;
            for i in 0..grid_len {
                for j in 0..grid_line_len {
                    let mut roll_count = 0;
                    if grid[i as usize][j as usize] == '.' {
                        continue;
                    }
                    for (x, y) in DIRECTIONS {
                        let dx = i + x;
                        let dy = j + y;
                        if dx >= 0 && dx < grid_len && dy >= 0 && dy < grid_line_len {
                            if grid[dx as usize][dy as usize] == '@' {
                                roll_count += 1;
                            }
                        }
                    }
                    if roll_count < 4 {
                        grid[i as usize][j as usize] = '.';
                        curr_removed_rolls += 1;
                        total_valid_rolls += 1;
                    }
                }
            }
            if curr_removed_rolls == 0 {
                break;
            }
        }
        total_valid_rolls
    }
}
