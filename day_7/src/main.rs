use std::io::Error;
use std::fs;

fn read_vector_from_file() -> Vec<i32> {

    fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i32>>()
}


fn part_1(input: &Vec<i32>) -> i32 {

    let mut min_cost: i32 = 1 << 30;
    let mut min_cost_pos: i32 = 0;
    let mut running_sum: i32 = 0;

    for i in 0..input.iter().max().unwrap()+1 {

        for crab in input {

            running_sum += (*crab - i).abs();
        }
        if running_sum < min_cost {
            
            min_cost = running_sum;
            min_cost_pos = i;
        }
        running_sum = 0;
    }

    println!("{}", min_cost_pos);

    min_cost
}


fn part_2(input: &Vec<i32>) -> i32 {

    let mut min_cost: i32 = 1 << 30;
    let mut min_cost_pos: i32 = 0;
    let mut running_sum: i32 = 0;

    for i in 0..input.iter().max().unwrap()+1 {

        for crab in input {

            let distance: i32 = (*crab -i).abs();

            running_sum += distance*(distance+1)/2;
        }
        if running_sum < min_cost {
            
            min_cost = running_sum;
            min_cost_pos = i;
        }
        running_sum = 0;
    }

    println!("{}", min_cost_pos);

    min_cost
}


fn main() -> Result<(), Error> {

    let input = read_vector_from_file();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}

