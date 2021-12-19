use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};


fn read_input_to_vec_vec_point3() -> Vec<Vec<Point3Vec>> {

    let br = BufReader::new(File::open("input.txt").unwrap());

    let mut outer_vec: Vec<Vec<Point3Vec>> = vec![];
    let mut inner_vec: Vec<Point3Vec> = vec![];

    let iter = br.lines().into_iter().skip(1);

    for line in iter {

        let tmp = line.unwrap();

        let vals = tmp.split(',').collect::<Vec<&str>>();

        if vals.len() == 3 {
            inner_vec.push(Point3Vec::new( Point3{ i:vals[0].parse().unwrap(), 
                                                   j:vals[1].parse().unwrap(), 
                                                   k:vals[2].parse().unwrap() }));
        } else {
            if inner_vec.len() > 0 {
                outer_vec.push(inner_vec);
                inner_vec = vec![];
            } else {
                ();
            }
        }
    }

    outer_vec.push(inner_vec);

    outer_vec
}

#[derive(Copy, Clone)]
struct Point3 {
    i: i32,
    j: i32,
    k: i32
}

impl Point3 {

    fn multiply_by_mat3(&self, matrix: Mat3) -> Self {

        let mut new_point3 = Point3{ i: 0, j: 0, k: 0 };

        new_point3.i = matrix.a.i * self.i + matrix.a.j * self.j + matrix.a.k * self.k;
        new_point3.j = matrix.b.i * self.i + matrix.b.j * self.j + matrix.b.k * self.k;
        new_point3.k = matrix.c.i * self.i + matrix.c.j * self.j + matrix.c.k * self.k;

        new_point3
    }

    fn is_equal(&self, other: Point3) -> bool {

        self.i == other.i && self.j == other.j && self.k == other.k
    }
}

#[derive(Copy, Clone)]
struct Mat3 {
    a: Point3,
    b: Point3,
    c: Point3
}

impl Mat3 {

    fn new(ai: i32, aj: i32, ak: i32, bi: i32, bj: i32, bk: i32, ci: i32, cj: i32, ck: i32) -> Self {
        
        Mat3{a: Point3{i:ai, j:aj, k:ak}, 
             b: Point3{i:bi, j:bj, k:bk}, 
             c: Point3{i:ci, j:cj, k:ck}}
    }
}


struct Point3Vec {

    values: Vec<Point3>,
}

impl Point3Vec {

    fn new(initial_value: Point3) -> Self {

        let base = Mat3::new(1, 0, 0, 0, 1, 0, 0, 0, 1);

        let x_90 = Mat3::new(1, 0, 0, 0, 0, 1, 0, -1, 0);
        let y_90 = Mat3::new(0, 0, 1, 0, 1, 0, -1, 0, 0);
        let z_90 = Mat3::new(0, -1, 0, 1, 0, 0, 0, 0, 1);
        let xy90 = Mat3::new(0, 1, 0, 0, 0, -1, -1, 0, 0);
        let yx90 = Mat3::new(0, 0, 1, 1, 0, 0, 0, 1, 0);

        let x180 = Mat3::new(1, 0, 0, 0, -1, 0, 0, 0, -1);
        let y180 = Mat3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1);
        let z180 = Mat3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1);

        let off_axis_rotations: Vec<Mat3> = vec![base, x_90, y_90, z_90, xy90, yx90];
        let on_axis_rotations: Vec<Mat3> = vec![base, x180, y180, z180];

        let mut to_values: Vec<Point3> = vec![];

        for i in off_axis_rotations.iter() {
            for j in on_axis_rotations.iter() {
                to_values.push(initial_value.multiply_by_mat3(*i).multiply_by_mat3(*j));
            }
        }

        Point3Vec{ values: to_values }
    }

}


