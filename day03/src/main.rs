use std::fs;

// https://adventofcode.com/2021/day/3
fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("input").expect("Error while reading file");

    let first_line = contents
        .lines()
        .next()
        .expect("There should be at least one line");
    let word_length = first_line.len();

    // count appearences of 0 and 1. 0 substracts and 1 adds to the count
    // in the end, a negative count means that the most common bit was 0.
    let mut count = vec![0; word_length];

    for line in contents.lines() {
        let mut index = 0;
        for char in line.chars() {
            match char {
                '0' => count[index] -= 1,
                '1' => count[index] += 1,
                _ => panic!("Expected 0 or 1"),
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
    let epsilon_parsed =
        u32::from_str_radix(&epsilon_rate, 2).expect("Could not parse binary number");

    println!("Gamma rate is: {} in binary", gamma_rate);
    println!("Gamma rate is: {}", gamma_parsed);
    println!("Epsilon rate is: {} in binary", epsilon_rate);
    println!("Epsilon rate is: {}", epsilon_parsed);

    let result = gamma_parsed * epsilon_parsed;
    println!("Power consumption is: {}", result);
}

fn part_2() {
    let contents = fs::read_to_string("input").expect("Error while reading file");

    let mut lines_oxygen: Vec<&str> = contents.lines().collect();
    let mut lines_co2: Vec<&str> = contents.lines().collect();

    let mut position: usize = 0;
    loop {
        let most_common = most_common_bit(&lines_oxygen, position, '1');
        // keep only lines with the most common bit on position
        lines_oxygen.retain(|l| l.chars().nth(position).unwrap() == most_common);
        position += 1;
        if lines_oxygen.len() == 1 {
            break;
        }
    }

    let mut position: usize = 0;
    loop {
        let least_common = least_common_bit(&lines_co2, position, '0');
        // keep only lines with the least common bit on position
        lines_co2.retain(|l| l.chars().nth(position).unwrap() == least_common);
        position += 1;
        if lines_co2.len() == 1 {
            break;
        }
    }

    println!("Oxygen: {}", lines_oxygen[0]);
    println!("CO2: {}", lines_co2[0]);

    let oxygen_parsed =
        u32::from_str_radix(&lines_oxygen[0], 2).expect("Could not parse binary number");
    let co2_parsed = u32::from_str_radix(&lines_co2[0], 2).expect("Could not parse binary number");

    let life_suport_rating = oxygen_parsed * co2_parsed;

    println!("Life support rating: {}", life_suport_rating);
}

fn most_common_bit(lines: &Vec<&str>, position: usize, default: char) -> char {
    let mut zero_count = 0;
    for line in lines {
        if line.chars().nth(position).unwrap() == '0' {
            zero_count += 1;
        }
    }
    if zero_count > lines.len() / 2 {
        return '0';
    } else if zero_count == lines.len() / 2 {
        return default;
    } else {
        return '1';
    }
}

fn least_common_bit(lines: &Vec<&str>, position: usize, default: char) -> char {
    let mut zero_count = 0;
    for line in lines {
        if line.chars().nth(position).unwrap() == '0' {
            zero_count += 1;
        }
    }
    if zero_count > lines.len() / 2 {
        return '1';
    } else if zero_count == lines.len() / 2 {
        return default;
    } else {
        return '0';
    }
}
