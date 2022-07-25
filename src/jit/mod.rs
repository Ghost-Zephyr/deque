use std::collections::{VecDeque, HashMap};
use std::str;

pub mod instructions;
pub mod run;

#[derive(Default)]
pub struct Runner {
  pub program: Vec<String>,
  pub labels: HashMap<String, usize>,
  pub deque: VecDeque<usize>,
  pub left: bool,
  pub ip: usize,
}

fn labels(prog: Vec<String>) -> Result<HashMap<String, usize>, String> {
  let mut labels = HashMap::new();
  let mut ip = 0;

  while ip < prog.len() {
    let mut word = prog[ip].clone();
    let bytes = word.as_bytes();

    if bytes[bytes.len() -1] as char == ':' {
      word = str::from_utf8(
        &word.as_bytes()[..word.len()-1]
      ).unwrap().to_owned();

      if labels.get(&word) != None {
        return Err(format!("Label {} is already defined!", word));
      } else {
        labels.insert(word, ip);
      }
    }
    ip += 1;
  }
  Ok(labels)
}

impl Runner {
  pub fn new(input: String) -> Self {
    let program: Vec<String> = input.split_whitespace().map(
      |x| x.to_owned()).collect();
    Self {
      program: program.clone(),
      labels: labels(program).unwrap(),
      deque: VecDeque::new(),
      left: false,
      ip: 0,
    }
  }

  pub fn push(&mut self, v: usize) {
    if self.left {
      self.deque.push_front(v)
    } else {
      self.deque.push_back(v)
    }
  }

  pub fn pop(&mut self) -> usize {
    if self.left {
      self.deque.pop_front().unwrap()
    } else {
      self.deque.pop_back().unwrap()
    }
  }

  pub fn popn(&mut self, n: usize) -> Vec<usize> {
    let mut pops = vec![];
    for _ in 0..n {
      pops.push(self.pop())
    }
    pops
  }
}
