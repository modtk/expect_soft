# Archived as of 2025, use [set_hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html) instead

`.expect` but exit instead of panic

# Example usage

```rust
use expect_soft::ExpectSoft;
use std::fs;

fn main() {
  fs::read_to_string("./readme.md").expect_soft("readme.md does not exist!");
}
```
