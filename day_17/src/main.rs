use regex::Regex;
use std::fs;
use std::io::Error;

fn read_target_area() -> [Point; 2] {

    let input_string = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"-?\d*").unwrap();

    assert!(re.is_match(&input_string));
    let coords: Vec<i32> = re.find_iter(&input_string)
                    .filter_map(|digits| digits.as_str().parse().ok())
                    .collect();
    [Point{x: coords[0], y:coords[2]}, Point{x:coords[1], y:coords[3]}]

}


struct Point {
    x: i32,
    y: i32
}


fn part_1(input: &[Point; 2]) -> i32 {

    let x_range = [input[0].x, input[1].x];
    let y_range = [input[0].y, input[1].y];

    let mut velocity = Point{x: 0, y: 0};
    let mut starting_velocity_at_greatest_height = Point{x:0, y:0};
    let mut greatest_height: i32 = 0;

    let mut count = 0;

    for i in 0..1000 {
        for j in -1000..1000 {

            let mut position = Point{x: 0, y: 0};
            let mut tmp_height: i32 = 0;

            velocity.x = i;
            velocity.y = j;
            
            loop {
                position.x += velocity.x;
                position.y += velocity.y;
                
                if position.y > tmp_height {
                    tmp_height = position.y;
                }

                if velocity.x > 0 {
                    velocity.x -= 1;
                } else if velocity.x < 0 {
                    velocity.x += 1;
                } else {
                    ();
                }
                velocity.y -= 1;

                if x_range[0] <= position.x  && position.x <= x_range[1] && y_range[0] <= position.y && position.y <= y_range[1] {
                    
                    println!("valid at {}, {}", i, j);
                    count += 1;
                    
                    if tmp_height > greatest_height {
                        greatest_height = tmp_height;
                        starting_velocity_at_greatest_height = Point{x: i, y: j};
                    }
                    break;

                } else if position.x > x_range[1] || position.y < y_range[0] {
                    break;
                }
            }
        }
    }

    println!("{}, {}", starting_velocity_at_greatest_height.x, starting_velocity_at_greatest_height.y);
    println!("{} possible solutions", count);
    greatest_height
}


fn part_2(input: &[Point; 2]) -> u32 {

    0
}



fn main() -> Result<(), Error> {

    let input = read_target_area();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}