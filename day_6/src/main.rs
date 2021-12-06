use std::io::Error;
use std::fs;

fn read_vector_from_file() -> Vec<u32> {

    fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u32>>()
}


fn part_1(input: &Vec<u32>, iteration_count: u32) -> u32 {

    let mut fish_vector = input.to_vec();

    for _i in 0..iteration_count {

        let mut fish_to_add = 0;

        for j in 0..fish_vector.len() {

            if fish_vector[j] == 0 {

                fish_to_add += 1;
                fish_vector[j] = 6;
            }
            else {

                fish_vector[j] -= 1;
            }
        }
        for _k in 0..fish_to_add {
            fish_vector.push(8);
        }

    }

    fish_vector.len() as u32
}


fn part_2(input: &Vec<u32>, iteration_count: u32) -> u64 {

    let mut population_vec: Vec<u64> = vec![0; 9];

    for fish in input {

        population_vec[*fish as usize] += 1;
    }

    for i in 0..iteration_count {

        let zero_population = population_vec[0];
        
        for j in 0..8 {

            population_vec[j] = population_vec[j+1];
        }
        population_vec[6] += zero_population;
        population_vec[8] = zero_population; 

    }
    
    population_vec.iter().sum()
}


fn main() -> Result<(), Error> {

    let input = read_vector_from_file();

    println!("{}", part_1(&input, 80));

    println!("{}", part_2(&input, 256));

    Ok(())
}
