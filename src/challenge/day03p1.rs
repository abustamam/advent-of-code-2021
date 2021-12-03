pub fn run() {
  let (input, mut _aoc) = super::get(3);
  let mut vec: Vec<u32> = Vec::new();
  let len = input.lines().count() as u32;
  let mut gamma_vec: Vec<&str> = Vec::new();
  let mut epsilon_vec: Vec<&str> = Vec::new();
  for line in input.lines() {
    for (idx, c) in line.chars().enumerate() {
      if c.to_string() == "1" {
        if let Some(i) = vec.get(idx) {
          vec[idx] = i + 1;
        } else {
          vec.push(1);
        }
      }
    }
  }
  
  for num in vec.iter() {
    let g = if num > &(len - num) { "1" } else { "0" };
    gamma_vec.push(g);
    let e = if num > &(len - num) { "0" } else { "1" };
    epsilon_vec.push(e);
  } 

  let gamma_str = &gamma_vec.join("");
  let epsilon_str = &epsilon_vec.join("");
  let gamma_int = isize::from_str_radix(gamma_str, 2).unwrap();
  let epsilon_int = isize::from_str_radix(epsilon_str, 2).unwrap();

  println!("day03p1: {}", gamma_int * epsilon_int);
}
