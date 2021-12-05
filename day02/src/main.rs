use std::fs;

enum Direction {
    Forward,
    Up,
    Down
}

struct Movement {
    direction: Direction, 
    distance: i32
}

fn main() {
    part_1();
    part_2();
}

fn part_1(){
    let contents = fs::read_to_string("input")
        .expect("Error while reading file");

    let mut horizontal = 0;
    let mut vertical = 0;

    for line in contents.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        let movement = vector_to_movement(split);

        match movement.direction {
            Direction::Forward => { horizontal += movement.distance },
            Direction::Up => { vertical -= movement.distance },
            Direction::Down => { vertical += movement.distance },
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Vertical: {}", vertical);
    println!("Total distance: {}", horizontal * vertical);
}

fn part_2(){
    let contents = fs::read_to_string("input")
        .expect("Error while reading file");

    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;

    for line in contents.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        let movement = vector_to_movement(split);

        match movement.direction {
            Direction::Forward => { horizontal += movement.distance; 
                vertical += aim * movement.distance;
            },
            Direction::Up => { aim -= movement.distance },
            Direction::Down => { aim += movement.distance },
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Vertical: {}", vertical);
    println!("Total distance: {}", horizontal * vertical);
}


fn vector_to_movement(vector: Vec<&str>) -> Movement {
    let direction = match vector[0] {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("Expected a direction")
    };
    Movement {
        direction,
        distance: line_to_int(vector[1]),
    }
}

fn line_to_int(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}
