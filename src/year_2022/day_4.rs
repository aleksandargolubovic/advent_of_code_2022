use std::fs;

pub fn solve(file_path: &String) {
  let input: String = fs::read_to_string(file_path)
    .expect("Error while reading the file");

  let mut count = 0;
  let mut count2 = 0;
  for line in input.lines() {
    let vec: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();
    let vec2: Vec<i32> =
      vec.into_iter().map(|v| v.parse::<i32>().unwrap()).collect();

    // task 1.
    if (vec2[0] <= vec2[2] && vec2[1] >= vec2[3]) ||
      (vec2[0] >= vec2[2] && vec2[1] <= vec2[3]) {

      count += 1;
    }

    // task 2.

    if (vec2[0] <= vec2[3] && vec2[1] >= vec2[2]) ||
      (vec2[2] <= vec2[1] && vec2[3] >= vec2[0]) {

      count2 += 1;
    }
  }
  // count2 += count;

  println!("Result 1 is {count} and result 2 is {count2}");
}