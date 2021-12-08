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


fn part_1(input: &Vec<String>) -> u32 {

    let output_values = input.iter()
                                .map(|i| i.split(" | ")
                                            .collect::<Vec<&str>>()[1]
                                            .split(' ')
                                            .collect())
                                .collect::<Vec<Vec<&str>>>();

    let mut occurances: Vec<u32> = vec![0, 0, 0, 0];
    let mut _other: u32 = 0;

    for i in 0..output_values.len() {
        for j in 0..4 {
            match output_values[i][j].chars().count() {
                2 => occurances[0] += 1,
                3 => occurances[1] += 1,
                4 => occurances[2] += 1,
                7 => occurances[3] += 1,
                _ => _other += 1,
            }
        }
    }

    occurances.iter().sum()

}


fn part_2(input: &Vec<String>) -> u32 {

    let output_values = input.iter()
                                .map(|i| i.split(" | ")
                                            .collect::<Vec<&str>>()[1]
                                            .split(' ')
                                            .collect())
                                .collect::<Vec<Vec<&str>>>();

    let input_values = input.iter()
                            .map(|i| i.split(" | ")
                                        .collect::<Vec<&str>>()[0]
                                        .split(' ')
                                        .collect())
                            .collect::<Vec<Vec<&str>>>();

    let mut running_total: u32 = 0;

    for i in 0..input_values.len() {

        let mut identities: Vec<u8> = vec![0; 10];

        for item in &input_values[i] {

            match item.chars().count() {

                2 => identities[1] = binarize(&item),
                3 => identities[7] = binarize(&item),
                4 => identities[4] = binarize(&item),
                7 => identities[8] = binarize(&item),
                _ => (),
            }
        }

        for item in &input_values[i] {

            let binarized_char: u8 = binarize(&item);

            if item.chars().count() == 6 {
                if binarized_char & identities[1] != identities[1] {
                    identities[6] = binarized_char;
                } else if binarized_char & identities[4] == identities[4] {
                    identities[9] = binarized_char;
                } else {
                    identities[0] = binarized_char;
                }
            } else if item.chars().count() == 5 {
                let five_two_bits: u8 = identities[4] & !identities[1];
                if binarized_char & identities[1] == identities[1] {
                    identities[3] = binarized_char;
                } else if binarized_char & five_two_bits == five_two_bits {
                    identities[5] = binarized_char;
                } else {
                    identities[2] = binarized_char;
                }
            }
        }

        for (j, item) in output_values[i].iter().enumerate() {

            let binarized_char: u8 = binarize(&item);
            let mut digit_value: u32 = 0;

            for i in 0..10 {
                if binarized_char == identities[i] {
                    digit_value = i as u32;
                }
            }
            let multiplier = u32::pow(10, (3-j) as u32);
            running_total += multiplier * digit_value;
        }
        
    }

    running_total

}


fn binarize(input: &str) -> u8 {

    let mut val: u8 = 0;

    for character in input.chars() {

        let shift = character as u32 - 'a' as u32;
            
        val += 1 << shift;
    }

    val
}


fn main() -> Result<(), Error> {

    let input = strings_from_file(File::open("input.txt")?);

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}

