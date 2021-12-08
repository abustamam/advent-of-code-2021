fn summation(num: i64) -> i64 {
  return num * (num + 1) / 2;
}

fn check(input: &Vec<i64>, num: i64) -> i64 {
  return input.iter().fold(0, |acc, x| acc + summation((x - num).abs()))
}

pub fn run() {
//    let input = "16,1,2,0,4,2,7,1,2,14";
   let (input, mut _aoc) = super::get(7);
    let parsed: Vec<i64> = input.lines().nth(0).unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    println!("test: {}", parsed.iter().map(|&id| id.to_string() + ",").collect::<String>());
    let mut counter = 0;
    let mut curr = check(&parsed, counter);
    counter += 1;
    let mut next = check(&parsed, counter);
    while next < curr {
      curr = next;
      counter += 1;
 //     println!("counter: {}, {}, {}", counter, curr, next);
      next = check(&parsed, counter);
    } 
  println!("day07p2: {}", curr);
}
