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


fn part_1(input: &Vec<String>) -> Result<i32, Error> {

    let mut bingo_cards : Vec<BingoEntry> = Vec::new();

    let mut input_iter = input.iter();

    let str_calls: Vec<&str> = input_iter.next().unwrap().split(',').collect();
    let mut running_calls: Vec<i32> = Vec::new();
    let number_of_cards: i32 = (input_iter.len() as f32 / 6.).floor() as i32;

    for _i in 0..number_of_cards {
        let mut new_card_vector: Vec<i32> = Vec::new();
        input_iter.next();
        for _j in 0..5 {
            for val in input_iter
                                .next()
                                .unwrap()
                                .split_whitespace()
                                .collect::<Vec<&str>>()
                                .iter() {
                new_card_vector.push(val.parse::<i32>().unwrap());
            }
        }

        bingo_cards.push(BingoEntry::new(new_card_vector));
    }

    for call in str_calls.iter() {
        running_calls.push(call.parse::<i32>().unwrap());

        for card in bingo_cards.iter() {
            if card.check(&running_calls) {

                let mut running_sum = 0;

                for val in card.numbers.iter() {
                    if !running_calls.contains(val) {
                        running_sum += val;
                    }
                }

                let score = running_sum * running_calls.last().unwrap();

                println!("{}", score);
                return Ok(score);
            }
        }
    }

    Ok(0)
}

fn part_2(input: &Vec<String>) -> Result<i32, Error> {

    let mut bingo_cards : Vec<BingoEntry> = Vec::new();

    let mut input_iter = input.iter();

    let str_calls: Vec<&str> = input_iter.next().unwrap().split(',').collect();
    let mut running_calls: Vec<i32> = Vec::new();
    let number_of_cards: i32 = (input_iter.len() as f32 / 6.).floor() as i32;

    for _i in 0..number_of_cards {
        let mut new_card_vector: Vec<i32> = Vec::new();
        input_iter.next();
        for _j in 0..5 {
            for val in input_iter
                                .next()
                                .unwrap()
                                .split_whitespace()
                                .collect::<Vec<&str>>()
                                .iter() {
                new_card_vector.push(val.parse::<i32>().unwrap());
            }
        }

        bingo_cards.push(BingoEntry::new(new_card_vector));
    }

    let card_count = bingo_cards.len();
    let mut matched_cards = vec![0; card_count];

    for call in str_calls.iter() {

        running_calls.push(call.parse::<i32>().unwrap());

        for (card_ndx, card) in bingo_cards.iter().enumerate() {

            if (card.check(&running_calls)) && (matched_cards[card_ndx] == 0) {

                matched_cards[card_ndx] = 1;

                if matched_cards.iter().sum::<i32>() == card_count as i32 {

                    let mut running_sum = 0;

                    for val in card.numbers.iter() {
                        if !running_calls.contains(val) {
                            running_sum += val;
                        }
                    }

                    let score = running_sum * running_calls.last().unwrap();

                    println!("{}", score);
                    return Ok(score);
                }
            }
        }
    }

    Ok(0)
}

struct BingoEntry {
    numbers: Vec<i32>,
    check_masks: Vec<i32>
}

trait BingoTrait {
    fn new (values: Vec<i32>) -> Self;
    fn check (&self, check_vals: &Vec<i32>) -> bool;
}


impl BingoTrait for BingoEntry {

    fn new (values: Vec<i32>) -> BingoEntry {
        let check_masks = vec![17318416i32, 8659208, 4329604, 2164802, 1082401, 32505856, 1015808, 31744, 992, 31];
        return BingoEntry{numbers: values, check_masks: check_masks};
    }

    fn check (&self, check_vals: &Vec<i32>) -> bool {

        let mut hit_tracker: i32 = 0;

        for check_val in check_vals.iter() {
            for (i, item) in self.numbers.iter().enumerate() {
                if *item == *check_val {
                    hit_tracker += i32::pow(2, (24-i) as u32);
                }
            }
        }

        for item in self.check_masks.iter() {
            if *item&hit_tracker == *item {
                return true;
            }
        }

        false
    }
}


fn main() -> Result<(), Error> {

    let input = strings_from_file(File::open("input.txt")?);

    part_1(&input).expect("Warning: Part 1 failed");

    part_2(&input).expect("Warning: Part 2 failed");

    Ok(())
}