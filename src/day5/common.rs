pub struct Rule {
  first: i32,
  second: i32,
}

impl Rule {
  pub fn parse(str: &str) -> Rule {
    let parts: Vec<_> = str
      .split("|")
      .map(|part| part.parse::<i32>())
      .flatten()
      .collect();

    Rule {
      first: parts[0],
      second: parts[1],
    }
  }

  pub fn find_violating_indexes(&self, pages: &Vec<i32>) -> Option<(usize, usize)> {
    let mut first_idx: Option<usize> = None;
    let mut second_idx: Option<usize> = None;
    for (i, page) in pages.iter().enumerate() {
      if *page == self.first {
        first_idx = Some(i);
      } else if *page == self.second {
        second_idx = Some(i);
      }
    }

    if first_idx? < second_idx? {
      return None;
    }

    Some((first_idx?, second_idx?))
  }
}

pub fn find_first_violation(pages: &Vec<i32>, rules: &Vec<Rule>) -> Option<(usize, usize)> {
  for rule in rules {
    if let Some((first, second)) = rule.find_violating_indexes(pages) {
      return Some((first, second));
    }
  }

  None
}
