use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").expect("read input");

    let mut dial_pos = 50;
    let mut sum = 0;

    for line in input.lines()  {
        let direction = line.chars().nth(0).unwrap();
        let n: i32 = *&line[1..].parse().unwrap();
        match direction {
            'L' => {
                dial_pos -= n;
            }
            'R' => {
                dial_pos += n;
            }
            _ => { println!("invalid first char") }
        }

        dial_pos = dial_pos.rem_euclid(100);

        if dial_pos == 0 {
            sum += 1;
        }
    }

    println!("p1: {}", sum);

    let mut dial_pos_p2: i32 = 50;
    let mut sum_p2 = 0;

    for line in input.lines()  {
        let direction = line.chars().nth(0).unwrap();
        let n: i32 = *&line[1..].parse().unwrap();

        for _ in 0..n {
            match direction {
                'L' => dial_pos_p2 -= 1,
                'R' => dial_pos_p2 += 1,
                _ => { println!("invalid first char") }
            }

            dial_pos_p2 = dial_pos_p2.rem_euclid(100);

            if dial_pos_p2 == 0 {
                sum_p2 += 1
            }
        }

    }

    println!("p2: {}", sum_p2)
}
