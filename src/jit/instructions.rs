use crate::jit::Runner;

impl Runner {
  // Builtin functions
  pub fn add(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(a + b);
  }

  pub fn sub(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(b - a);
  }

  pub fn dup(&mut self) {
    let a = self.pop();
    self.push(a);
    self.push(a);
  }

  pub fn swap(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(a);
    self.push(b);
  }

  pub fn r#move(&mut self) {
    let a = self.pop();
    self.left = !self.left;
    self.push(a);
    self.left = !self.left;
  }

  pub fn over(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(b);
    self.push(a);
    self.push(b);
  }

  pub fn shr(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(b >> a);
  }

  pub fn shl(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(b << a);
  }

  pub fn eq(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push((a == b) as usize);
  }

  pub fn or(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(a | b);
  }

  pub fn and(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push(a & b);
  }

  pub fn gt(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push((a > b) as usize);
  }

  pub fn lt(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push((a < b) as usize);
  }

  pub fn gte(&mut self) {
    let a = self.pop();
    let b = self.pop();
    self.push((a >= b) as usize);
  }

  pub fn jmpif(&mut self) {
    let addr = self.pop();
    if self.pop() == 1 {
      self.ip = addr;
    } else {
      self.ip += 1;
    }
  }

  pub fn jmp(&mut self) {
    self.ip = self.pop();
  }

  pub fn print(&mut self) {
    println!("{}", self.pop());
  }

  pub fn trace(&mut self) {
    let mut out = "".to_owned();
    for n in self.deque.clone() {
      if n == 1 {
        out += "*";
      } else {
        out += " ";
      }
    }
    println!("{}", out);
  }
}
