use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn exercise_1() -> i32 {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut counter: i32 = 0;
        let mut last: i32 = 999999;
        for line in lines {
            if let Ok(ip) = line {
                let i: i32 = ip.parse().unwrap_or(0);
                if i > last {
                    counter = counter + 1;
                }
                last = i;
            }
        }
        return counter;
    }
    return 0;
}

fn exercise_2() -> i32 {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut counter: i32 = 0;
        let mut last1: i32 = -1;
        let mut last2: i32 = -1;
        let mut last3: i32 = -1;
        let mut last_sum: i32 = 9999999;
        
        for line in lines {
            if let Ok(ip) = line {
                let i: i32 = ip.parse().unwrap_or(0);
                last3 = last2;
                last2 = last1;
                last1 = i;
                if last1 > 0 && last2 > 0 && last3 > 0 {
                    // we have our first "window"
                    let sum = last1 + last2 + last3;
                    if sum > last_sum {
                        counter = counter + 1;
                    }
                    last_sum = sum;
                }
            }
        }
        return counter;
    }
    return 0;
}

fn main() {
    println!("Exercise 1: {}", exercise_1()); 
    println!("Exercise 2: {}", exercise_2()); 
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>

where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
