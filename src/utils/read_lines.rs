use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Flatten;

pub fn read_lines(file_path: String) -> Result<Flatten<Lines<BufReader<File>>>, io::Error> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  Ok(reader.lines().flatten())
}
