use std::fs;

// https://adventofcode.com/2021/day/1
fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("input")
        .expect("Error while reading file");

    let mut increases = 0;
    let mut prev_measurement :i32;

    let mut lines = contents.lines();

    prev_measurement = line_to_int(lines.next().unwrap()); 

    for line in contents.lines() {
        let current_measurement = line_to_int(line);
        if current_measurement > prev_measurement {
            increases += 1;
        }
        prev_measurement = current_measurement;
    }

    println!("Height increased {} times", increases);
}

fn part_2() {
    let mut increases = 0;

    let contents = fs::read_to_string("input")
        .expect("Error while reading file");

    let mut lines = contents.lines();

    let mut prev_window : [i32; 3] = [
        line_to_int(lines.next().unwrap()),
        line_to_int(lines.next().unwrap()),
        line_to_int(lines.next().unwrap())
    ];

    for line in contents.lines() {
        let current_measurement = line_to_int(line);
        let current_window = [prev_window[1], prev_window[2], current_measurement];

        if is_window_bigger(current_window, prev_window) {
            increases += 1;
        }

        prev_window = current_window;
    }

//    let collected: Vec<i32> = lines.map(|line| line_to_int(line)).collect();
//    for current_window in collected.chunks(3) {
//        if is_window_bigger(current_window, prev_window) {
//            increases += 1;
//        }
//        prev_window = current_window;
//    }

    println!("Height increased {} times", increases);
}

fn is_window_bigger(window_1 : [i32; 3], window_2 : [i32; 3]) -> bool {
    window_1.iter().sum::<i32>() > window_2.iter().sum::<i32>()
}

fn line_to_int(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}
