use std::io::Error;

fn part_1() -> u32 {

    let mut player_1: [u32; 2] = [7, 0];
    let mut player_2: [u32; 2] = [5, 0];
    let mut move_distance: u32 = 6;
    let mut roll_count: u32 = 0;

    loop {
        roll_count += 3;

        if (player_1[0] + move_distance) % 10 == 0 {
            player_1[0] = 10;
        } else {
            player_1[0] = (player_1[0] + move_distance) % 10
        }

        player_1[1] += player_1[0];

        if player_1[1] > 999 {
            return player_2[1] * roll_count;
        }

        //println!("Player 1 rolls {} and moves to {} for score of {}", move_distance,)

        roll_count += 3;
        move_distance += 9;

        if (player_2[0] + move_distance) % 10 == 0 {
            player_2[0] = 10;
        } else {
            player_2[0] = (player_2[0] + move_distance) % 10
        }
        
        player_2[1] += player_2[0];

        if player_2[1] > 999 {
            return player_1[1] * roll_count;
        }

        move_distance += 9;
    }
}


fn player_1_roll(scores: &mut [u64; 2], positions: &mut [u64; 2]) -> [u64; 2] {

    let mut win_count_1: u64 = 0;
    let mut win_count_2: u64 = 0;

    for roll_1 in 3..10 {

        let new_position: u64;

        if (positions[0] + roll_1) % 10 == 0 {
            new_position = 10;
        } else {
            new_position = (positions[0] + roll_1) % 10;
        }
        
        let roll_1_multiplier: u64 = match roll_1 {
            3 => 1,
            4 => 3,
            5 => 6,
            6 => 7,
            7 => 6, 
            8 => 3,
            9 => 1,
            _ => unreachable!(),
        };

        if scores[0] + new_position > 20 {
            win_count_1 += roll_1_multiplier;
        } else {
            let subsequent_wins = player_2_roll(&mut [scores[0] + new_position, scores[1]], &mut [new_position, positions[1]]);
            win_count_1 += subsequent_wins[0] * roll_1_multiplier;
            win_count_2 += subsequent_wins[1] * roll_1_multiplier;
        }
    }

    [win_count_1, win_count_2]
}


fn player_2_roll(scores: &mut [u64; 2], positions: &mut [u64; 2]) -> [u64; 2] {

    let mut win_count_1: u64 = 0;
    let mut win_count_2: u64 = 0;

    for roll_2 in 3..10 {

        let new_position: u64;

        if (positions[1] + roll_2) % 10 == 0 {
            new_position = 10;
        } else {
            new_position = (positions[1] + roll_2) % 10;
        }

        let roll_2_multiplier: u64 = match roll_2 {
            3 => 1,
            4 => 3,
            5 => 6,
            6 => 7,
            7 => 6, 
            8 => 3,
            9 => 1,
            _ => unreachable!(),
        };

        if scores[1] + new_position > 20 {
            win_count_2 += roll_2_multiplier;
        } else {
            let subsequent_wins = player_1_roll(&mut [scores[0], scores[1] + new_position], &mut [positions[0], new_position]);
            win_count_1 += subsequent_wins[0] * roll_2_multiplier;
            win_count_2 += subsequent_wins[1] * roll_2_multiplier;
        }
    }

    [win_count_1, win_count_2]
}


fn part_2() -> u32 {
    
    let mut player_1: [u32; 2] = [4, 0];
    let mut player_2: [u32; 2] = [8, 0];

    let mut scores: [u64; 2] = [0, 0];
    let mut positions: [u64; 2] = [7, 5];

    let running_totals = player_1_roll(&mut scores, &mut positions);

    println!("{}, {}", running_totals[0], running_totals[1]);

    0
}


fn main() -> Result<(), Error> {

    println!("{}", part_1());

    println!("{}", part_2());

    Ok(())
}
