use crate::utils::read_lines;

pub struct Input {
  pub list1: Vec<i32>,
  pub list2: Vec<i32>,
}

pub fn read_input(input_path: String) -> Result<Input, Box<dyn std::error::Error>> {
  let mut list1 = Vec::new();
  let mut list2 = Vec::new();

  for line in read_lines(input_path)? {
    let mut parts = line.split("   ");
    let first = parts
      .next()
      .ok_or("failed to get the first number")?
      .parse::<i32>()?;

    let second = parts
      .next()
      .ok_or("failed to get the second number")?
      .parse::<i32>()?;

    list1.push(first);
    list2.push(second);
  }

  Ok(Input { list1, list2 })
}
