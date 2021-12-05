use std::fs;

// https://adventofcode.com/2021/day/1
fn main() {
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


fn line_to_int(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}
