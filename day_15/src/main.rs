use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_vector_of_vectors_from_file() -> Vec<Vec<u32>> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    br.lines()
        .map(|l| l.unwrap()
                  .chars()
                  .map(|d| d.to_digit(10)
                            .unwrap())
                            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}


#[derive(Copy, Clone)]
struct DijkstraNode {

    value: u32,
    tentative_distance: u32,
    visited: bool
}


impl DijkstraNode {

    fn new(value: u32) -> Self {

        DijkstraNode{value: value, tentative_distance: u32::MAX, visited: false}
    }

    fn compare_distance(&mut self, proposed_distance: u32) {

        if proposed_distance < self.tentative_distance {
            self.tentative_distance = proposed_distance;
        }
    }

    fn mark_visited(&mut self) {

        self.visited = true;
    }
}


fn part_1(input: &Vec<Vec<u32>>) -> u32 {

    let mut dijkstra_map: Vec<Vec<DijkstraNode>>;

    dijkstra_map = input.iter().map(
                        |line| line.iter().map(
                            |v| DijkstraNode::new(*v))
                                .collect::<Vec<DijkstraNode>>())
                        .collect::<Vec<Vec<DijkstraNode>>>();

    let max_i = dijkstra_map.len() - 1 ;
    let max_j = dijkstra_map[0].len() - 1;

    let mut current_position: [usize; 2] = [0, 0];

    dijkstra_map[current_position[0]][current_position[1]].mark_visited();
    dijkstra_map[current_position[0]][current_position[1]].tentative_distance = 0;

    let mut unvisited_set: Vec<[usize; 2]> = vec![];

    loop {

        let mut locations_to_check: Vec<[usize; 2]> = vec![];
        let i = current_position[0];
        let j = current_position[1];

        if i < max_i {
            locations_to_check.push([i+1, j]);
        }
        if j < max_j  {
            locations_to_check.push([i, j+1]);
        }
        if i > 0 {
            locations_to_check.push([i-1, j]);
        }
        if j > 0 {
            locations_to_check.push([i, j-1]);
        }

        for location in locations_to_check.iter() {

            let proposed_distance = dijkstra_map[location[0]][location[1]].value + dijkstra_map[current_position[0]][current_position[1]].tentative_distance;
            
            if !dijkstra_map[location[0]][location[1]].visited {
                dijkstra_map[location[0]][location[1]].compare_distance(proposed_distance);
                unvisited_set.push([location[0], location[1]]);
            }
        }    
        
        let mut smallest_distance: u32 = u32::MAX;
        let mut next_location: [usize; 2] = [0, 0];

        for location in &unvisited_set {
            if dijkstra_map[location[0]][location[1]].tentative_distance < smallest_distance {
                smallest_distance = dijkstra_map[location[0]][location[1]].tentative_distance;
                next_location = *location;
            }
        }

        dijkstra_map[i][j].mark_visited();

        if (i, j) == (max_i, max_j) {
            break;
        }

        unvisited_set.retain(|&x| x!= current_position);

        current_position = next_location;
    }

    dijkstra_map[max_i][max_j].tentative_distance
}


fn part_2(input: &Vec<Vec<u32>>) -> u32 {
    
    let mut dijkstra_map: Vec<Vec<DijkstraNode>>;

    dijkstra_map = input.iter().map(
                        |line| line.iter().map(
                            |v| DijkstraNode::new(*v))
                                .collect::<Vec<DijkstraNode>>())
                        .collect::<Vec<Vec<DijkstraNode>>>();

    let mut tiled_dijkstra_map: Vec<Vec<DijkstraNode>> = vec![];

    for i in 0..5 {

        for line in dijkstra_map.clone().iter() {

            let mut new_line: Vec<DijkstraNode> = vec![];

            for j in 0..5 {
                for mut node in line.clone() {
                    node.value += i+j;
                    if node.value > 9 {
                        node.value -= 9;
                    }
                    new_line.push(node);
                }
            }
            tiled_dijkstra_map.push(new_line);
        }
    }

    dijkstra_map = tiled_dijkstra_map;

    let max_i = dijkstra_map.len() - 1 ;
    let max_j = dijkstra_map[0].len() - 1;

    let mut current_position: [usize; 2] = [0, 0];

    dijkstra_map[current_position[0]][current_position[1]].mark_visited();
    dijkstra_map[current_position[0]][current_position[1]].tentative_distance = 0;

    let mut unvisited_set: Vec<[usize; 2]> = vec![];

    loop {

        let mut locations_to_check: Vec<[usize; 2]> = vec![];
        let i = current_position[0];
        let j = current_position[1];

        if i < max_i {
            locations_to_check.push([i+1, j]);
        }
        if j < max_j  {
            locations_to_check.push([i, j+1]);
        }
        if i > 0 {
            locations_to_check.push([i-1, j]);
        }
        if j > 0 {
            locations_to_check.push([i, j-1]);
        }

        for location in locations_to_check.iter() {

            let proposed_distance = dijkstra_map[location[0]][location[1]].value + dijkstra_map[current_position[0]][current_position[1]].tentative_distance;
            
            if !dijkstra_map[location[0]][location[1]].visited {
                dijkstra_map[location[0]][location[1]].compare_distance(proposed_distance);
                unvisited_set.push([location[0], location[1]]);
            }
        }    
        
        let mut smallest_distance: u32 = u32::MAX;
        let mut next_location: [usize; 2] = [0, 0];

        for location in &unvisited_set {
            if dijkstra_map[location[0]][location[1]].tentative_distance < smallest_distance {
                smallest_distance = dijkstra_map[location[0]][location[1]].tentative_distance;
                next_location = *location;
            }
        }

        dijkstra_map[i][j].mark_visited();

        if (i, j) == (max_i, max_j) {
            break;
        }

        unvisited_set.retain(|&x| x!= current_position);

        current_position = next_location;
    }

    dijkstra_map[max_i][max_j].tentative_distance
}


fn main() -> Result<(), Error> {

    let input = read_vector_of_vectors_from_file();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}
