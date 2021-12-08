use regex::Regex;

pub fn run() {
  let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
  // let (input, mut _aoc) = super::get(3);
  let lines: Vec<&str> = input.split("\n").collect();
  let call: Vec<u32> = lines.get(0).unwrap().split(",").map(|x| x.parse().unwrap()).collect();
  let rows = &lines[2..];
  let boards: Vec<Vec<Vec<u32>>>;
  let mut rows_iter = rows.iter();
  let space_regex = Regex::new(r"\D").expect("Invalid regex");
  while let Some(row) = rows_iter.next() {
    if row == &"" {
      rows_iter.nth(1);
      continue;
    }
    println!("row {}", row);
    let splits: Vec<_> = space_regex.split(row).into_iter().collect();
    //let nums: Vec<u32> = row.split(" ").map(|x| x.parse().unwrap()).collect();
  }

  println!("call: {}", call.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
  println!("boards: {}", rows.join("\n"));
  // println!("day03p1: {}", "");
}
