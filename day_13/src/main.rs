use regex::Regex;


use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::FromIterator,
};


fn read_dots_and_folds_from_file() -> (Vec<Point>, Vec<Point>) {

    let mut folds: Vec<Point> = vec![];
    let mut dots: Vec<Point> = vec![];

    let br = BufReader::new(File::open("input.txt").unwrap());

    let dot_re = Regex::new(r",").unwrap();
    let fold_re = Regex::new(r"^fold along").unwrap();

    for line in br.lines() {
        if dot_re.is_match(&line.as_ref().unwrap()) {
            dots.push(Point{
                x: line.as_ref()
                        .unwrap()
                        .split(',')
                        .collect::<Vec<&str>>()[0]
                        .parse::<usize>()
                        .unwrap(), 
                y: line.as_ref()
                        .unwrap()
                        .split(',')
                        .collect::<Vec<&str>>()[1]
                        .parse::<usize>()
                        .unwrap()
            });

        } else if fold_re.is_match(&line.as_ref().unwrap()) {
            let relevant_chars: Vec<&str> = line.as_ref()
                                                .unwrap()
                                                .split_whitespace()
                                                .collect::<Vec<&str>>()[2]
                                                .split('=')
                                                .collect();
            
            if relevant_chars[0] == "x" {
                folds.push(Point{
                    x: relevant_chars[1].parse().unwrap(),
                    y: 0
                });

            } else {
                folds.push(Point{
                    x: 0,
                    y: relevant_chars[1].parse().unwrap()
                });
            }

        } else {
            ();
        }
    }
    (dots, folds)
}


struct Point {

    x: usize,
    y: usize
}


fn part_1(dots: &Vec<Point>, folds: &Vec<Point>) -> u32 {

    let mut max_coords: (usize, usize) = (0, 0);

    for point in dots.iter() {
        if point.x > max_coords.0 { max_coords.0 = point.x };
        if point.y > max_coords.1 { max_coords.1 = point.y };
    }

    let mut dot_map: Vec<Vec<bool>> = vec![vec![false; max_coords.0 + 1]; max_coords.1 + 1];

    for dot in dots.iter() {
        dot_map[dot.y][dot.x] = true;
    }

    //for fold in folds.iter()[0] {

    let fold = &folds[0];

    if fold.x == 0 {
        for i in 0..(fold.y+1) {
            for j in 0..dot_map[0].len() {
                dot_map[i][j] = dot_map[i][j] || dot_map[2*(fold.y)-i][j];
                dot_map[2*(fold.y)-i][j] = false;
            }
        }
    } else {
        for j in 0..(fold.x+1) {
            for i in 0..dot_map.len() {
                dot_map[i][j] = dot_map[i][j] || dot_map[i][2*(fold.x)-j];
                dot_map[i][2*(fold.x)-j] = false;
            }
        }
    }
    //}

    let mut dot_count: u32 = 0;

    for line in dot_map.iter() {
        for dot in line {
            if *dot {dot_count += 1};
        }
    }

    dot_count
}


fn part_2(dots: &Vec<Point>, folds: &Vec<Point>) -> u32 {

    let mut max_coords: (usize, usize) = (0, 0);

    for point in dots.iter() {
        if point.x > max_coords.0 { max_coords.0 = point.x };
        if point.y > max_coords.1 { max_coords.1 = point.y };
    }

    let mut dot_map: Vec<Vec<bool>> = vec![vec![false; max_coords.0 + 1]; max_coords.1 + 1];

    for dot in dots.iter() {
        dot_map[dot.y][dot.x] = true;
    }

    for fold in folds.iter() {

        if fold.x == 0 {
            for i in 0..(fold.y+1) {
                for j in 0..dot_map[0].len() {
                    dot_map[i][j] = dot_map[i][j] || dot_map[2*(fold.y)-i][j];
                    dot_map[2*(fold.y)-i][j] = false;
                }
            }
        } else {
            for j in 0..(fold.x+1) {
                for i in 0..dot_map.len() {
                    dot_map[i][j] = dot_map[i][j] || dot_map[i][2*(fold.x)-j];
                    dot_map[i][2*(fold.x)-j] = false;
                }
            }
        }
    }

    let mut readable_dot_map: Vec<Vec<char>> = vec![vec!['.'; max_coords.0 + 1]; max_coords.1 + 1];

    for (i, line) in dot_map.iter().enumerate() {
        for (j, dot) in line.iter().enumerate() {
            if *dot {readable_dot_map[i][j] = '#'};
        }
    }

    for line in &readable_dot_map[0..10] {
        let str_line: String = String::from_iter(&line[0..60]);
        println!("{}", str_line);
    }

    0
}


fn main() -> Result<(), Error> {

    let (dots, folds) = read_dots_and_folds_from_file();

    println!("{}", part_1(&dots, &folds));

    println!("{}", part_2(&dots, &folds));

    Ok(())
}