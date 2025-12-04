use std::fs;
use crate::days::day::Day;

#[allow(dead_code)]
pub struct Day4 {

}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // top-left
    (-1,  0), // top
    (-1,  1), // top-right
    ( 0, -1), // left
    ( 0,  1), // right
    ( 1, -1), // bottom-left
    ( 1,  0), // bottom
    ( 1,  1), // bottom-right
];

impl Day<Vec<Vec<u8>>, i32> for Day4 {
    fn get_input(&self) -> Vec<Vec<u8>>{
        let file = fs::read_to_string("inputs/day4.txt").expect("read input");
        file.lines().map(|x| x.as_bytes().to_vec()).collect()
    }

    fn part1(&self, input: &Vec<Vec<u8>>) -> i32 {
        let grid_len = input.len();
        let grid_line_len = input[0].len();
        let grid_len_i= input.len() as isize;
        let grid_line_len_i= input[0].len() as isize;
        let mut valid_rolls = 0;

        for i in 0..grid_len{
            for j in 0..grid_line_len {
                let mut roll_count = 0;
                if input[i][j] == b'.' {
                    continue;
                }
                for (x, y) in DIRECTIONS {
                    let dx = i as isize + x;
                    let dy = j as isize + y;
                    if dx >= 0 && dx < grid_len_i && dy >= 0 && dy < grid_line_len_i{
                        if input[dx as usize][dy as usize] == b'@' {
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

    fn part2(&self, input: &Vec<Vec<u8>>) -> i32 {
        let mut grid = input.to_vec();
        let grid_len= grid.len();
        let grid_line_len= grid[0].len();
        let grid_len_i= grid.len() as isize;
        let grid_line_len_i= grid[0].len() as isize;
        let mut total_valid_rolls = 0;

        loop {
            let mut curr_removed_rolls = 0;
            for i in 0..grid_len {
                'outer: for j in 0..grid_line_len {
                    let mut roll_count = 0;
                    if grid[i as usize][j as usize] == b'.' {
                        continue;
                    }
                    for (x, y) in DIRECTIONS {
                        let dx = i as isize + x;
                        let dy = j as isize + y;
                        if dx >= 0 && dx < grid_len_i && dy >= 0 && dy < grid_line_len_i {
                            if grid[dx as usize][dy as usize] == b'@' {
                                roll_count += 1;
                            }
                        }
                        if roll_count >= 4 {
                            continue 'outer
                        }
                    }
                    if roll_count < 4 {
                        grid[i as usize][j as usize] = b'.';
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
