use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn generate_map() -> Vec<Vec<char>> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    br.lines().map(|l| l.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}


fn part_1(input: &mut Vec<Vec<char>>) -> u32 {

    let mut steps: u32 = 0;

    loop {
        let mut moved = false;
        let mut input_copy = input.clone();

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == '>' {
                    if j < input[0].len() - 1 {
                        if input[i][j+1] == '.' {
                            input_copy[i][j+1] = '>';
                            input_copy[i][j] = '.';
                            moved = true;
                        }
                    }
                    if j == input[0].len() - 1 && input[i][0] == '.' {
                        input_copy[i][0] = '>';
                        input_copy[i][j] = '.';
                        moved = true;
                    }
                }
            }
        }

        *input = input_copy;
        let mut input_copy = input.clone();

        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == 'v' {
                        if i < input.len() - 1 {
                            if input[i+1][j] == '.' {
                                input_copy[i+1][j] = 'v';
                                input_copy[i][j] = '.';
                                moved = true;
                            }
                        }
                    if i == input.len() - 1 && input[0][j] == '.' {
                        input_copy[0][j] = 'v';
                        input_copy[i][j] = '.';
                        moved = true;
                    }
                    }              
            }
        }

        *input = input_copy;

        if !moved {
            return steps + 1;
        }
        else {
            steps += 1;
            //println!("steps: {}", steps);
            //println!("moved: {}", moved);
            //for line in input.iter() {
            //    println!("{}{}{}{}{}{}{}{}{}{}", line[0], line[1], line[2], line[3], line[4], line[5], line[6], line[7], line[8], line[9]);
            //}
            //println!("---------");
        }
    }

    0
}


fn main() -> Result<(), Error> {

    let mut input = generate_map();

    println!("{}", part_1(&mut input));

    Ok(())
}