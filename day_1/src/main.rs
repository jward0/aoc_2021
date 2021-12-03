extern crate itertools; // 0.7.8

use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read},
};


fn ints_from_file<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            );
    }
    Ok(v)
}


fn part_1(input: &Vec<i64>) -> Result<i64, Error> {

    let mut count = 0;

    for (a, b) in input.iter().tuple_windows() {
        if b > a {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(count)
}


fn part_2(input: &Vec<i64>) -> Result<i64, Error> {

    let mut count = 0;

    for (a, b, c, d) in input.iter().tuple_windows() {
        if b + c + d > a + b + c {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(count)
}


fn main() -> Result<(), Error> {

    let input = ints_from_file(File::open("input.txt")?)?;

    part_1(&input).expect("Warning: Part 1 failed");

    part_2(&input).expect("Warning: Part 2 failed");

    Ok(())
}
