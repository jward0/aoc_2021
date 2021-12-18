use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_input_to_op_vectors() -> Vec<Vec<SNVal>> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    br.lines().map(|l| l.unwrap().chars()
                        .map(|c| match c {
                            '0'..='9' => SNVal::val(c.to_digit(10).unwrap() as u32),
                            '[' => SNVal::open,
                            ']' => SNVal::close,
                            ',' => SNVal::comma,
                            _ => unreachable!(),
                        }).collect::<Vec<SNVal>>()
                    ).collect::<Vec<Vec<SNVal>>>()
}


#[derive(Copy, Clone, PartialEq)]
enum SNVal {
    open,
    close,
    comma,
    val(u32)
}


struct SNLine {

    values: Vec<SNVal>,
}


impl SNLine {

    fn add_to_self(&mut self, other: &mut Vec<SNVal>) -> () {

        self.values.insert(0, SNVal::open);
        self.values.push(SNVal::comma);
        self.values.append(other);
        self.values.push(SNVal::close);
    }

    fn try_split(&mut self) -> bool {

        let mut split_line = self.values.clone();

        for (ndx, item) in self.values.iter().enumerate() {
            
            match item {
                SNVal::val(n) => {
                    if *n > 9 {
                        //println!("Splitting:");
                        //self.print_self();
                        split_line.remove(ndx);
                        split_line.insert(ndx, SNVal::close);
                        split_line.insert(ndx, SNVal::val((*n as f32/2.0).ceil() as u32));
                        split_line.insert(ndx, SNVal::comma);
                        split_line.insert(ndx, SNVal::val((*n as f32/2.0).floor() as u32));
                        split_line.insert(ndx, SNVal::open);
                        
                        self.values = split_line;
                        //self.print_self();
                        //println!("==========");
                        return (true);
                    }
                }
                _ => (),
            }
        }
        
        false
    }

    fn try_explode(&mut self) -> bool {

        let mut current_level = 0;
        let mut exploded_line = self.values.clone();
    
        for (ndx, item) in self.values.iter().enumerate() {
    
            match item {
                SNVal::open => {
                    current_level += 1;
                }
                SNVal::close => {
                    current_level -= 1;
                } 
                SNVal::val(n) => {
    
                    if current_level > 4 && self.values[ndx+1] == SNVal::comma && self.values[ndx+2] != SNVal::open {
    
                        //println!("Exploding");
                        //self.print_self();
    
                        let l_val = match self.values[ndx] {
                            SNVal::val(n) => n,
                            _ => unreachable!(),
                        };
                        let r_val = match self.values[ndx + 2] {
                            SNVal::val(n) => n,
                            _ => unreachable!(),
                        };
            
                        exploded_line[ndx-1] = SNVal::val(0);
                        exploded_line.drain(ndx..ndx+4);
                    
                        for t in exploded_line[..ndx-1].iter_mut().rev() {
                            if let SNVal::val(n) = t {
                                *t = SNVal::val(*n + l_val);
                                break;
                            }
                        }
                        for t in exploded_line[ndx..].iter_mut() {
                            if let SNVal::val(n) = t {
                                *t = SNVal::val(*n + r_val);
                                break;
                            }
                        }

                        if exploded_line[ndx] == SNVal::open {
                            exploded_line.insert(ndx, SNVal::comma);
                        }

                        self.values = exploded_line;
                        //self.print_self();
                        //println!("---------------");
                        return true;
                    }
                }
                _ => (), 
            }
        }

        false
    }

    fn collapse(&mut self) -> () {

        loop {
            let mut explode_tracker = false;
            let mut split_tracker = false;
            let mut any_explodes = false;
            let mut any_splits = false;

            
    
            loop {
                explode_tracker = self.try_explode();
                if !explode_tracker {
                    break;
                } else {
                    any_explodes = true;
                }
            }
    
            loop {
                split_tracker = self.try_split();
                if split_tracker {
                    any_splits = true;
                }
                break;
            }
    
            if !any_explodes && !any_splits {
                break;
            }
        }

        //self.print_self();
    }

    fn find_magnitude(&self) -> u32 {

        println!("Finding magnitude...");

        let mut line = self.values.clone();

        loop {

            let mut m_line = line.clone();
   
            for (ndx, item) in line.iter().enumerate() {

                if m_line[ndx] == SNVal::open && m_line[ndx+4] == SNVal::close {
                    //println!("Index {}", ndx);
                    let l_val = match m_line[ndx+1] {
                        SNVal::val(n) => n,
                        _ => unreachable!(),
                    };
                    let r_val = match m_line[ndx+3] {
                        SNVal::val(n) => n,
                        _ => unreachable!(),
                    };

                    //println!("{}, {} -> {}", l_val, r_val, 3*l_val + 2*r_val);

                    m_line.insert(ndx, SNVal::val(3*l_val + 2*r_val));
                    m_line.drain(ndx+1..ndx+6);

                    if m_line.len() == 1 {
                        return 3*l_val + 2*r_val;
                    }

                    break;
                }
            }

            line = m_line.clone();
        }

    }

    fn print_self(&self) -> () {

        let mut printable_line: String = String::new();

        for item in self.values.iter() {
            match item {
                SNVal::open => printable_line.push('['),
                SNVal::close => printable_line.push(']'),
                SNVal::comma => printable_line.push(','),
                SNVal::val(n) => printable_line.push_str(&n.to_string()),
                _ => unreachable!(),
            }
        }
    
        println!("{}", printable_line);        
    }
}


fn part_1(input: &Vec<Vec<SNVal>>) -> u32 {

    let mut operative_line: SNLine = SNLine{ values: input[0].clone() };

    for i in 1..input.len() {

        let mut next_line = input[i].clone();

        operative_line.add_to_self(&mut next_line);
    
        operative_line.collapse();

    }

    operative_line.find_magnitude()

}


fn part_2(input: &Vec<Vec<SNVal>>) -> u32 {

    let mut largest_product: u32 = 0;

    for i in 0..input.len() - 1 {
        for j in 0..input.len() -1 {
            if i != j {
                let mut first_line: SNLine = SNLine{ values: input[i].clone() };
                let mut second_line = input[j].clone();

                first_line.add_to_self(&mut second_line);

                first_line.collapse();

                let mag = first_line.find_magnitude();

                if mag > largest_product {
                    largest_product = mag;
                }
            }
        }
    }

    largest_product
}



fn main() -> Result<(), Error> {

    let input = read_input_to_op_vectors();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}