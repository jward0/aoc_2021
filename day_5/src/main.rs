use regex::Regex;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn vent_lines_from_file<R: Read>(io: R) -> Vec<VentLine> {
    let br = BufReader::new(io);
    let lines = br.lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect::<Vec<String>>();

    let mut vent_lines_vector: Vec<VentLine> = Vec::new();

    let re = Regex::new(r"\d+").unwrap();

    for line in lines {

        assert!(re.is_match(&line));
        let axis_bounds: Vec<u32> = re.find_iter(&line)
                                        .filter_map(|digits| digits.as_str().parse().ok())
                                        .collect();

        vent_lines_vector.push(VentLine::new(axis_bounds));
    }

    vent_lines_vector
}


struct VentLine {

    x_vals: Vec<u32>,
    y_vals: Vec<u32>,
    is_straight: bool
}

trait VentLineTrait {

    fn new (axis_bounds: Vec<u32>) -> Self;
}

impl VentLineTrait for VentLine {

    fn new (axis_bounds: Vec<u32>) -> Self {

        let is_straight: bool;

        if (axis_bounds[0] == axis_bounds[2]) || (axis_bounds[1] == axis_bounds[3]) {
            is_straight = true;
        } else {
            is_straight = false;
        }

        let x_start = axis_bounds[0];
        let y_start = axis_bounds[1];
        let x_end = axis_bounds[2];
        let y_end = axis_bounds[3];

        let x_vec: Vec<u32>; // = Vec::new();
        let y_vec: Vec<u32>; // = Vec::new();

        if x_start <= x_end {
            x_vec = (x_start..x_end+1).collect();
        } else {
            x_vec = (x_end..x_start+1).rev().collect();
        }

        if y_start <= y_end {
            y_vec = (y_start..y_end+1).collect();
        } else {
            y_vec = (y_end..y_start+1).rev().collect();
        }

        VentLine{x_vals: x_vec,
                 y_vals: y_vec,
                 is_straight: is_straight}
    }
}


fn part_1(input: &Vec<VentLine>) -> Result<i32, Error> {

    let mut occupancy_arr = vec![[0u32; 1000]; 1000];
    let mut running_total = 0;

    for vent_line in input {
        if vent_line.is_straight {
            for i in &vent_line.x_vals {
                for j in &vent_line.y_vals {
                    occupancy_arr[*i as usize][*j as usize] += 1;
                }
            }
        }
    }

    for i in 0..1000 {
        for j in 0..1000 {
            if occupancy_arr[i][j] > 1 {
                running_total += 1;
            }
        }
    }

    println!("{}", running_total);

    Ok(running_total)
}

fn part_2(input: &Vec<VentLine>) -> Result<i32, Error> {

    let mut occupancy_arr = vec![[0u32; 1000]; 1000];
    let mut running_total = 0;

    for vent_line in input {
        if vent_line.is_straight {
            for i in &vent_line.x_vals {
                for j in &vent_line.y_vals {
                    occupancy_arr[*i as usize][*j as usize] += 1;
                }
            }
        } else {
            let coord_zip: Vec<(&u32, &u32)> = vent_line.x_vals.iter().zip(vent_line.y_vals.iter()).collect();
            for (i, j) in coord_zip {
                occupancy_arr[*i as usize][*j as usize] += 1;
            }
        }
    }

    for i in 0..1000 {
        for j in 0..1000 {
            if occupancy_arr[i][j] > 1 {
                running_total += 1;
            }
        }
    }

    println!("{}", running_total);

    Ok(running_total)
}


fn main() -> Result<(), Error> {

    let input = vent_lines_from_file(File::open("input.txt")?);

    part_1(&input).expect("Warning: Part 1 failed");

    part_2(&input).expect("Warning: Part 2 failed");

    Ok(())
}
