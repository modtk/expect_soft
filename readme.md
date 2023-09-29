`.expect` but exit instead of panic

# Example usage

```rust
use expect_soft::ExpectSoft;
use std::fs;

fn main() {
  fs::read_to_string("./readme.md").expect_soft("readme.md does not exist!");
}
```