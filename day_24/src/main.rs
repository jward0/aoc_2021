extern crate num_digitize;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use num_digitize::ToDigits;


fn generate_command_list() -> Vec<Command> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    br.lines().map(|l| Command::read_from_string(l.unwrap())).collect::<Vec<Command>>()
}

struct Command {
    operation: String,
    target: usize,
    value_arg: Option<i64>,
    variable_arg: Option<usize>
}

impl Command {

    fn read_from_string(input: String) -> Self {
        let split_input: Vec<&str> = input.split_whitespace().collect();

        let target: usize = match split_input[1].chars().next().unwrap() {
            'x' => 0,
            'y' => 1,
            'z' => 2,
            'w' => 3,
            _ => unreachable!()
        };

        if split_input.len() > 2 {

            let mut value_arg: Option<i64> = None;
            let mut variable_arg: Option<usize> = None;

            match split_input[2] {
                "x" => {
                    value_arg = None;
                    variable_arg = Some(0);
                }
                "y" => {
                    value_arg = None;
                    variable_arg = Some(1);
                }
                "z" => {
                    value_arg = None;
                    variable_arg = Some(2);
                }
                "w" => {
                    value_arg = None;
                    variable_arg = Some(3);
                }
                _ => ()
            }

            if value_arg == None && variable_arg == None {
                value_arg = Some(split_input[2].parse().unwrap());
            }

            return Command{ operation: split_input[0].to_string(),
                            target: target,
                            value_arg: value_arg,
                            variable_arg: variable_arg }
        } else {
            return Command{ operation: split_input[0].to_string(), 
                            target: target,
                            value_arg: None,
                            variable_arg: None };
        }
    }

    fn apply(&self, storage: &mut Vec<i64>, input: &mut Vec<i64>) -> () {

        let mut arg_val: i64 = 0;

        if self.value_arg != None {
            arg_val = self.value_arg.unwrap();
        } else if self.variable_arg != None {
            arg_val = storage[self.variable_arg.unwrap()];
        } else {
            ();
        }
        
        match self.operation.as_str() {
            "inp" => {
                storage[self.target] = input.remove(0) as i64;
                println!("reading input val: {}", storage[self.target]);
            },
            "add" => storage[self.target] += arg_val,
            "mul" => storage[self.target] *= arg_val,
            "div" => storage[self.target] /= arg_val,
            "mod" => storage[self.target] = storage[self.target] % arg_val,
            "eql" => storage[self.target] = match storage[self.target] == arg_val {
                true => 1,
                false => 0
            },
            _ => unreachable!()
        }
    }
}


fn part_1(input: &Vec<Command>) -> u64 {

    let mut model_number = 13621111481315  ;
    let mut largest_valid_number = 0;


   // while model_number < 83998422597919 + 1  {
        //println!("model_number: {}", model_number);

    if model_number % 1000000 == 0 {
        println!("{}", model_number);
    }

    let mut model_number_vec = model_number.to_digits().iter()
                                        .map(|d| *d as i64)
                                        .collect::<Vec<i64>>();
    
    if !model_number_vec.contains(&0) {
        let mut storage: Vec<i64> = vec![0, 0, 0, 0];
        for command in input {
            command.apply(&mut storage, &mut model_number_vec);
        }

        println!("{}, {}, {}, {}", storage[0], storage[1], storage[2], storage[3]);
        println!("----------------------------");
        if storage[2] == 0 && model_number > largest_valid_number {
            largest_valid_number = model_number;
            println!("{}", largest_valid_number);
        } else {
        }
    }   
    model_number += 1;                
    //}
    
    largest_valid_number
}

fn part_2(input: &Vec<Command>) -> u64 {

    0
}


fn main() -> Result<(), Error> {

    let input = generate_command_list();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}