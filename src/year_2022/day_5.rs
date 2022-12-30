use std::fs;
use itertools::Itertools;

#[derive(Debug)]
pub struct Instruction {
  amount: usize,
  from: usize,
  to: usize,
}

pub fn solve(file_path: &String) {
  let input = fs::read_to_string(file_path)
    .expect("Error while reading the file");

  let (left, instructions_list) =
    input.split_once("\n\n").unwrap();

  // parse crates.
  let (stacks_stats, platforms) =
    left.rsplit_once('\n').unwrap();

  let num_of_stacks: usize = platforms.split_whitespace().last().unwrap().parse().ok().unwrap();
  let mut stacks = vec![Vec::<char>::new(); num_of_stacks];

  for line in stacks_stats.lines().rev() {
    for (index, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
      let cratee = chunk.nth(1).unwrap();
      if cratee.is_alphabetic() {
        stacks[index].push(cratee);
      }
    }
  }

  // parse instructions.
  let mut instructions = Vec::new();
  for line in instructions_list.lines() {
    let rest = line.strip_prefix("move ").unwrap();
    let (amount, rest) = rest.split_once(" from ").unwrap();
    let (from, to) = rest.split_once(" to ").unwrap();
    let instruction = Instruction {
      amount: amount.parse().ok().unwrap(),
      from: from.parse::<usize>().ok().unwrap() - 1,
      to: to.parse::<usize>().ok().unwrap() - 1,
    };
    instructions.push(instruction);
  }

  // part 1.
  let mut stacks1 = stacks.clone();
  for Instruction { amount, from, to} in &instructions {
    for _ in 0..*amount {
      if let Some(removed) = stacks1[*from].pop() {
        stacks1[*to].push(removed);
      }
    }
  }

  let result1: String = stacks1.into_iter()
    .filter_map(|stack| stack.into_iter().last())
    .collect();

  println!("Result is: {result1}");

  // part 2.
  for Instruction { amount, from, to } in instructions {
    let from_stack_len = stacks[from].len();
    let removed_part = stacks[from].split_off(from_stack_len - amount);
    stacks[to].extend(removed_part);
  }

  let result2: String = stacks.into_iter()
    .filter_map(|stack| stack.into_iter().last())
    .collect();

  println!("Result2 is {result2}");
}
