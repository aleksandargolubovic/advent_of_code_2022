use std::env;
// use std::fs;

pub mod year_2021;
pub mod year_2022;


fn main() {
  let args: Vec<String> = env::args().collect();
  let year: i32 = args[1].parse().unwrap();
  let day: i32 = args[2].parse().unwrap();
  let file_path = &args[3];
  println!("In file {}", file_path);
  match year {
    2021 => run_2021(day, file_path),
    2022 => run_2022(day, file_path),
    _ => println!("Not supported!"),
  }
}

fn run_2021(day: i32, file_path: &String) {
  match day {
    1 => year_2021::day_1::solve(&file_path),
    2 => year_2021::day_1::solve2(&file_path),
    _ => println!("Not supported!"),
  }
}

fn run_2022(day: i32, file_path: &String) {
  match day {
    1 => year_2022::day_1::solve(&file_path),
    2 => year_2022::day_2::solve(&file_path),
    3 => year_2022::day_3::solve(&file_path),
    4 => year_2022::day_4::solve(&file_path),
    5 => year_2022::day_5::solve(&file_path),
    _ => println!("Not supported!"),
  }
}
