use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

// https://adventofcode.com/2021/day/5
fn main() {
    part_1();
    // part_2();
}

fn part_1() {
    let contents = fs::read_to_string("input").expect("Error while reading file");
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    let mut count: u32 = 0;
    for line in contents.lines() {
        let coords = parse_line(line);

        // redundant if :)
        if coords[0].0 == coords[1].0 || coords[0].1 == coords[1].1 {
            // println!("inserting {}", line);
            if coords[0].0 == coords[1].0 {
                // equal x coordinate
                for i in min(coords[0].1, coords[1].1)..=max(coords[0].1, coords[1].1) {
                    // println!("inserting {},{}", coords[0].0, i);
                    // insert_into_map(&mut map, (coords[0].0, i));
                    if insert_into_map(&mut map, (coords[0].0, i)) == 2 {
                        count += 1;
                    }
                }
            } else if coords[0].1 == coords[1].1 {
                // equal y coordinate
                for i in min(coords[0].0, coords[1].0)..=max(coords[0].0, coords[1].0) {
                    // println!("inserting {},{}", i, coords[1].1);
                    // insert_into_map(&mut map, (i, coords[1].1));
                    if insert_into_map(&mut map, (i, coords[1].1)) == 2 {
                        count += 1;
                    }
                }
            }
            // println!("{},{}", coords[0].0, coords[0].1);
            // println!("{},{}\n", coords[1].0, coords[1].1);
        }
    }

    println!("Count: {}", count);
    //
    // unnecessary double check :)
    let sum: u32 = map.values().filter(|v| v > &&1).map(|_| 1).sum();
    println!("Intersections: {}", sum);
}

// insert 1 into map if key doesn't exist, otherwise increment value by 1
fn insert_into_map(map: &mut HashMap<(u32, u32), u32>, key: (u32, u32)) -> u32 {
    // could be otpimized with unwrap_or(0)
    if !map.contains_key(&key) {
        map.insert(key, 1);
        return 1;
    } else {
        // println!("already existed!");
        let current_value = *map.get(&key).unwrap();
        map.insert(key, current_value + 1);
        return current_value + 1;
    }
}

// parse a line such as "0,9 -> 5,9" into two tuples
fn parse_line(line: &str) -> [(u32, u32); 2] {
    let split = line.split(" -> ");
    let collected: Vec<(u32, u32)> = split.map(|coord| parse_coord(coord)).collect();
    [collected[0], collected[1]]
}

// parse a string such as "5,9" into a tuple
fn parse_coord(coord: &str) -> (u32, u32) {
    let mut split = coord.split(",");
    let first = split.next().unwrap().parse().unwrap();
    let second = split.next().unwrap().parse().unwrap();
    (first, second)
}
