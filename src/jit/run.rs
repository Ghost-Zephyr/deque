use std::num::IntErrorKind::Empty;

use crate::jit::Runner;
use crate::BANG;

// Internal helper fucntions
pub trait CharsUtils {
  fn startswith(&self, x: char) -> bool;
  fn endswith(&self, x: char) -> bool;
}

impl CharsUtils for Vec<u8> {
  fn startswith(&self, x: char) -> bool {
    self[0] as char == x
  }
  fn endswith(&self, x: char) -> bool {
    self[self.len() -1] as char == x
  }
}

impl Runner {
  // JIT run a program
  pub fn run(&mut self) -> Result<(), String> {
    while self.ip < self.program.len() {
      let mut word = self.program[self.ip].to_owned();
      let bytes: Vec<u8> = word.as_bytes().to_vec();
      // println!("{}: {} <- deque({:?})", self.ip, self.program[self.ip].to_owned(), self.deque.clone());

      word = if bytes.startswith(BANG) {
        self.left = true;
        word[1..].to_owned()
      } else if bytes.endswith(BANG) {
        self.left = false;
        word[..word.len() -1].to_owned()
      } else if bytes.endswith(':') {
        self.ip += 1;
        continue
      } else {word};

      match word.as_str() {
        "drop" => {self.pop();},
        "add" => self.add(),
        "sub" => self.sub(),
        "dup" => self.dup(),
        "swap" => self.swap(),
        "move" => self.r#move(),
        "over" => self.over(),
        "shr" => self.shr(),
        "shl" => self.shl(),
        "eq" => self.eq(),
        "or" => self.or(),
        "and" => self.and(),
        ">=" => self.gte(),
        ">" => self.gt(),
        "<" => self.lt(),
        "jmpif" => {self.jmpif();continue},
        "jmp" => {self.jmp();continue},
        "print" => self.print(),
        "exit" => break,
        "trace" => self.trace(),
        s => match word.parse::<usize>() {
          Ok(n) => self.push(n),
          Err(err) => {
            if *err.kind() == Empty {
              dbg!(err);
            } else {
              match self.labels.get(&word) {
                Some(l) => {self.push(*l);}
                None => {
                  return Err(format!("Error identifier {} not recognized!", s));
                }
              };
            }
          }
        }
      }
      // dbg!(&self.ip, self.program[self.ip].to_owned(), self.deque.clone());
      self.ip += 1;
    }
    Ok(())
  }
}