fn match_scanner_results(scanner_1: &Vec<Point3Vec>, scanner_2: &Vec<Point3Vec>, i_orientation: &usize) -> (bool, Point3, usize) {

    let mut cloned_scanner_1 = scanner_1.clone();
    let mut cloned_scanner_2 = scanner_2.clone();

    let mut candidate_offsets: Vec<(Point3, usize)> = vec![];

    for beacon_1 in scanner_1.iter() {
        for beacon_2 in scanner_2.iter() {
            for orientation in 0..24 {

                candidate_offsets.push((Point3{ i: beacon_1.values[*i_orientation].i - beacon_2.values[orientation].i,
                                               j: beacon_1.values[*i_orientation].j - beacon_2.values[orientation].j,
                                               k: beacon_1.values[*i_orientation].k - beacon_2.values[orientation].k},
                                                orientation));

            }
        }
    }

    for (offset, orientation) in candidate_offsets.iter() {

        //println!("orientation ndx: {}", orientation);
        //println!("Candidate offset: {}, {}, {}", offset.i, offset.j, offset.k);
        let mut matches: u32 = 0;

        for beacon_1 in scanner_1.iter() {
            for beacon_2 in cloned_scanner_2.iter() {
                //println!("beacon 2: {}, {}, {}", beacon_2.values[*orientation].i, beacon_2.values[*orientation].j, beacon_2.values[*orientation].k);

                let offset_beacon_2 = Point3{ i: beacon_2.values[*orientation].i + offset.i,
                                              j: beacon_2.values[*orientation].j + offset.j,
                                              k: beacon_2.values[*orientation].k + offset.k};

                if beacon_1.values[*i_orientation].is_equal(offset_beacon_2) {
                    matches += 1;
                    //println!("{}, {}, {} matches {}, {}, {} with offset of {}, {}, {}", beacon_1.values[0].i, beacon_1.values[0].j, beacon_1.values[0].k, beacon_2.values[*orientation].i, beacon_2.values[*orientation].j, beacon_2.values[*orientation].k, offset.i, offset.j, offset.k );
                }
            }
        }

        if matches > 11 {
            return(true, *offset, *orientation);
        }
    }

    (false, Point3{ i: 0, j: 0, k: 0 }, 0)
}


fn transform_and_count_unique(input: &Vec<Vec<Point3Vec>>, offsets: &Vec<Point3>, rotations: &Vec<usize>) -> u32 {

    let mut beacons_wrt_zero: Vec<Point3> = vec![];
    let mut unique_beacons_wrt_zero: Vec<Point3> = vec![];

    for (idx, scanner) in input.iter().enumerate() {
        for ndx in 0..scanner.len() {
            let beacon = Point3{ i: offsets[idx].i + scanner[ndx].values[rotations[idx]].i,
                                j: offsets[idx].j + scanner[ndx].values[rotations[idx]].j,
                                k: offsets[idx].k + scanner[ndx].values[rotations[idx]].k};

            beacons_wrt_zero.push(beacon);
        }
    }

    unique_beacons_wrt_zero.push(beacons_wrt_zero[0]);

    let mut included_flag = false;

    for beacon in beacons_wrt_zero.iter() {

        included_flag = false;

        for unique_beacon in unique_beacons_wrt_zero.iter() {

            if unique_beacon.is_equal(*beacon) {

                included_flag = true;
            }
        }

        if !included_flag {
            unique_beacons_wrt_zero.push(*beacon);
        }
    }
    for beacon in unique_beacons_wrt_zero.iter() {
        println!("{}, {}, {}", beacon.i, beacon.j, beacon.k);
    }


    unique_beacons_wrt_zero.len() as u32
}


