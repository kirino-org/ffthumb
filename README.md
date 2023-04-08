
# ffthumb

Safe ffmpegthumbnailer wrapper for Rust

```rust
extern crate ffthumb;

use std::{
  fs::File,
  io::{stdout, Write},
};

use ffthumb::Thumbnailer;

fn main() {
  let mut th = Thumbnailer::builder().finalize();
  File::create("thumbnail.png")
    .unwrap()
    .write(
      th.generate("bad_apple.mkv", Some(25), None)
        .unwrap()
        .as_slice()
    )
    .unwrap();
}
```
