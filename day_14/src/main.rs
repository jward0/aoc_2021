extern crate itertools; // 0.7.8

use itertools::Itertools;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_start_and_rules_from_file() -> (Vec<char>, HashMap<String, char>) {

    let mut start: Vec<char> = vec![];
    let mut rules: HashMap<String, char> = HashMap::new();

    let br = BufReader::new(File::open("input.txt").unwrap());

    for (ndx, line) in br.lines().enumerate() {
        if ndx == 0 {
            start = line.unwrap().chars().collect();
        } else if ndx > 1 {
            let tmp = line.unwrap();
            let line_sections = tmp.split(" -> ").collect::<Vec<&str>>();
            rules.insert(line_sections[0].to_string(), line_sections[1].chars().next().unwrap());
        }
    }

    (start, rules)
}


fn part_1(start: &Vec<char>, rules: &HashMap<String, char>) -> u32 { // naive solution

    let mut current_vec = start.to_vec();

    for _i in 0..10 {

        let mut new_vec: Vec<char> = vec![];

        for (a, b) in current_vec.iter().tuple_windows() {
            new_vec.push(*a);
            new_vec.push(*rules.get(&(vec![*a, *b].iter().collect::<String>())).unwrap());
        }

        new_vec.push(*current_vec.last().unwrap());

        current_vec = new_vec;
    }

    current_vec.sort();
    let mut unique_characters: Vec<char> = vec![];

    for character in current_vec.iter() {
        if !unique_characters.contains(character) {
            unique_characters.push(*character);
        }
    }

    println!("{}", unique_characters.iter().collect::<String>());

    for character in unique_characters {
        let character_count: u32 = current_vec.iter().filter(|n| **n == character).count() as u32;
        println!("{}: {}", character, character_count)
    }

    current_vec.len() as u32
}


fn part_2(start: &Vec<char>, rules: &HashMap<String, char>) -> u32 { // general solution

    let mut modified_rules: HashMap<String, [String; 2]> = HashMap::new();

    for (k, v) in rules.iter() {

        modified_rules.insert(k.to_string(), [vec![k.chars().collect::<Vec<char>>()[0], *v].iter().collect::<String>(), vec![*v, k.chars().collect::<Vec<char>>()[1]].iter().collect::<String>()]);
    }

    let mut pair_counter: HashMap<String, u64> = HashMap::new();

    for (a, b) in start.iter().tuple_windows() {
        let pair = vec![*a, *b].iter().collect::<String>();
        *pair_counter.entry(pair).or_insert(0) += 1;
    }

    for i in 0..40 {

        let mut new_pair_counter: HashMap<String, u64> = HashMap::new();

        for (k, v) in pair_counter.clone().iter() {
            for new_pair in modified_rules.get(&*k).unwrap().iter() {
                *new_pair_counter.entry(new_pair.to_string()).or_insert(0) += v;
            }
        }

        pair_counter = new_pair_counter;
    }

    let mut character_counter: HashMap<char, u64> = HashMap::new();

    for (k, v) in pair_counter.iter() {
        *character_counter.entry(k.chars().collect::<Vec<char>>()[1]).or_insert(0) += v;
    }

    *character_counter.entry(start[0]).or_insert(0) += 1;

    for (k, v) in character_counter.iter() {
        println!("{}: {}", k, v);
    }

    0
}


fn main() -> Result<(), Error> {

    let (start, rules) = read_start_and_rules_from_file();

    println!("{}", part_1(&start, &rules));

    println!("{}", part_2(&start, &rules));

    Ok(())
}