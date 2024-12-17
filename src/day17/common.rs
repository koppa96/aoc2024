use crate::utils::read_lines;
use regex::Regex;
use std::error::Error;

pub struct Program {
  pub register_a: i64,
  pub register_b: i64,
  pub register_c: i64,
  pub output: Vec<u8>,
  pub instructions: Vec<u8>,
  instruction_ptr: usize,
}

impl Program {
  pub fn parse(input_path: String) -> Result<Self, Box<dyn Error>> {
    let regex = Regex::new("\\d+")?;
    let mut lines = read_lines(input_path)?;

    let register_a: i64 = regex
      .find(&lines.next().unwrap())
      .ok_or("Failed to find the value for register_a.")?
      .as_str()
      .parse()?;
    let register_b: i64 = regex
      .find(&lines.next().unwrap())
      .ok_or("Failed to find the value for register_b.")?
      .as_str()
      .parse()?;
    let register_c: i64 = regex
      .find(&lines.next().unwrap())
      .ok_or("Failed to find the value for register_c.")?
      .as_str()
      .parse()?;

    lines.next();

    let line = lines.next().ok_or("Failed to read instruction line.")?;
    let data = line.split(": ").last().unwrap();
    let instructions: Vec<u8> = data.split(",").map(|i| i.parse()).flatten().collect();

    Ok(Self {
      register_a,
      register_b,
      register_c,
      output: Vec::new(),
      instructions,
      instruction_ptr: 0,
    })
  }

  pub fn run(&mut self) -> Result<(), String> {
    while self.instruction_ptr < self.instructions.len() {
      self.step()?;
    }

    Ok(())
  }

  fn step(&mut self) -> Result<(), String> {
    let literal = self.instructions[self.instruction_ptr + 1] as i64;
    let combo = self.resolve_operand_value()?;
    match self.instructions[self.instruction_ptr] {
      0 => {
        self.register_a = self.register_a / 2_i64.pow(combo as u32);
      }
      1 => {
        self.register_b = self.register_b ^ literal;
      }
      2 => self.register_b = combo % 8,
      3 => {
        if self.register_a != 0 {
          self.instruction_ptr = literal as usize;
          return Ok(());
        }
      }
      4 => {
        self.register_b = self.register_b ^ self.register_c;
      }
      5 => {
        self.output.push((combo % 8) as u8);
      }
      6 => {
        self.register_b = self.register_a / 2_i64.pow(combo as u32);
      }
      7 => {
        self.register_c = self.register_a / 2_i64.pow(combo as u32);
      }
      _ => {
        return Err(format!(
          "Unknown instruction: {}",
          self.instructions[self.instruction_ptr]
        ))
      }
    }

    self.instruction_ptr += 2;
    Ok(())
  }

  fn resolve_operand_value(&self) -> Result<i64, String> {
    let operand = self.instructions[self.instruction_ptr + 1];
    match operand {
      0..4 => Ok(operand as i64),
      4 => Ok(self.register_a),
      5 => Ok(self.register_b),
      6 => Ok(self.register_c),
      _ => Err(format!("Invalid operand value: {}", operand)),
    }
  }

  pub fn reset(&mut self, register_a: i64, register_b: i64, register_c: i64) {
    self.register_a = register_a;
    self.register_b = register_b;
    self.register_c = register_c;
    self.instruction_ptr = 0;
    self.output.clear();
  }
}
