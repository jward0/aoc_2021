use std::fs;
use std::io::Error;


fn read_binary_string_from_hex_input() -> String {

    let hex_string = fs::read_to_string("input.txt").unwrap();
    let mut binary_string: String = String::with_capacity(hex_string.len()*4);

    for character in hex_string.chars() {

        let mut binary_character: &str = "0000";

        match character {
            '0' => binary_character = "0000",
            '1' => binary_character = "0001",
            '2' => binary_character = "0010",
            '3' => binary_character = "0011",
            '4' => binary_character = "0100",
            '5' => binary_character = "0101",
            '6' => binary_character = "0110",
            '7' => binary_character = "0111",
            '8' => binary_character = "1000",
            '9' => binary_character = "1001",
            'A' => binary_character = "1010",
            'B' => binary_character = "1011",
            'C' => binary_character = "1100",
            'D' => binary_character = "1101",
            'E' => binary_character = "1110",
            'F' => binary_character = "1111",
            _ => (),
        }

        for c in binary_character.to_string().chars() {
            binary_string.push(c);
        }
    }
    binary_string
}


fn analyse_packet(binary: &mut Vec<char>) -> u64 {

    println!("Analysing packet...");

    let mut value: u64 = 0;

    // Start of packet: V, T
    let v = u64::from_str_radix((binary[0..3].iter().collect::<String>()).as_str(), 2).unwrap();
    let t = u8::from_str_radix((binary[3..6].iter().collect::<String>()).as_str(), 2).unwrap();

    binary.drain(0..6);

    if t == 4 { // if literal packet
        value += analyse_literal_packet(binary);
    } else { // if operator packet
        value += analyse_operator_packet(t, binary);
    }

    value
}


fn analyse_literal_packet(binary: &mut Vec<char>) -> u64 {

    println!("Literal packet");
    let mut value: u64 = 0;

    loop {

        value = value << 4;
        let leading_bit = binary[0];
        let next_four = u64::from_str_radix((binary[1..5].iter().collect::<String>()).as_str(), 2).unwrap();
        value += next_four;
        binary.drain(0..5);

        if leading_bit == '1' { // check for leading 1 bit
            ();
        } else {
            break;
        }
    }

    println!("literal packet contains {}", value);
    value
}


fn analyse_operator_packet(packet_type: u8, binary: &mut Vec<char>) -> u64 {

    let mut value: u64 = 0;
    let mut packet_contents: Vec<u64> = vec![];
    let length_type = binary[0];
    binary.drain(0..1);

    if length_type == '1' { 
        println!("Operator packet, storage type 1");

        let next_11 = u64::from_str_radix((binary[0..11].iter().collect::<String>()).as_str(), 2).unwrap();

        binary.drain(0..11);

        for _i in 0..next_11 {
            packet_contents.push(analyse_packet(binary));
        }

    } else {
        println!("Operator packet, storage 0");

        let next_15 = u64::from_str_radix((binary[0..15].iter().collect::<String>()).as_str(), 2).unwrap();

        binary.drain(0..15);

        let target_len = binary.len() as u64 - next_15;

        while binary.len() as u64 > target_len {
            packet_contents.push(analyse_packet(binary));
        }

    }

    println!("Operator packet operation type {}", packet_type);
    println!("Subpacket contents:");
    for item in packet_contents.iter() {
        println!("{}", item);
    }
    println!("--------");

    match packet_type {
        0 => { // sum
            value = packet_contents.iter().sum();
        }
        1 => { // product
            value = 1;
            for item in packet_contents.iter() {
                value *= item;
            }
        }
        2 => { // minimum
            value = u64::MAX;
            for item in packet_contents.iter() {
                if item < &value {
                    value = *item;
                }
            }
        }
        3 => { // maximum
            value = 0;
            for item in packet_contents.iter() {
                if item > &value {
                    value = *item;
                }
            }
        }
        5 => { // greater than
            if packet_contents[0] > packet_contents[1] {
                value = 1;
            } else {
                value = 0;
            }
        }
        6 => { // less than
            if packet_contents[0] < packet_contents[1] {
                value = 1;
            } else {
                value = 0;
            }
        }
        7 => { // equal to
            if packet_contents[0] == packet_contents[1] {
                value = 1;
            } else {
                value = 0;
            }
        }
        _ => (),
    }

    println!("Operator packet returns {}", value);
    value
}


fn part_1(input: &String) -> u64 {

    let mut binary = input.clone().chars().collect::<Vec<char>>();
    //let mut version_sum: u32 = 0;

    //version_sum += analyse_packet(&mut binary);
    
    analyse_packet(&mut binary)
    //version_sum
}


fn part_2(input: &String) -> u32 {

    0
}


fn main() -> Result<(), Error> {

    let input = read_binary_string_from_hex_input();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}