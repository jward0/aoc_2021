struct Bin {
    values: Vec<char>,
    remove_cost: u32
}

fn letter_cost(input: &char) -> u32 {
    match input {
        'a' => 1,
        'b' => 10,
        'c' => 100,
        'd' => 1000,
        _ => unreachable!()
   }
}

fn push_move_to_corridor(bins: &[Bin; 4], corridor: &[char; 7], cost: &u32, bin_ndx: usize, corridor_ndx: usize, move_distance: u32) -> ([Bin; 4], [char; 7], u32) {

    let mut new_bins = [Bin{ values: (*bins[0].values).to_vec(), remove_cost: bins[0].remove_cost }, Bin{ values: (*bins[1].values).to_vec(), remove_cost: bins[1].remove_cost }, Bin{ values: (*bins[2].values).to_vec(), remove_cost: bins[2].remove_cost }, Bin{ values: (*bins[3].values).to_vec(), remove_cost: bins[3].remove_cost }];
    let mut new_corridor = corridor.clone();
    let mut new_cost: &mut u32 = &mut 0;
    *new_cost = *cost;
    new_corridor[corridor_ndx] = new_bins[bin_ndx].values[0];
    *new_cost += (move_distance + new_bins[bin_ndx].remove_cost)*letter_cost(&new_bins[bin_ndx].values[0]);
    new_bins[bin_ndx].values.remove(0);
    new_bins[bin_ndx].remove_cost += 1;

    (new_bins, new_corridor, *new_cost)
}

fn push_move_to_bin(bins: &[Bin; 4], corridor: &[char; 7], cost: &u32, bin_ndx: usize, corridor_ndx: usize, move_distance: u32) -> ([Bin; 4], [char; 7], u32) {
    let mut new_bins = [Bin{ values: (*bins[0].values).to_vec(), remove_cost: bins[0].remove_cost }, Bin{ values: (*bins[1].values).to_vec(), remove_cost: bins[1].remove_cost }, Bin{ values: (*bins[2].values).to_vec(), remove_cost: bins[2].remove_cost }, Bin{ values: (*bins[3].values).to_vec(), remove_cost: bins[3].remove_cost }];
    let mut new_corridor = corridor.clone();
    let mut new_cost: &mut u32 = &mut 0;    
    *new_cost = *cost;
    *new_cost += (move_distance + new_bins[bin_ndx].remove_cost - 1)*letter_cost(&new_corridor[corridor_ndx]);
    new_bins[bin_ndx].values.insert(0, new_corridor[corridor_ndx]);
    new_corridor[corridor_ndx] = '.';
    new_bins[bin_ndx].remove_cost -= 1;

    (new_bins, new_corridor, *new_cost)
}

fn all_elements_equal(vec: &Vec<char>, character: char) -> bool {

    if vec.is_empty() {
        return true;
    }

    vec.iter().all(|&item| item == character)
}

fn find_deadend_states(bins: &[Bin; 4], corridor: &[char; 7]) -> bool {

    if corridor[0] == 'd' || corridor[1] == 'd' || corridor[2] == 'd' || corridor[3] == 'd' {
        return true;
    }

    if corridor[4] == 'd' && bins[3].values.len() == 3 && bins[3].values[0] == 'a' {
        return true;
    }

    if corridor[2] == 'c' && corridor[3] == 'a' {
        return true;
    }

    if corridor[2] == 'd' && corridor[3] == 'a' {
        return true;
    }

    if corridor[3] == 'd' && corridor[4] == 'a' {
        return true;
    }

    if corridor[3] == 'd' && corridor[4] == 'b' {
        return true;
    }

    return false;
}

fn find_bin_corridor_path(corridor_spaces: &[bool; 7], corridor_ndx: usize, bin_ndx: usize) -> (bool, u32) {

    match (bin_ndx, corridor_ndx) {
        (0, 0) => {
            return (!corridor_spaces[1], 3);
        }
        (0, 1) => {
            return (true, 2);
        }
        (0, 2) => {
            return (true, 2);
        }
        (0, 3) => {
            return (!corridor_spaces[2], 4);
        }
        (0, 4) => {
            return (!(corridor_spaces[2]||corridor_spaces[3]), 6);
        }
        (0, 5) => {
            return (!(corridor_spaces[2]||corridor_spaces[3]||corridor_spaces[4]), 8);
        }
        (0, 6) => {
            return (!(corridor_spaces[2]||corridor_spaces[3]||corridor_spaces[4]||corridor_spaces[5]), 9);
        }
        (1, 0) => {
            return (!(corridor_spaces[1]||corridor_spaces[2]), 5);
        }
        (1, 1) => {
            return (!corridor_spaces[2], 4);
        }
        (1, 2) => {
            return (true, 2);
        }
        (1, 3) => {
            return (true, 2);
        }
        (1, 4) => {
            return (!corridor_spaces[3], 4);
        }
        (1, 5) => {
            return (!(corridor_spaces[3]||corridor_spaces[4]), 6);
        }
        (1, 6) => {
            return (!(corridor_spaces[3]||corridor_spaces[4]||corridor_spaces[5]), 7);
        }
        (2, 0) => {
            return (!(corridor_spaces[1]||corridor_spaces[2]||corridor_spaces[3]), 7);
        }
        (2, 1) => {
            return (!(corridor_spaces[2]||corridor_spaces[3]), 6);
        }
        (2, 2) => {
            return (!corridor_spaces[3], 4);
        }
        (2, 3) => {
            return (true, 2);
        }
        (2, 4) => {
            return (true, 2);
        }
        (2, 5) => {
            return (!corridor_spaces[4], 4);
        }
        (2, 6) => {
            return (!(corridor_spaces[4]||corridor_spaces[5]), 5);
        }
        (3, 0) => {
            return (!(corridor_spaces[1]||corridor_spaces[2]||corridor_spaces[3]||corridor_spaces[4]), 9);
        }
        (3, 1) => {
            return (!(corridor_spaces[2]||corridor_spaces[3]||corridor_spaces[4]), 8);
        }
        (3, 2) => {
            return (!(corridor_spaces[3]||corridor_spaces[4]), 6);
        }
        (3, 3) => {
            return (!corridor_spaces[4], 4);
        }
        (3, 4) => {
            return (true, 2);
        }
        (3, 5) => {
            return (true, 2);
        }
        (3, 6) => {
            return (!corridor_spaces[5], 3);
        }
        _ => unreachable!()
    };
}


