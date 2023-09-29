pub trait ExpectSoft<T, E> {
  fn expect_soft(self, msg: &str) -> T;
  fn expect_err_soft(self, msg: &str) -> E;
}

impl<T, E: std::fmt::Debug> ExpectSoft<T, E> for Result<T, E> {
  fn expect_soft(self, msg: &str) -> T {
    self.unwrap_or_else(|_| {
      println!("FATAL: {}", msg);
      std::process::exit(1);
    })
  }
  
  fn expect_err_soft(self, msg: &str) -> E {
    self.err().unwrap_or_else(|| {
      println!("FATAL: {}", msg);
      std::process::exit(1);
    })
  }
}

impl<T> ExpectSoft<T, ()> for Option<T> {
  fn expect_soft(self, msg: &str) -> T {
    self.unwrap_or_else(|| {
      println!("FATAL: {}", msg);
      std::process::exit(1);
    })
  }
  
  fn expect_err_soft(self, msg: &str) -> () {
    if self.is_some() {
      println!("FATAL: {}", msg);
      std::process::exit(1);
    }
  }
}