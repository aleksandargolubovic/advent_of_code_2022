
use std::fs;
use std::collections::HashSet;


pub fn solve(file_path: &String) {
  let input = fs::read_to_string(file_path)
    .expect("Error while reading the file");
    let mut sum = 0;
    for line in input.lines() {
      let (first, last) = line.split_at(line.len() / 2);
      let set: HashSet<char> = first.chars().collect();
      // last.chars().any(|c| set.contains(&c))

      let response: HashSet<char> = last.chars().filter(|c| set.contains(&c)).collect();
      for r in response {

        let sub =  if r.is_ascii_lowercase() { 96 } else { 38 };
        // let c = r;
        // let ascii = r as i32 - sub;
        // println!("char: {}, ascii: {}", c, ascii);
        sum += (r as i32) - sub;
      }

    }
    println!("Result 1, sum: {}", sum);

    let mut count = 0;
    sum = 0;
    let mut set: HashSet<char> = HashSet::new();
    for line in input.lines() {
      if count == 0 {
        set = line.chars().collect();

      } else {
        set = line.chars().filter(|c| set.contains(&c)).collect();
        if count == 2 {
          for s in &set {
            let sub =  if s.is_ascii_lowercase() { 96 } else { 38 };
            let c = *s;
            let ascii = *s as i32 - sub;
            println!("char: {}, ascii: {}", c, ascii);
            sum += (*s as i32) - sub;
          }
          set.clear();
        }

      }

      count += 1;
      count %= 3;
    }


    println!("Result 2, sum = {}", sum);
}
