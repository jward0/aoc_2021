extern crate itertools; // 0.7.8

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn strings_from_file<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn part_1(input: &Vec<String>) -> Result<i64, Error> {

    let mut horizontal_position = 0;
    let mut depth = 0;

    for line in input.iter() {
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "forward" => horizontal_position += v[1].parse::<i64>().unwrap(),
            "up" => depth -= v[1].parse::<i64>().unwrap(),
            "down" => depth += v[1].parse::<i64>().unwrap(),
            _ => depth += 0,
        }

    }

    let product = horizontal_position * depth;
    println!("{}", product);

    Ok(product)
}


fn part_2(input: &Vec<String>) -> Result<i64, Error> {

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.iter() {
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "forward" => {
                horizontal_position += v[1].parse::<i64>().unwrap();
                depth += aim * v[1].parse::<i64>().unwrap();
            },
            "up" => aim -= v[1].parse::<i64>().unwrap(),
            "down" => aim += v[1].parse::<i64>().unwrap(),
            _ => depth += 0,
        }

    }

    let product = horizontal_position * depth;
    println!("{}", product);

    Ok(product)
}


fn main() -> Result<(), Error> {

    let input = strings_from_file(File::open("input.txt")?);

    part_1(&input).expect("Warning: Part 1 failed");

    part_2(&input).expect("Warning: Part 2 failed");

    Ok(())
}