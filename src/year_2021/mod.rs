
pub mod day_1 {
    use std::fs;

  pub fn solve(file_path: &String) {
    let input = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    let mut vec: Vec<i32> = Vec::new();

    for line in input.lines() {
      // println!("{}", line);
      vec.push(line.parse().unwrap());
    }
    // println!("Input: \n{input}");
    println!("vec size {}", vec.len());
    // for e in &vec {
    //   println!("{}", e);
    // }

    let mut count = 0;
    for window in vec.windows(2) {
      if window[1] > window[0] {
        count += 1;
      }
    }
    // let mut iter = vec.windows(2);

    println!("Result is: {}", count);
  }

  pub fn solve2(file_path: &String) {
    let input: String = fs::read_to_string(file_path)
      .expect("path doesn't exist!");

    let mut vec: Vec<i32> = Vec::new();
    for line in input.lines() {
      vec.push(line.parse().unwrap());
    }

    let mut count = 0;
    for i in 0..vec.len() - 3 {
      if vec[i] < vec[i + 3] {
        count += 1;
      }
    }

    println!("Result is: {count}");
  }
}