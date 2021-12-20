use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_algorithm_and_image() -> (Vec<char>, Vec<Vec<char>>){

    let br = BufReader::new(File::open("input.txt").unwrap());

    let iter_lines = br.lines()
                       .into_iter()
                       .map(|l| l.unwrap())
                       .collect::<Vec<String>>();

    let algorithm = iter_lines[0].chars()
                                        .map(|c| match c {
                                            '.' => '0',
                                            '#' => '1',
                                            _ => unreachable!()})
                                        .collect::<Vec<char>>();

    let image = iter_lines[2..].iter().map(|l| l.chars()
                                                .map(|c| match c {
                                                    '.' => '0',
                                                    '#' => '1',
                                                    _ => unreachable!()})
                                                .collect::<Vec<char>>())
                                       .collect::<Vec<Vec<char>>>();

    (algorithm, image)
}


fn part_1(algorithm: &Vec<char>, image:&Vec<Vec<char>>) -> u32 {

    // Assemble copy of image padded with 5 0s in each direction

    let mut padded_image: Vec<Vec<char>> = vec![];

    let image_dimensions = [image.len(), image[0].len()];

    for _i in 0..5 {
        padded_image.push(vec!['0'; image_dimensions[1]+10]);
    }

    for i in 0..image_dimensions[0] {
        let mut i_vec: Vec<char> = vec!['0'; 5];
        for character in &image[i] {
            i_vec.push(*character);
        }
        for _j in 0..5 {
            i_vec.push('0');
        }
        padded_image.push(i_vec);
    }

    for _i in 0..5 {
        padded_image.push(vec!['0'; image_dimensions[1]+10]);
    }

    // Create copy of image, updated appropriately

    for _step in 0..2 {

        let mut new_image: Vec<Vec<char>> = vec![vec!['0'; padded_image.len()]; padded_image[0].len()];

        for i in 0..padded_image.len() {
            for j in 0..padded_image[0].len() {
                if i == 0 || j == 0 || i == padded_image.len()-1 || j == padded_image[0].len()-1 {
                    new_image[i][j] = match padded_image[i][j] {
                        '0' => algorithm[0],
                        '1' => *algorithm.last().unwrap(),
                        _ => unreachable!(),
                    }
                } else {
                    let mut subsection_string = String::new();
                    for sub_i in i-1..i+2 {
                        for sub_j in j-1..j+2 {
                            subsection_string.push(padded_image[sub_i][sub_j]);

                        }
                    }
                    new_image[i][j] = algorithm[usize::from_str_radix(&subsection_string, 2).unwrap()];
                }
            }
        }

        padded_image = new_image;
    }

    let lit_count: u32 = padded_image.iter()
                                     .map(|l| l.iter()
                                               .map(|c| match c {
                                                    '0' => 0,
                                                    '1' => 1,
                                                    _ => unreachable!()})
                                                .sum::<u32>())
                                     .sum::<u32>();

    lit_count
}


fn part_2(algorithm: &Vec<char>, image:&Vec<Vec<char>>) -> u32 {

    // Assemble copy of image padded with 55 0s in each direction

    let mut padded_image: Vec<Vec<char>> = vec![];

    let image_dimensions = [image.len(), image[0].len()];

    for _i in 0..55 {
        padded_image.push(vec!['0'; image_dimensions[1]+110]);
    }

    for i in 0..image_dimensions[0] {
        let mut i_vec: Vec<char> = vec!['0'; 55];
        for character in &image[i] {
            i_vec.push(*character);
        }
        for _j in 0..55 {
            i_vec.push('0');
        }
        padded_image.push(i_vec);
    }

    for _i in 0..55 {
        padded_image.push(vec!['0'; image_dimensions[1]+110]);
    }

    // Create copy of image, updated appropriately

    for _step in 0..50 {

        let mut new_image: Vec<Vec<char>> = vec![vec!['0'; padded_image.len()]; padded_image[0].len()];

        for i in 0..padded_image.len() {
            for j in 0..padded_image[0].len() {
                if i == 0 || j == 0 || i == padded_image.len()-1 || j == padded_image[0].len()-1 {
                    new_image[i][j] = match padded_image[i][j] {
                        '0' => algorithm[0],
                        '1' => *algorithm.last().unwrap(),
                        _ => unreachable!(),
                    }
                } else {
                    let mut subsection_string = String::new();
                    for sub_i in i-1..i+2 {
                        for sub_j in j-1..j+2 {
                            subsection_string.push(padded_image[sub_i][sub_j]);

                        }
                    }
                    new_image[i][j] = algorithm[usize::from_str_radix(&subsection_string, 2).unwrap()];
                }
            }
        }

        padded_image = new_image;
    }

    let lit_count: u32 = padded_image.iter()
                                     .map(|l| l.iter()
                                               .map(|c| match c {
                                                    '0' => 0,
                                                    '1' => 1,
                                                    _ => unreachable!()})
                                                .sum::<u32>())
                                     .sum::<u32>();

    lit_count
}


fn main() -> Result<(), Error> {

    let (algorithm, image) = read_algorithm_and_image();

    println!("{}", part_1(&algorithm, &image));

    println!("{}", part_2(&algorithm, &image));

    Ok(())
}
