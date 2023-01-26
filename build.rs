#[cfg(feature = "bindgen")]
extern crate bindgen;
extern crate pkg_config;

use pkg_config::Config;

fn main() {
  #[cfg(feature = "bindgen")]
  bindgen::builder()
    .use_core()
    .rustified_enum("*")
    .generate_comments(true)
    .wrap_unsafe_ops(true)
    .header("/usr/include/libffmpegthumbnailer/videothumbnailerc.h")
    .generate()
    .unwrap()
    .write_to_file("src/bindings.gen.rs")
    .unwrap();
  Config::new()
    .statik(true)
    .probe("libffmpegthumbnailer")
    .unwrap();
}
