#![no_std]
#![doc = include_str!("../README.md")]

#[allow(unused)]
mod bindings;

use bindings::*;

#[derive(Clone)]
pub struct Thumbnailer {
  ctx: *mut video_thumbnailer,
}
impl Thumbnailer {
  pub fn new() -> Self {
    let ctx = unsafe { video_thumbnailer_create() };
    Self { ctx }
  }
  /// Generate thumbnail of **`input`**
  //  pub fn generate(&mut self, input: String) {
  //    unsafe {
  //        let buf = video_thumbnailer_create_image_data();
  //      video_thumbnailer_generate_thumbnail_to_buffer(
  //        self.ctx,
  //        input.as_bytes().as_ptr() as *const _,
  //        buf,
  //      );
  //        &std::ptr::slice_from_raw_parts((*buf).image_data_ptr as *const _,
  // (*buf).image_data_size.try_into().unwrap());    }
  //  }

  /// Generate thumbnail of **`input`** and write to file **`output`**
  pub fn generate_to_file(&mut self, input: &str, output: &str) {
    unsafe {
      video_thumbnailer_generate_thumbnail_to_file(
        self.ctx,
        input.as_bytes().as_ptr() as *const _,
        output.as_bytes().as_ptr() as *const _,
      );
    }
  }
  /// Set thumbnail size
  ///
  /// `0` to keep aspect ratio
  pub fn set_size(&mut self, width: i32, height: i32) {
    unsafe {
      video_thumbnailer_set_size(self.ctx, width, height);
    }
  }
  /// Set thumbnail quality
  ///
  /// **`quality`**: 1-10 (JPEG only)
  pub fn set_quality(&mut self, quality: u8) {
    unsafe {
      (*self.ctx).thumbnail_image_quality = quality.try_into().unwrap();
    }
  }
}
impl Drop for Thumbnailer {
  fn drop(&mut self) {
    unsafe {
      video_thumbnailer_destroy(self.ctx);
    }
  }
}
