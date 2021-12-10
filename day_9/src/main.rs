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


fn part_1(input: &Vec<Vec<u32>>) -> u32 {

    let mut running_total: u32 = 0;

    for (i, line) in input.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {

            let up: u32;
            let down: u32;
            let left: u32;
            let right: u32;

            match i {
                0 => {
                    up = 10;
                    down = input[i+1][j];
                },
                99 => {
                    up = input[i-1][j];
                    down = 10;
                },
                _ => {
                    up = input[i-1][j];
                    down = input[i+1][j];
                },
            }
            match j {
                0 => {
                    left = 10;
                    right = input[i][j+1];
                },
                99 => {
                    left = input[i][j-1];
                    right = 10;
                },
                _ => {
                    left = input[i][j-1];
                    right = input[i][j+1];
                },             
            }
            
            if (value < &up) && (value < &down) & (value < &left) & (value < &right) {
                running_total += value+1
            }
        }
    }

    running_total
}

#[derive(Copy, Clone)]
struct Point {
    height: u32,
    x: usize,
    y: usize
}


struct PathSection {
    size: u32
}


impl PathSection {

    fn new(start_point: Point, map: &Vec<Vec<u32>>, visited_list: &mut Vec<(usize, usize)>, size: &mut u32) -> Self {
        
        visited_list.push((start_point.x, start_point.y));
        // println!("Pushing {}, {} to visited list", start_point.x, start_point.y);
        // for (x, y) in visited_list.iter() {
        //     println!("{}, {}", x, y);
        // }
        *size += 1;

        let i = start_point.y;
        let j = start_point.x;
        // println!("{}", i);
        // println!("{}", j);
        // println!("------------------");
        let i_buf: usize = cmp::max((i as i32)-1, 0) as usize;
        let j_buf: usize = cmp::max((j as i32)-1, 0) as usize;

        let mut next_steps: Vec<Point> = vec![];

        let mut up_point = Point{height: 10, x: j, y: i_buf};
        let mut down_point = Point{height: 10, x: j, y: i+1};
        let mut left_point = Point{height: 10, x: j_buf, y: i};
        let mut right_point = Point{height: 10, x: j+1, y: i};  

        match i {
            0 => {
                down_point.height = map[i+1][j];
            },
            99 => {
                up_point.height = map[i-1][j];
            },
            _ => {
                up_point.height = map[i-1][j];
                down_point.height = map[i+1][j];
            },
        }
        match j {
            0 => {
                right_point.height = map[i][j+1];
            },
            99 => {
                left_point.height = map[i][j-1];
            },
            _ => {
                left_point.height = map[i][j-1];
                right_point.height = map[i][j+1];
            },             
        }

        for point in vec![up_point, down_point, left_point, right_point].iter() {

            
             if (point.height > start_point.height) 
                 && (point.height < 9) 
                 && (!visited_list.iter().any(|i| *i == (point.x, point.y))) {
                     next_steps.push(*point);
                     // println!("---{}, {}---", point.x, point.y);
             }
             
        }

        for point in next_steps.iter() {
            if !visited_list.iter().any(|i| *i == (point.x, point.y)) {
                PathSection::new(*point, &map, visited_list, size);
            }
        }

        PathSection{size:*size}

    }
}


fn part_2(input: &Vec<Vec<u32>>) -> u32 {

    let mut low_point_locations: Vec<Point> = vec![];

    for (i, line) in input.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {

            let up: u32;
            let down: u32;
            let left: u32;
            let right: u32;

            match i {
                0 => {
                    up = 10;
                    down = input[i+1][j];
                },
                99 => {
                    up = input[i-1][j];
                    down = 10;
                },
                _ => {
                    up = input[i-1][j];
                    down = input[i+1][j];
                },
            }
            match j {
                0 => {
                    left = 10;
                    right = input[i][j+1];
                },
                99 => {
                    left = input[i][j-1];
                    right = 10;
                },
                _ => {
                    left = input[i][j-1];
                    right = input[i][j+1];
                },             
            }
            
            if (value < &up) && (value < &down) & (value < &left) & (value < &right) {
                low_point_locations.push(Point{height: *value, x: j, y: i});
            }
        }
    }

    let mut basin_sizes: Vec<u32> = vec![];

    for low_point in low_point_locations.iter() {
        let mut visited_list: Vec<(usize, usize)> = vec![];
        let mut size: u32 = 0;
        let basin = PathSection::new(*low_point, input, &mut visited_list, &mut size);
        basin_sizes.push(basin.size);
        // println!("======================");
    }

    basin_sizes.sort();
    let n = basin_sizes.len();

    basin_sizes[n-1]*basin_sizes[n-2]*basin_sizes[n-3]
}


fn main() -> Result<(), Error> {

    let input = read_vector_of_vectors_from_file();

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}
