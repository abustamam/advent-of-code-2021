pub fn run() {
  let (input, mut _aoc) = super::get(2);
  let mut h: i32 = 0;
  let mut v: i32 = 0;
  let mut aim: i32 = 0;
  for line in input.lines() {
    let mut split = line.split(" ");
    let dir = split.next().unwrap();
    let dist: i32 = split.next().unwrap().parse().unwrap();
    if dir == "forward" {
      h += dist;
      v += (dist * aim);
    }
    if dir == "down" {
      aim += dist
    }
    if dir == "up" {
      aim -= dist
    }
  }
  println!("day02p1: {}", h * v);
}
