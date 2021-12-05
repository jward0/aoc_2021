use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

extern crate transpose; // 0.2.1
extern crate itertools; // 0.7.8


fn strings_from_file<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn part_1(input: &Vec<String>) -> Result<i32, Error> {

    let binary_length = input[0].len();

    let max_binary_digit = binary_length as i32;

    let mut gamma = 0;
    let mut epsilon = 0;

    let mut binary_digits : Vec<i32> = Vec::new();

    for _ in 0..max_binary_digit {
        binary_digits.push(0);
    }

    for line in input.iter() {

        let mut int_val = i32::from_str_radix(line, 2).unwrap();

        for n in (0..max_binary_digit).rev() {
            let exponent_check = i32::pow(2, n as u32) - 1;
            if int_val > exponent_check {
                binary_digits[n as usize] += 1;
                int_val -= exponent_check + 1;
            }
        }

    }    

    for (i, val) in binary_digits.iter().enumerate() {

        if val > &(input.len() as i32 / 2) {
            gamma += i32::pow(2, i as u32) as i32;
        } else {
            epsilon += i32::pow(2, i as u32) as i32;
        }
    }

    let product = gamma * epsilon;
    println!("{}", product);

    Ok(product)
}


fn part_2(input: &Vec<String>) -> Result<i32, Error> {

    let mut input_as_ints : Vec<i32> = Vec::new();

    for line in input.iter() {
        input_as_ints.push(i32::from_str_radix(line, 2).unwrap());
    }

    let exponent = input[0].len() as i32 - 1;

    let (mut more_vec, mut less_vec) = split_vector(&input_as_ints, &exponent);

    for i in (0..exponent).rev() {

        if more_vec.len() > 1 {
            more_vec = split_vector(&more_vec, &i).0;
        }

        if less_vec.len() > 1 {
            less_vec = split_vector(&less_vec, &i).1;
        }

    }

    let product = more_vec[0] * less_vec[0];

    println!("{}", product);

    Ok(product)
    
}


fn split_vector(input: &Vec<i32>, exponent: &i32) -> (Vec<i32>, Vec<i32>) {

    let mut one_vec : Vec<i32> = Vec::new();
    let mut zero_vec : Vec<i32> = Vec::new();

    let mask = i32::pow(2, (exponent + 1) as u32) - 1;

    for item in input.iter() {
        if item & mask > i32::pow(2, *exponent as u32) - 1 {
            one_vec.push(*item);
        } else {
            zero_vec.push(*item);
        }
    }

    if one_vec.len() >= zero_vec.len() {
        (one_vec, zero_vec)
    } else {
        (zero_vec, one_vec)
    }
}


fn main() -> Result<(), Error> {

    let input = strings_from_file(File::open("input.txt")?);

    part_1(&input).expect("Warning: Part 1 failed");

    part_2(&input).expect("Warning: Part 2 failed");

    Ok(())
}