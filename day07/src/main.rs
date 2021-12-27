use std::fs;

fn main() {
    part_1();
    // part_2();
}


fn part_1() {
    let mut positions: Vec<i32> = Vec::new();

    let contents = fs::read_to_string("input").expect("Error while reading file");

    for crabs in contents.trim().split(',') {
        let position: i32 = crabs.parse().expect("Couldn't parse number");

        insert_ordered(&mut positions, position);
    }

    // find median
    let length = positions.len();

    let half = length/2;
    let median = positions[half];

    let mut cost = 0;

    for i in positions {
        cost += (median-i).abs();
    }

    println!("total crabs: {}", length);
    println!("Fuel cost: {}", cost);
}

fn insert_ordered(positions: &mut Vec<i32>, position: i32) {
    if positions.is_empty() {
        positions.push(position);
        return;
    }
    for i in 0..positions.len() {
        if position < positions[i] {
            positions.insert(i, position);
            return;
        }
    }
    positions.push(position);
}

