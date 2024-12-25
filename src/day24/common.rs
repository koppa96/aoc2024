use std::collections::HashMap;

pub trait Resolver {
  fn resolve(&self, wires: &HashMap<String, Box<dyn Resolver>>) -> bool;
  fn as_const(&self) -> Option<&Const>;
  fn as_binary_operation(&self) -> Option<&BinaryOperation>;
}

pub struct Const {
  pub value: bool,
}

impl Resolver for Const {
  fn resolve(&self, _: &HashMap<String, Box<dyn Resolver>>) -> bool {
    self.value
  }

  fn as_const(&self) -> Option<&Const> {
    Some(self)
  }

  fn as_binary_operation(&self) -> Option<&BinaryOperation> {
    None
  }
}

pub struct BinaryOperation {
  pub lhs: String,
  pub rhs: String,
  pub op: Operator,
}

impl Resolver for BinaryOperation {
  fn resolve(&self, wires: &HashMap<String, Box<dyn Resolver>>) -> bool {
    let lhs_val = wires[&self.lhs].resolve(wires);
    let rhs_val = wires[&self.rhs].resolve(wires);

    self.op.apply(lhs_val, rhs_val)
  }

  fn as_const(&self) -> Option<&Const> {
    None
  }

  fn as_binary_operation(&self) -> Option<&BinaryOperation> {
    Some(self)
  }
}

pub enum Operator {
  And,
  Or,
  Xor,
}

impl Operator {
  pub fn apply(&self, lhs: bool, rhs: bool) -> bool {
    match self {
      Operator::And => lhs && rhs,
      Operator::Or => lhs || rhs,
      Operator::Xor => lhs ^ rhs,
    }
  }
}
