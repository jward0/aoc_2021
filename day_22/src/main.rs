use std::{
    cmp::min,
    cmp::max,
    fs::File,
    io::{BufRead, BufReader, Error},
};

extern crate regex;
use regex::Regex;

extern crate time;
use time::PreciseTime;


fn read_input() -> Vec<(bool, Point3, Point3)> {

    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut out_vec: Vec<(bool, Point3, Point3)> = vec![];


    let re = Regex::new(r"-?\d+").unwrap();
    let type_re = Regex::new(r"on").unwrap();

    let iter_lines = br.lines().into_iter();

    for line in iter_lines.map(|l| l.unwrap()) {

        let vals: Vec<i64> = re.find_iter(&line.as_str()).map(|v| v.as_str().parse().unwrap()).collect::<Vec<i64>>();

        let mut neg_point = Point3{ x: vals[0], y: vals[2], z: vals[4] };
        let mut pos_point = Point3{ x: vals[1], y: vals[3], z: vals[5] };

        let point_vals = [[neg_point.x, pos_point.x], [neg_point.y, pos_point.y], [neg_point.z, pos_point.z]];

        let mut do_not: bool = false;

        /*
        for pair in point_vals.iter() {
            if pair[0].abs() > 50 && pair[1].abs() > 50 {
                do_not = true;
            }
        }

        if neg_point.x < -50 { neg_point.x = -50; }
        if pos_point.x > 50 { pos_point.x = 50; }
        if neg_point.y < -50 { neg_point.y = -50; }
        if pos_point.y > 50 { pos_point.y = 50; }
        if neg_point.z < -50 { neg_point.z = -50; }
        if pos_point.z > 50 { pos_point.z = 50; }
        */
        let type_val: bool;
        if type_re.is_match(&line.as_str()) {
            type_val = true;
        } else {
            type_val = false;
        }

        if !do_not {
            println!("{}, {}, {}, {}, {}, {}", neg_point.x, pos_point.x, neg_point.y, pos_point.y, neg_point.z, pos_point.z);

            out_vec.push((type_val, neg_point, pos_point));
        }

    }

    out_vec
}


struct Point3 {
    x: i64,
    y: i64,
    z: i64
}

#[derive(PartialEq, Copy, Clone)]
struct Volume {

    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2]
} 

impl Volume {

    fn size(&self) -> u64 {

        ((self.x[1]+1 - self.x[0]) * (self.y[1]+1 - self.y[0]) * (self.z[1]+1 - self.z[0])).abs() as u64
    }

    fn find_non_intersection(self, new: Volume) -> Vec<Volume> {

        let mut out_vec: Vec<Volume> = vec![];

        //x goes left to right, y goes front to back, z goes bottom to top

        if new.x[1] >= self.x[0] && new.y[1] >= self.y[0]  && new.z[1] >= self.z[0] && new.x[0] <= self.x[1] && new.y[0] <= self.y[1] && new.z[0] <= self.z[1] {

            let x_possibilities = [[self.x[0], new.x[0]-1], [new.x[1]+1, self.x[1]], [max(new.x[0], self.x[0]), min(new.x[1], self.x[1])]];
            let y_possibilities = [[self.y[0], new.y[0]-1], [new.y[1]+1, self.y[1]], [max(new.y[0], self.y[0]), min(new.y[1], self.y[1])]];
            let z_possibilities = [[self.z[0], new.z[0]-1], [new.z[1]+1, self.z[1]], [max(new.z[0], self.z[0]), min(new.z[1], self.z[1])]];

            for (xdx, x) in x_possibilities.iter().enumerate() {
                if x[0] <= x[1] {
                    for (ydx, y) in y_possibilities.iter().enumerate() {
                        if y[0] <= y[1] {
                            for (zdx, z) in z_possibilities.iter().enumerate() {
                                if z[0] <= z[1] && xdx*ydx*zdx != 8 {
                                    out_vec.push(Volume{ x: *x, y: *y, z: *z });
                                } else{()}
                            }
                        } else{()}
                    }
                } else{()}
            }
        } else {
            out_vec.push(self);
        }

        out_vec
    }
}


struct VolumeCollection {

    data: Vec<Volume>
}

impl VolumeCollection {

    fn new() -> Self {

        VolumeCollection{ data: vec![] }
    }

    fn count(&self) -> u64 {

        let mut running_total: i64 = 0;

        for volume in &self.data {

            //println!("{}, {}, {}, {}, {}, {}", volume.x[0], volume.x[1], volume.y[0], volume.y[1], volume.z[0], volume.z[1]);
            running_total += ((volume.x[1]+1 - volume.x[0]) * (volume.y[1]+1 - volume.y[0]) * (volume.z[1]+1 - volume.z[0])) as i64;
            //println!("{}", running_total);
        }

        running_total.abs() as u64
    }
}


fn part_1(input: &Vec<(bool, Point3, Point3)>) -> u64 {

    let mut volumes = VolumeCollection::new();

    for (ndx, line) in input.iter().enumerate() {

        //println!("---Command {}------", ndx);

        let new_volume = Volume{x: [line.1.x, line.2.x], y: [line.1.y, line.2.y], z: [line.1.z, line.2.z]};

        if ndx == 0 {
            volumes.data.push(new_volume);
        } else {

            let mut new_volumes = VolumeCollection::new();

            //println!("New volume size: {}", new_volume.size());
            let mut intersection_size = 0;

            for volume in volumes.data {
                
                intersection_size += volume.size();

                let segmented_vols = volume.find_non_intersection(new_volume);
                //println!("Initial:");
                //println!("{}, {} | {}, {} | {}, {}", volume.x[0], volume.x[1], volume.y[0], volume.y[1], volume.z[0], volume.z[1]);
                //println!("Segments:");
                for sv in segmented_vols {
                    //println!("{}, {} | {}, {} | {}, {}", sv.x[0], sv.x[1], sv.y[0], sv.y[1], sv.z[0], sv.z[1]);
                    new_volumes.data.push(sv);
                    intersection_size -= sv.size();
                }
            }
            //println!("Intersection size of new volume: {}", intersection_size);
            if line.0 {
                new_volumes.data.push(new_volume);
            } else {
                ();
                //println!("Not adding");
            }
            volumes = new_volumes;

        }

        //println!("Volume number: {}", volumes.data.len());
        //for volume in volumes.data.iter() {
        //    println!("x: {}, {}, y: {}, {}, z: {}, {}", volume.x[0], volume.x[1], volume.y[0], volume.y[1], volume.z[0], volume.z[1]);
        //}
        //println!("Count: {}", volumes.count());

    }

    volumes.count()
}


fn part_2(input: &Vec<(bool, Point3, Point3)>) -> u64 {

    0
}


fn main() -> Result<(), Error> {

    let start = PreciseTime::now();

    let input = read_input();

    println!("{}", input.len());

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));

    let end = PreciseTime::now();

    println!("{}", start.to(end));

    Ok(())
}
