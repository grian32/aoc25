use std::cmp::{max, min};
use std::fs;
use crate::days::day::Day;

pub struct Day9 {

}

fn area_rec(first: [i64; 2], second: [i64; 2]) -> i64 {
    ((second[0] - first[0]).abs() + 1) * ((second[1] - first[1]).abs() + 1)
}

impl Day<Vec<[i64; 2]>, i64> for Day9 {
    fn get_input(&self) -> Vec<[i64; 2]> {
        let file = fs::read_to_string("inputs/day9.txt").expect("read input");
        let mut vec = Vec::new();

        for i in file.lines() {
            let coords = i.split(",").map(|x|x.parse::<i64>().unwrap()).collect::<Vec<_>>();
            vec.push([coords[0], coords[1]])
        }

        vec
    }

    fn part1(&self, input: &mut Vec<[i64; 2]>) -> i64 {
        let mut curr_max_area: i64 = i64::MIN;

        for i in 0..input.len() {
            for j in 0..input.len() {
                if i != j {
                    let area = area_rec(input[i], input[j]);
                    if area > curr_max_area {
                        curr_max_area = area;
                    }
                }
            }
        }

       curr_max_area
    }

    fn part2(&self, input: &mut Vec<[i64; 2]>) -> i64 {
        // border size, level at which its at
        let mut x_borders: Vec<([i64; 2], i64)> = Vec::new();
        let mut y_borders: Vec<([i64; 2], i64)> = Vec::new();

        for i in 0..input.len() - 1 {
            let first = input[i];
            let second = input[i+1];

            if first[0] == second[0] {
                y_borders.push(([first[1], second[1]], first[0]));
            } else if first[1] == second[1] {
                x_borders.push(([first[0], second[0]], first[1]));
            }
        }

        let first = input[0];
        let last = input[input.len() - 1];
        if last[0] == first[0] {
            y_borders.push(([last[1], first[1]], last[0]));
        } else if first[1] == last[1] {
            x_borders.push(([last[0], first[0]], last[1]));

        }

        let mut grid = vec![vec!['.'; 20]; 20];

        for i in x_borders {
            for r in i.0[0]..i.0[1] {
                grid[i.1 as usize][r as usize] = '#';
            }
        }

        for i in y_borders {
            for r in i.0[0]..i.0[1] {
                grid[r as usize][i.1 as usize] = '#';
            }
        }

        for row in grid {
            println!("{:?}", row);
        }

        let mut max_area: i64 = i64::MIN;

        for i in 0..input.len() {
            for j in 0..input.len() {
                if i != j {
                    let first = input[i];
                    let second = input[j];

                    let min_x = min(first[0], second[0]);
                    let max_x = max(first[0], second[0]);
                    let min_y = min(first[1], second[1]);
                    let max_y = max(first[1], second[1]);

                    let mut rect_in_bounds = true;

                    // for i in &x_borders {
                    //     let y = i.1;
                    //     let xs = i.0;
                    //
                    //     if min_y <= y && y <= max_y && !(xs[0] <= min_x && max_x <= xs[1]) {
                    //         rect_in_bounds = false;
                    //     }
                    // }
                    //
                    //
                    // for i in &y_borders {
                    //     let x = i.1;
                    //     let ys = i.0;
                    //
                    //     if min_x <= x && x <= max_x && !(ys[0] <= min_y && max_y <= ys[1]) {
                    //         rect_in_bounds = false;
                    //     }
                    // }
                    //
                    if rect_in_bounds {
                        let area = area_rec(first, second);
                        if area > max_area {
                            max_area = area;
                        }
                    }
                }
            }
        }

        // println!("{:?}", x_borders);
        // println!("{:?}", y_borders);


        max_area
    }
}