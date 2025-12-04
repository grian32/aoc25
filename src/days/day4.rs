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

impl Day for Day4 {
    fn get_input(&self) -> String {
        fs::read_to_string("inputs/day4.txt").expect("read input")
    }

    fn part1(&self) {
        let input = self.get_input();
        let grid: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
        let grid_len = grid.len() as i32;
        let grid_line_len = grid[0].len() as i32;
        let mut valid_rolls = 0;

        for i in 0..grid_len{
            for j in 0..grid_line_len {
                let mut roll_count = 0;
                if grid[i as usize][j as usize] == '.' {
                    continue;
                }
                for (x, y) in DIRECTIONS {
                    let dx = i + x;
                    let dy = j + y;
                    if dx >= 0 && dx < grid_len && dy >= 0 && dy < grid_line_len {
                        if (grid[dx as usize][dy as usize] == '@') {
                            roll_count += 1;
                        }
                    }
                }
                if roll_count < 4 {
                    valid_rolls += 1;
                }
            }
        }

        println!("Day 4 Part 1: {}", valid_rolls);
    }

    fn part2(&self) {
        let input = self.get_input();
        let mut grid: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
        let grid_len = grid.len() as i32;
        let grid_line_len = grid[0].len() as i32;
        let mut total_valid_rolls = 0;

        while true {
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
                            if (grid[dx as usize][dy as usize] == '@') {
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
            if (curr_removed_rolls == 0) {
                break;
            }
        }
        println!("Day 4 Part 2: {}", total_valid_rolls)
    }
}
