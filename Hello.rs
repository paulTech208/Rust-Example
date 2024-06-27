use std::io;

fn main() {
  let company_string = "TutorialsPoint";
  let rating_float = 4.5;
  let is_growing_boolean = true;
  let icon_char = "♥";

  println!("company name is:{}", company_string);
  println!("company name is:{}", rating_float);
  println!("company name is:{}", is_growing_boolean);
  println!("company name is:{}", icon_char);

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
}
