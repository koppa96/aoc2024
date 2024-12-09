use crate::day9::common::read_nums;
use crate::utils;
use std::io;

pub fn solve(input_path: String) -> utils::Result {
  let mut fs = read_fs(input_path)?;
  compact(&mut fs);
  println!("{}", checksum_fs(&fs));

  Ok(())
}

struct Segment {
  start: usize,
  len: usize,
  fid: Option<usize>,
}

fn read_fs(input_path: String) -> io::Result<Vec<Segment>> {
  let nums = read_nums(input_path)?;
  let mut fs: Vec<Segment> = Vec::new();
  let mut start_idx = 0;
  for i in 0..nums.len() {
    fs.push(Segment {
      start: start_idx,
      len: nums[i] as usize,
      fid: if i % 2 == 0 { Some(i / 2) } else { None },
    });

    start_idx += nums[i] as usize;
  }

  Ok(fs)
}

fn compact(fs: &mut Vec<Segment>) {
  for i in (0..fs.len()).rev() {
    if let Some(_) = fs[i].fid {
      if let Some(idx) = find_empty_segment_before(i, fs[i].len, fs) {
        fs[i].start = fs[idx].start;
        fs[idx].start += fs[i].len;
        fs[idx].len -= fs[i].len;
      }
    }
  }
}

fn find_empty_segment_before(idx: usize, len: usize, fs: &Vec<Segment>) -> Option<usize> {
  for i in 1..idx {
    if fs[i].fid == None && fs[i].len >= len {
      return Some(i);
    }
  }

  None
}

fn checksum_fs(fs: &Vec<Segment>) -> usize {
  let mut sum = 0;
  for segment in fs {
    if let Some(fid) = segment.fid {
      for i in 0..segment.len {
        sum += (segment.start + i) * fid;
      }
    }
  }

  sum
}
