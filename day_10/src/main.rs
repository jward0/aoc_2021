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

    let mut score: u32 = 0;

    for line in input.iter() {

        score += check_line(line.to_string()).0;
    }

    score
}


fn part_2(input: &Vec<String>) -> u64 {

    let mut scores: Vec<u64> = vec![];

    for line in input.iter() {

        scores.push(check_line(line.to_string()).1)
    }

    let mut trimmed_scores: Vec<u64> = vec![];

    for score in scores {
        if score != 0 {
            trimmed_scores.push(score);
        }
    }

    trimmed_scores.sort();

    trimmed_scores[(trimmed_scores.len()+1)/2 - 1]
}


fn check_line(line: String) -> (u32, u64) {

    let mut line_stack: Vec<char> = vec![];

    let characters: Vec<char> = line.chars().collect();

    for character in characters.iter() {

        match character {
            '(' | '[' | '{' | '<' => line_stack.push(*character),
            ')' => {
                if line_stack.pop().unwrap() != '(' {
                    return (3, 0);
                }
            }
            ']' => {
                if line_stack.pop().unwrap() != '[' {
                    return (57, 0);
                }
            }
            '}' => {
                if line_stack.pop().unwrap() != '{' {
                    return (1197, 0);
                }
            }
            '>' => {
                if line_stack.pop().unwrap() != '<' {
                    return (25137, 0);
                }
            }
            _ => ()
        }
    }

    let required_characters: Vec<&char> = line_stack.iter().rev().collect();
    let mut score: u64 = 0;

    for character in required_characters.iter() {

        match character {
            '(' => score = score*5 + 1,
            '[' => score = score*5 + 2,
            '{' => score = score*5 + 3,
            '<' => score = score*5 + 4,
            _ => ()
        }
    }

    (0, score)
}


fn main() -> Result<(), Error> {

    let input = strings_from_file(File::open("input.txt")?);

    println!("{}", part_1(&input));
    println!("---");
    println!("{}", part_2(&input));

    Ok(())
}