extern crate cc;
extern crate pkg_config;

use cc::Build;
use pkg_config::Config;

fn main() {
  Build::new().file("src/bindings.c").compile("ffw");
  Config::new()
    .statik(true)
    .probe("libffmpegthumbnailer")
    .unwrap();
}
