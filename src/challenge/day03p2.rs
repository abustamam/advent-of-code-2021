pub fn run() {

  let (input, mut _aoc) = super::get(3);
  let mut o2_lines: Vec<&str> = input.split("\n").collect();
  let mut co2_lines: Vec<&str> = input.split("\n").collect();
  let mut o2_len = o2_lines.len() as u32;
  let mut co2_len = co2_lines.len() as u32;
  
  let mut o2_idx = 0;
  let mut co2_idx = 0;
  let mut o2_ctr: Vec<u32> = Vec::new();
  let mut co2_ctr: Vec<u32> = Vec::new();

  while o2_len > 1 {
    for o2_line in o2_lines.iter() {
      let o2_lines_vec: Vec<char> = o2_line.chars().collect();
      let curr_char = o2_lines_vec[o2_idx];
      if curr_char.to_string() == "1" {
        if let Some(i) = o2_ctr.get(o2_idx) {
          o2_ctr[o2_idx] = i + 1;
        } else {
          o2_ctr.push(1);
        }      
      }
    }
    let o2_ctr_val = o2_ctr.get(o2_idx).unwrap();
    let o2_rating = if o2_ctr_val >= &(o2_len - o2_ctr_val) { "1" } else { "0" };
    o2_lines.retain(|l| l.chars().collect::<Vec<char>>().get(o2_idx).unwrap().to_string() == o2_rating);
    o2_len = o2_lines.len() as u32;
    o2_idx += 1;
  }

  while co2_len > 1 {
    for co2_line in co2_lines.iter() {
      let co2_lines_vec: Vec<char> = co2_line.chars().collect();
      let curr_char = co2_lines_vec[co2_idx];
      if curr_char.to_string() == "1" {
        if let Some(i) = co2_ctr.get(co2_idx) {
          co2_ctr[co2_idx] = i + 1;
        } else {
          co2_ctr.push(1);
        }      
      }
    }
    let co2_ctr_val = co2_ctr.get(co2_idx).unwrap();
    let co2_rating = if co2_ctr_val >= &(co2_len - co2_ctr_val) { "0" } else { "1" };
    co2_lines.retain(|l| l.chars().collect::<Vec<char>>().get(co2_idx).unwrap().to_string() == co2_rating);
    co2_len = co2_lines.len() as u32;
    
    co2_idx += 1;
  }
  let o2_str = o2_lines[0]; 
  let co2_str = co2_lines[0]; 
  let o2_int = isize::from_str_radix(o2_str, 2).unwrap();
  let co2_int = isize::from_str_radix(co2_str, 2).unwrap();
  
  println!("day03p2: {}", o2_int * co2_int );
}
