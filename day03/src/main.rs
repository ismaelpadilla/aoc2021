use std::fs;

// https://adventofcode.com/2021/day/3
fn main() {
    let contents = fs::read_to_string("input")
        .expect("Error while reading file");

    let first_line = contents.lines().next().expect("There should be at least one line");
    let word_length = first_line.len();

    // count appearences of 0 and 1. 0 substracts and 1 adds to the count
    // in the end, a negative count means that the most common bit was 0.
    let mut count = vec![0; word_length];
    
    for line in contents.lines() {
        let mut index = 0;
        for char in line.chars() {
            //match 

            match char {
                '0' => { count[index] -= 1 },
                '1' => { count[index] += 1 },
                _ => panic!("Expected 0 or 1")
            }
            index = (index + 1) % word_length;
        }
    }
    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();

    for c in count.iter() {
        if c > &0 {
            gamma_rate = gamma_rate + "1";
            epsilon_rate = epsilon_rate + "0";
        } else {
            gamma_rate = gamma_rate + "0";
            epsilon_rate = epsilon_rate + "1";
        }
    }

    let gamma_parsed = u32::from_str_radix(&gamma_rate, 2).expect("Could not parse binary number");
    let epsilon_parsed = u32::from_str_radix(&epsilon_rate, 2).expect("Could not parse binary number");

    println!("Gamma rate is: {} in binary", gamma_rate);
    println!("Gamma rate is: {}", gamma_parsed);
    println!("Epsilon rate is: {} in binary", epsilon_rate);
    println!("Epsilon rate is: {}", epsilon_parsed);

    let result = gamma_parsed * epsilon_parsed;
    println!("Power consumption is: {}", result);
}
