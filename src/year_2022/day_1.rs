use std::fs;
use std::cmp;
use std::collections::BinaryHeap;


use std::{cmp::Ordering };

#[derive(PartialEq, Debug)]
struct MinNonNan(i32);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


pub fn solve(file_path: &String) {
  let input = fs::read_to_string(file_path)
    .expect("should have been able to read the file");

  let mut max_so_far = 0;
  let mut current_max = 0;
  for line in input.lines() {

    if line.is_empty() {
      println!("empty, next please");
      max_so_far = cmp::max(max_so_far, current_max);
      current_max = 0;
      continue;
    }

    current_max += line.parse::<i32>().unwrap();

  }
  max_so_far = cmp::max(max_so_far, current_max);
  println!("Result1 is: {max_so_far}");

  // second part of the task.
  current_max = 0;
  let mut heap = BinaryHeap::new();
  for line in input.lines() {
    if line.is_empty() {
      heap.push(MinNonNan(current_max));
      current_max = 0;
      if heap.len() > 3 {
        heap.pop();
      }
      continue;
    }

    current_max += line.parse::<i32>().unwrap();
  }
  heap.push(MinNonNan(current_max));
  if heap.len() > 3 {
    heap.pop();
  }

  println!("Result2 is:");

  let mut sum = 0;
  for h in &heap {
    sum += h.0;
    println!("{:?}", h);
  }
  println!("Sum is: {sum}");

}
