pub fn run() {
    let (input, mut _aoc) = super::get(1);
    let lines: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut counter: u32 = 0;

    let mut last1: i32 = -1;
    let mut last2: i32 = -1;
    let mut last3: i32;
    let mut last_sum: i32 = 9999999;
     
    for line in lines {
        last3 = last2;
        last2 = last1;
        last1 = line;
        if last1 > 0 && last2 > 0 && last3 > 0 {
            // we have our first "window"
            let sum = last1 + last2 + last3;
            if sum > last_sum {
                counter = counter + 1;
            }
            last_sum = sum;
        }
    }
    println!("day01p2: {}", counter);
}
