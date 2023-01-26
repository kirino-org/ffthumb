
# ffthumb

Safe ffmpegthumbnailer wrapper for Rust

```rust
fn main() {
  let th = Thumbnailer::new();
  th.set_size(256, 0);
  th.generate_to_file("test.mkv", "out.png");
}
```

 > This is part of [the Kirino Media Server project](https://kirino.io).

