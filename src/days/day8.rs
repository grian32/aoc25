use std::fs;
use crate::days::day::Day;

pub struct Day8 {
    
}

#[derive(Debug)]
#[derive(Clone)]
pub struct JunctionBox {
    coord: [i64; 3],
    closest_box: Option<usize>,
    distance: Option<f64>
}

impl JunctionBox {
    fn distance_to(self, other: JunctionBox) -> f64 {
        let dx = self.coord[0] - other.coord[0];
        let dy = self.coord[1] - other.coord[1];
        let dz = self.coord[2] - other.coord[2];

        ((dx*dx + dy*dy+dz*dz) as f64).sqrt()
    }
}

impl Day<Vec<JunctionBox>, i64> for Day8 {
    fn get_input(&self) -> Vec<JunctionBox> {
        let file = fs::read_to_string("inputs/day8.txt").expect("read input");
        let mut vec = Vec::new();

        for i in file.lines() {
            let coords = i.split(",").map(|x|x.parse::<i64>().unwrap()).collect::<Vec<_>>();
            vec.push(JunctionBox {
                coord: [coords[0], coords[1], coords[2]],
                closest_box: None,
                distance: None,
            })
        }

        vec
    }

    fn part1(&self, input: &mut Vec<JunctionBox>) -> i64 {
        let mut cloned_input = input.clone();
        let len = cloned_input.len();

        for i in 0..len {
            let mut curr_min_distance = f64::MAX;
            let mut curr_closest_box: usize = 0;
            for j in 0..len {
                if i != j {
                    let dist = cloned_input[i].clone().distance_to(cloned_input[j].clone());
                    if dist < curr_min_distance {
                        curr_min_distance = dist;
                        curr_closest_box = j;
                    }
                }
            }
            cloned_input[i].closest_box = Some(curr_closest_box);
            cloned_input[i].distance = Some(curr_min_distance);
        }

        for i in 0..len{
            print!("{:?}", cloned_input[i].coord);
            print!(" closest to ");
            print!("{:?}", cloned_input[cloned_input[i].closest_box.unwrap()].coord);
            println!();
        }


        0
    }

    fn part2(&self, input: &mut Vec<JunctionBox>) -> i64 {
        return 0;
    }
}