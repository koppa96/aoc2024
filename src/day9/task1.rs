use crate::day9::common::{checksum, read_fs};
use crate::utils;

pub fn solve(input_path: String) -> utils::Result {
  let mut fs = read_fs(input_path)?;
  compact(&mut fs);
  println!("{}", checksum(&fs));

  Ok(())
}

fn compact(fs: &mut Vec<Option<u32>>) {
  let mut i = 0;
  let mut j = fs.len() - 1;

  while i < j {
    while i < j && fs[i] != None {
      i += 1;
    }

    while i < j && fs[j] == None {
      j -= 1;
    }

    if i >= j {
      break;
    }

    fs[i] = fs[j];
    fs[j] = None;
  }
}