fn part_1(input: &Vec<Vec<Point3Vec>>) -> u32 {

    let mut scanner_locations: Vec<Point3> = vec![Point3{ i: 0, j: 0, k: 0}; input.len()];

    let mut scanners_located: Vec<bool> = vec![false; input.len()];

    scanners_located[0] = true;
    let mut located_scanners_count: usize = 0;
    let mut checked_against: Vec<bool> = vec![false; input.len()];
    let mut orientations: Vec<usize> = vec![0; input.len()];

    while located_scanners_count < input.len()-1 {
        for i in 0..input.len() {
            if scanners_located[i] && !checked_against[i] {
                for j in 0..input.len() {
                    if !scanners_located[j] {
                        println!("{}, {}", i, j);

                        let scanner_i = &input[i];
                        let scanner_j = &input[j];

                        let (is_match, offset, orientation) = match_scanner_results(scanner_i, scanner_j, &orientations[i]);

                        if is_match {
                            located_scanners_count += 1;
                            scanners_located[j] = true;
                            scanner_locations[j] = Point3{ i: offset.i + scanner_locations[i].i,
                                                           j: offset.j + scanner_locations[i].j,
                                                           k: offset.k + scanner_locations[i].k};
                            
                            orientations[j] = orientation;

                            println!("found {} relative to {} at orientation {}", j, i, orientation);
                                                        
                        }
                    }
                }
                checked_against[i] = true;
            }
        }
    }

    for location in &scanner_locations {
        println!("{}, {}, {}", location.i, location.j, location.k);
    }

    println!("------------");

    transform_and_count_unique(&input, &scanner_locations, &orientations)
}


fn part_2(input: &Vec<Vec<Point3Vec>>) -> u32 {
    let s0: [i32; 3] = [0, 0, 0];
    let s1: [i32; 3] = [-2464, -3573, -1227];
    let s2: [i32; 3] = [-3722, -6019, -87];
    let s3: [i32; 3] = [-2540, -2433, -1155];
    let s4: [i32; 3] = [-1219, -3654, 1060];
    let s5: [i32; 3] = [22, -3572, -1159];
    let s6: [i32; 3] = [-1224, -4801, -1291];
    let s7: [i32; 3] = [-2544, -4815, -1258];
    let s8: [i32; 3] = [-94, -2376, -1345];
    let s9: [i32; 3] = [-4867, -3702, -2531];
    let s10: [i32; 3] = [-1333, -3618, -24];
    let s11: [i32; 3] = [-2434, -7343, 40];
    let s12: [i32; 3] = [-106, -3706, -2449];
    let s13: [i32; 3] = [-3620, -6069, -1317];
    let s14: [i32; 3] = [-2506, -3612, -2528];
    let s15: [i32; 3] = [-1221, -3606, -1168];
    let s16: [i32; 3] = [-2543, -4769, -88];
    let s17: [i32; 3] = [-3607, -3589, -2467];
    let s18: [i32; 3] = [-1233, -3560, -3625];
    let s19: [i32; 3] = [-3728, -7224, -1252];
    let s20: [i32; 3] = [-2476, -3556, -4761];
    let s21: [i32; 3] = [-2437, -7347, -1231];
    let s22: [i32; 3] = [-63, -1315, -111];
    let s23: [i32; 3] = [-2521, -3593, -3700];
    let s24: [i32; 3] = [-143, -2477, -86];
    let s25: [i32; 3] = [-2350, -5964, -1282];
    let s26: [i32; 3] = [-26, -1217, 1093];
    let s27: [i32; 3] = [-2418, -3746, 37];

    let s_locs = [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18, s19, s20, s21, s22, s23, s24, s25, s25, s26, s27];

    let mut max_dist = 0;

    for l in 0..10 {
        for i in 0..s_locs.len() {
            for j in 0..s_locs.len() {
                if i != j {
                    let distance = (s_locs[i][0]-s_locs[j][0]).abs() + (s_locs[i][1]-s_locs[j][1]).abs() + (s_locs[i][2]-s_locs[j][2]).abs();
                    if distance > max_dist {
                        max_dist = distance;
                        println!("{}", distance);
                        println!("{}, {}", i, j);
                    }
                }
            }
        }
    }
    
    max_dist as u32
}


fn main() -> Result<(), Error> {

    let input = read_input_to_vec_vec_point3();

    // println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    Ok(())
}
