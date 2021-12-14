use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_caves_from_file() -> (HashMap<String, usize>, Vec<Cave>) {

    let mut caves: Vec<Cave> = vec![];
    let mut cave_indices: HashMap<String, usize> = HashMap::new();

    let br = BufReader::new(File::open("input.txt").unwrap());
    for cave_pair in br.lines()
                        .map(|line| line.unwrap()
                                        .split('-')
                                        .collect::<Vec<&str>>()
                                        .iter()
                                        .map(|s| s.to_string()) 
                                        .collect::<Vec<String>>()) {

        

        if !cave_indices.contains_key(&cave_pair[0]) {
            cave_indices.insert((*cave_pair[0]).to_string(), caves.len());
            caves.push(Cave::new((*cave_pair[0]).to_string()));
        }
        if !cave_indices.contains_key(&cave_pair[1]) {
            cave_indices.insert((*cave_pair[1]).to_string(), caves.len());
            caves.push(Cave::new((*cave_pair[1]).to_string()));
        }

        caves[*cave_indices.get(&*cave_pair[0]).unwrap()].links.push((*cave_pair[1]).to_string());
        caves[*cave_indices.get(&*cave_pair[1]).unwrap()].links.push((*cave_pair[0]).to_string());
    }

    (cave_indices, caves)
}


struct Cave {

    name: String,
    large: bool,
    links: Vec<String>
}


impl Cave {

    fn new(name: String) -> Self {

        let small: bool = name.chars().any(|c| matches!(c, 'a'..='z'));

        Cave{name: name, large: !small, links: vec![]}
    }
}


fn check_next_nodes(caves: &Vec<Cave>, cave_indices: &HashMap<String, usize>, current_room: &Cave, disallowed_rooms: &mut Vec<String>, path_counter: &mut u32, restricted_visiting: &mut bool) -> () {

    for link in &current_room.links {
        
        let linked_cave = &caves[*cave_indices.get(link).unwrap()];

        if linked_cave.name == "end" {
            *path_counter += 1;

        } else if linked_cave.name == "start" {
            ();

        } else {

            if linked_cave.large {
                check_next_nodes(caves, cave_indices, &linked_cave, &mut disallowed_rooms.to_vec(), path_counter, restricted_visiting);
            
            } else if disallowed_rooms.contains(&linked_cave.name) && !*restricted_visiting{
                *restricted_visiting = true;
                check_next_nodes(caves, cave_indices, &linked_cave, &mut disallowed_rooms.to_vec(), path_counter, restricted_visiting);
                *restricted_visiting = false;

            } else if disallowed_rooms.contains(&linked_cave.name) {
                ();

            } else {
                disallowed_rooms.push((*linked_cave.name).to_string());
                check_next_nodes(caves, cave_indices, &linked_cave, &mut disallowed_rooms.to_vec(), path_counter, restricted_visiting);   
                disallowed_rooms.pop();
            }
        }
    }
}


fn part_1(cave_indices: &HashMap<String, usize>, caves: &Vec<Cave>) -> u32 {

    let start_cave = &caves[*cave_indices.get("start").unwrap()];
    let mut path_counter: u32 = 0;
    let mut disallowed_rooms: Vec<String> = vec!["start".to_string()];
    let mut restricted_visiting: bool = true;

    check_next_nodes(&caves, &cave_indices, &start_cave, &mut disallowed_rooms, &mut path_counter, &mut restricted_visiting);

    path_counter

}


fn part_2(cave_indices: &HashMap<String, usize>, caves: &Vec<Cave>) -> u32 {

    let start_cave = &caves[*cave_indices.get("start").unwrap()];
    let mut path_counter: u32 = 0;
    let mut disallowed_rooms: Vec<String> = vec!["start".to_string()];
    let mut restricted_visiting: bool = false;

    check_next_nodes(&caves, &cave_indices, &start_cave, &mut disallowed_rooms, &mut path_counter, &mut restricted_visiting);

    path_counter

}


fn main() -> Result<(), Error> {

    let (cave_indices, caves) = read_caves_from_file();

    println!("{}", part_1(&cave_indices, &caves));

    println!("{}", part_2(&cave_indices, &caves));

    Ok(())
}
