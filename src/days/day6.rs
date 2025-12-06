use std::fs;
use crate::days::day::Day;

pub struct Day6 {

}

pub struct MathProblem {
    numbers: Vec<i64>,
    p2numbers: Vec<i64>,
    op: char
}

fn transpose_right(vec: Vec<String>) -> Vec<String> {
    let cols = vec[0].len();

    let mut new = vec![String::new(); cols];

    for row in vec {
        for (i, ch) in row.chars().enumerate() {
            new[i].push(ch);
        }
    }

    new
}

impl Day<Vec<MathProblem>, i64> for Day6 {
    fn get_input(&self) -> Vec<MathProblem> {
        let file = fs::read_to_string("inputs/day6.txt").expect("read input");
        let lines: Vec<_> = file.lines().collect();
        let grid: Vec<_> = lines.
            iter().
            map(|x| x.
                split(" ").
                map(|y| y.trim()).
                filter(|x| !x.is_empty()).
                collect::<Vec<_>>()
            ).
            collect();

        let op_line = lines[lines.len() - 1];
        let numbers_line = &lines[..lines.len() - 1];
        let mut col_positions = vec![];
        for (i, ch) in op_line.chars().enumerate() {
            if ch != ' ' {
                col_positions.push(i);
            }
        }

        let max_num_line_len = numbers_line.iter().map(|x| x.len()).max().unwrap();
        let mut untrimmed_grid: Vec<Vec<String>> = Vec::new();
        for line in numbers_line {
            let mut row = Vec::new();

            for i in 0..col_positions.len() - 1 {
                let start = col_positions[i];
                let end = col_positions[i + 1];
                let cell = &line[start..(end.min(line.len()) - 1)];
                row.push(cell.to_string());
            }

            let start = col_positions[col_positions.len() - 1];
            let mut cell = line[start..line.len()].to_string();
            // pad to deal with input
            if line.len() < max_num_line_len {
                let to_pad = max_num_line_len - line.len();
                for _ in 0..to_pad {
                    cell += " ";
                }
            }
            row.push(cell);
            untrimmed_grid.push(row);
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut vec: Vec<MathProblem> = Vec::new();

        for col in 0..cols {
            let mut numbers: Vec<i64> = Vec::new();
            let mut p2numbers: Vec<String> = Vec::new();
            let mut op: char = 'a';
            for row in 0..rows {
                if row != rows - 1 {
                    numbers.push(grid[row][col].parse().unwrap());
                    p2numbers.push(untrimmed_grid[row][col].clone());
                } else {
                    op = grid[row][col].parse().unwrap()
                }
            }

            p2numbers = transpose_right(p2numbers);
            let converted = p2numbers.iter().map( | x | x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

            vec.push(MathProblem {
                numbers,
                p2numbers: converted,
                op
            })
        }


        vec
    }

    fn part1(&self, input: &Vec<MathProblem>) -> i64 {
        let mut total: i64 = 0;

        for i in input {
            let mut curr = i.numbers[0];
            match i.op {
                '+' => {
                    for n in 1..i.numbers.len() {
                        curr += i.numbers[n];
                    }
                }
                '*' => {
                    for n in 1..i.numbers.len() {
                        curr *= i.numbers[n];
                    }
                }
                _ => panic!("unknown operation"),
            }

            total += curr;
        }

        total
    }

    fn part2(&self, input: &Vec<MathProblem>) -> i64 {
        let mut total: i64 = 0;

        for i in input {
            let mut curr = i.p2numbers[0];
            match i.op {
                '+' => {
                    for n in 1..i.p2numbers.len() {
                        curr += i.p2numbers[n];
                    }
                }
                '*' => {
                    for n in 1..i.p2numbers.len() {
                        curr *= i.p2numbers[n];
                    }
                }
                _ => panic!("unknown operation"),
            }

            total += curr;
        }

        total
    }
}