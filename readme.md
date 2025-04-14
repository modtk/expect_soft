# expect_soft

> This library has been retired in favor of Rust's native [`set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html).

A small utility that was designed to provide a softer alternative to Rust's `.expect()` method. Instead of panicking, it prints an error message and exits the program. This was made before I knew about the `set_hook` built-in function.

```rust
use expect_soft::ExpectSoft;
use std::fs;

fn main() {
  fs::read_to_string("./readme.md").expect_soft("readme.md could not be found");
}
```