fn find_valid_moves(bins: &[Bin; 4], corridor: &[char; 7], cost: &u32, min_cost: &mut u32) -> () {

    let c0: bool = corridor[0] != '.';
    let c1: bool = corridor[1] != '.';
    let c2: bool = corridor[2] != '.';
    let c3: bool = corridor[3] != '.';
    let c4: bool = corridor[4] != '.';
    let c5: bool = corridor[5] != '.';
    let c6: bool = corridor[6] != '.';

    let corridor_spaces = [c0, c1, c2, c3, c4, c5, c6];

    if bins[0].values.len() == 4 && bins[1].values.len() == 4 && bins[2].values.len() == 4 && bins[3].values.len() == 4 && *cost > 0 {
        if cost < min_cost {
            *min_cost = *cost;
            println!("CUrrent smallest: {}", min_cost);
        }
    //} else if !find_deadend_states(&bins, &corridor) {
    } else {
        /*
        println!("corridor: {}, {}, {}, {}, {}, {}, {}", &corridor[0], &corridor[1], &corridor[2], &corridor[3], &corridor[4], &corridor[5], &corridor[6]);
        println!("bins:");
        for item in bins[0].values.iter() {
            println!("{}", item);
        }
        println!("---");
        for item in bins[1].values.iter() {
            println!("{}", item);
        }
        println!("---");
        for item in bins[2].values.iter() {
            println!("{}", item);
        }
        println!("---");
        for item in bins[3].values.iter() {
            println!("{}", item);
        }
        println!("---");
        println!("cost to here: {}", *cost);
        */
        let mut next_states: Vec<([Bin; 4], [char; 7], u32)> = vec![];

        for (b_ndx, character) in [(0, 'a'), (1, 'b'), (2, 'c'), (3, 'd')] {

            if bins[b_ndx].values.len() > 0 && !all_elements_equal(&bins[b_ndx].values, character) {
                // bin can give  
                for (c_ndx, val) in corridor_spaces.iter().enumerate() {

                    if !*val {
                        // free space in corridor. Check if it's reachable and get distance:
                        let (reachable, distance) = find_bin_corridor_path(&corridor_spaces, c_ndx, b_ndx);
                        //println!("checking bin {} into corridor {}: {}, {}", b_ndx, c_ndx, reachable, distance);
                        if reachable {
                            next_states.push(push_move_to_corridor(&bins, &corridor, &cost, b_ndx, c_ndx, distance));
                        }
                    }

                } 

             } else if bins[b_ndx].values.len() < 4 && all_elements_equal(&bins[b_ndx].values, character) {
                // bin can receive
                for (c_ndx, val) in corridor_spaces.iter().enumerate() {

                    if *val {
                        //something in corridor. Check if it's the right type:
                        if corridor[c_ndx] == character {
                            // check if reachable and get distance:
                            let (reachable, distance) = find_bin_corridor_path(&corridor_spaces, c_ndx, b_ndx);

                            if reachable {
                                next_states.push(push_move_to_bin(&bins, &corridor, &cost, b_ndx, c_ndx, distance));
                            }
                        }
                    }
                    
                }
             }
        }

        //println!("valid moves: {}", next_states.len());
        for state in next_states.iter() {
            if state.2 < *min_cost {
                find_valid_moves(&state.0, &state.1, &state.2, min_cost);
            }
        }
    }
}

fn main() {
    let mut bins: [Bin; 4] = [Bin{ values: vec!['b', 'd', 'd', 'd'], remove_cost: 0 }, Bin{ values: vec!['b', 'c', 'b', 'c'], remove_cost: 0 }, Bin{ values: vec!['c', 'b', 'a', 'a'], remove_cost: 0 }, Bin{ values: vec!['d', 'a', 'c', 'a'], remove_cost: 0 }];
    let mut corridor: [char; 7] = ['.', '.', '.', '.', '.', '.', '.'];
    let mut cost: u32 = 0;

    let min_cost: &mut u32 = &mut 99999;

    find_valid_moves(&bins, &corridor, &cost, min_cost);

    println!("{}", min_cost)

}
