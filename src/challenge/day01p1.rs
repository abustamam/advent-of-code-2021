pub fn run() {
    let (input, mut _aoc) = super::get(1);
    let lines: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut counter: u32 = 0;
    let mut last: u32 = 999999;
    for line in lines {
         if line > last {
            counter = counter + 1;
        }
        last = line;
    }
    println!("day01p1: {}", counter);
    _aoc.submit(&counter.to_string()).unwrap()
}

