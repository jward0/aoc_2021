use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_vector_of_vectors_from_file() -> Vec<Vec<u32>> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    br.lines()
        .map(|l| l.unwrap()
                  .chars()
                  .map(|d| d.to_digit(10)
                            .unwrap())
                            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}


#[derive(Copy, Clone)]
struct Octopus {

    energy: u32,
    flashed: bool
}


impl Octopus {

    fn new(energy: u32) -> Self {

        Octopus{energy: energy, flashed: false}
    }

    fn increase_energy(&mut self) -> bool {

        self.energy += 1;
        if (self.energy > 9) && (!self.flashed) {
            //println!("Flash!");
            self.flashed = true;
            return true;
        }

        false
    }
}


fn flash(octopuses: &mut Vec<Vec<Octopus>>, indices: (usize, usize), flash_tracker: &mut u32) -> () {

    let i = indices.0;
    let j = indices.1;

    let i_range: Vec<usize>; // = vec![];
    let j_range: Vec<usize>; // = vec![];

    if i == 0 {
        i_range = [0, 1].to_vec();
    } else if i == octopuses.len()-1 {
        i_range = [octopuses.len()-2, i].to_vec();
    } else {
        i_range = [i-1, i, i+1].to_vec();
    }

    if j == 0 {
        j_range = [0, 1].to_vec();
    } else if j == octopuses[0].len()-1 {
        j_range = [octopuses[0].len()-2, j].to_vec();
    } else {
        j_range = [j-1, j, j+1].to_vec();
    }

    for i_ndx in i_range.iter() {
        for j_ndx in j_range.iter() {

            if octopuses[*i_ndx][*j_ndx].increase_energy() {
                *flash_tracker += 1;
                flash(octopuses, (*i_ndx, *j_ndx), flash_tracker);
            }
        }
    }
}


fn part_1(input: &Vec<Vec<u32>>) -> u32 {

    let mut octopuses: Vec<Vec<Octopus>> = input.iter()
                                                .map(|line| line.iter()
                                                                .map(|val| Octopus::new(*val))
                                                                .collect())
                                                .collect();

    let mut flash_tracker: u32 = 0;

    for step in 0..100 {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if octopuses[i][j].increase_energy() {
                    flash_tracker += 1;
                    flash(&mut octopuses, (i, j), &mut flash_tracker);
                }
            }
        }

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if octopuses[i][j].flashed {
                    octopuses[i][j].flashed = false;
                    octopuses[i][j].energy = 0;
                }
            }
        }
    }

    flash_tracker
}


fn part_2(input: &Vec<Vec<u32>>) -> u32 {

    let mut octopuses: Vec<Vec<Octopus>> = input.iter()
                                                .map(|line| line.iter()
                                                                .map(|val| Octopus::new(*val))
                                                                .collect())
                                                .collect();

    let mut flash_tracker: u32 = 0;

    let synchronised_step: u32;

    let mut step: u32 = 1;

    loop {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if octopuses[i][j].increase_energy() {
                    flash_tracker += 1;
                    flash(&mut octopuses, (i, j), &mut flash_tracker);
                }
            }
        }

        let mut flash_counter: u32 = 0;

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if octopuses[i][j].flashed {
                    flash_counter += 1;
                    octopuses[i][j].flashed = false;
                    octopuses[i][j].energy = 0;
                }
            }
        }

        if flash_counter == 100 {
            synchronised_step = step;
            break;
        }

        step += 1;

    }

    synchronised_step
}


fn main() -> Result<(), Error> {

    let input = read_vector_of_vectors_from_file();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}
