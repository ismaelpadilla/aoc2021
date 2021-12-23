use std::fs;

fn main() {
    part_1();
    // part_2();
}

fn part_1() {
    let total_days = 80;

    let contents = fs::read_to_string("input").expect("Error while reading file");

    // if lifetime[i] = n then there are n fish with a lifetime counter of i
    let mut lifetime = [0; 9];

    for number in contents.trim().split(',') {
        let time: usize = number.parse::<usize>().expect("Couldn't parse number");
        
        lifetime[time] += 1;
    }

    for _ in 0..total_days {
        let mut temp = lifetime[1];
        for i in (1..lifetime.len()).rev() {
            let temp2 = lifetime[i];
            lifetime[i] = temp;
            temp = temp2;
        }
        lifetime[6] += lifetime[0];
        lifetime[8] = lifetime[0];
        lifetime[0] = temp;
    }
    // let time: usize = "2".parse::<usize>().expect("Couldn't parse number");

    let total_fish: u32 = lifetime.iter().sum();
    println!("Total fish after {} days: {}", total_days, total_fish);
}

