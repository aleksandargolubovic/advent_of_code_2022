use std::fs;
use phf::phf_map;

static SHAPE: phf::Map<&'static str, i32> = phf_map! {
  "X" => 1,
  "Y" => 2,
  "Z" => 3,
};

static SHAPE2: phf::Map<&'static str, i32> = phf_map! {
  "X" => 0,
  "Y" => 3,
  "Z" => 6,
};

static RESULTS: phf::Map<&'static str, i32> = phf_map! {
  "A X" => 3,
  "A Y" => 6,
  "A Z" => 0,
  "B X" => 0,
  "B Y" => 3,
  "B Z" => 6,
  "C X" => 6,
  "C Y" => 0,
  "C Z" => 3,
};

static RESULTS2: phf::Map<&'static str, &'static str> = phf_map! {
  "A X" => "Z",
  "A Y" => "X",
  "A Z" => "Y",
  "B X" => "X",
  "B Y" => "Y",
  "B Z" => "Z",
  "C X" => "Y",
  "C Y" => "Z",
  "C Z" => "X",
};


pub fn solve(file_path: &String) {
  let input = fs::read_to_string(file_path)
    .expect("Error while reading the file");

  let mut sum = 0;
  let mut sum2 = 0;
  for line in input.lines() {
    sum += RESULTS.get(line).cloned().unwrap();
    sum += SHAPE.get(line.rsplit_once(' ').unwrap().1).unwrap();

    // second task.
    sum2 += SHAPE2.get(line.rsplit_once(' ').unwrap().1).unwrap();
    sum2 += SHAPE.get(RESULTS2.get(line).cloned().unwrap()).unwrap();
  }

  println!("Results are {sum} and {sum2}");


}

