// No test cases!! :'/

#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;
extern crate libc;

use alloc::{string::String, vec::Vec};
use core::{ptr::null_mut, slice};

use libc::c_void;

mod bindings;
use bindings::*;

pub struct ThumbnailerBuilder {
  ptr: *mut c_void,
  maintain_aspect_ratio: bool,
  film_overlay: bool,
  size: u32,
  time: u8,
}
impl ThumbnailerBuilder {
  /// Maintain aspect ratio
  ///
  /// If `false`, the thumbnail will be square (1:1).
  ///
  /// Defaults to `true`
  pub fn maintain_aspect_ratio(mut self, val: bool) -> Self {
    self.maintain_aspect_ratio = val;
    self
  }
  /// Film strip overlay
  ///
  /// Defaults to `false`
  pub fn film_overlay(mut self, val: bool) -> Self {
    self.film_overlay = val;
    self
  }
  /// Set thumbnail size
  ///
  /// Defaults to `128`
  pub fn size(mut self, val: u32) -> Self {
    self.size = val;
    self
  }
  /// Set seek time (percentage, 0-100)
  ///
  /// Defaults to `10`
  pub fn time(mut self, val: u8) -> Self {
    self.time = val;
    self
  }

  /// Finalize [Thumbnailer] configuration
  pub fn finalize(self) -> Thumbnailer {
    unsafe {
      ffw_thumbnailer_set_maintain_aspect_ratio(
        self.ptr,
        if self.maintain_aspect_ratio { 1 } else { 0 },
      );
      ffw_thumbnailer_set_film_overlay(self.ptr, if self.film_overlay { 1 } else { 0 });
    }

    Thumbnailer {
      ptr: self.ptr,
      default_size: self.size,
      default_time: self.time,
    }
  }
}
impl Default for ThumbnailerBuilder {
  fn default() -> Self {
    Self {
      ptr: null_mut(),
      maintain_aspect_ratio: true,
      film_overlay: false,

      // 128 and 10 are the defaults for their respective settings in
      // libffmpegthumbnailer, so I'm using them here
      size: 128,
      time: 10,
    }
  }
}

pub struct Thumbnailer {
  ptr: *mut c_void,
  default_size: u32,
  default_time: u8,
}
impl Thumbnailer {
  /// Initialize a [ThumbnailerBuilder]
  pub fn builder() -> ThumbnailerBuilder {
    let ptr = unsafe { ffw_thumbnailer_new() };
    if ptr.is_null() {
      panic!("failed to allocate thumbnailer");
    }
    ThumbnailerBuilder {
      ptr,
      ..Default::default()
    }
  }
  /// Generate thumbnail of **`input`**
  // I decided to **NOT** have a dedicated struct for image data, since I'm not
  // doing much with it. The allocation and destruction is-- of course-- handled
  // manually, so could potentially be unsafe.
  pub fn generate(
    &mut self,
    input: &str,
    time: Option<u8>,
    size: Option<u32>,
  ) -> Result<Vec<u8>, ()> {
    let input = String::from(input);
    unsafe {
      let buf = ffw_thumbnailer_img_data_new();

      ffw_thumbnailer_set_time(self.ptr, time.unwrap_or(self.default_time) as i32);
      ffw_thumbnailer_set_size(self.ptr, size.unwrap_or(self.default_size) as i32);

      let ret =
        ffw_thumbnailer_generate(self.ptr, buf, input.as_bytes().as_ptr() as *const _);
      if ret != 0 {
        return Err(());
      }

      let (data, size) = (
        ffw_thumbnailer_img_data_get_data(buf) as *const u8,
        ffw_thumbnailer_img_data_get_size(buf) as usize,
      );

      let th = Vec::from(slice::from_raw_parts(data, size));
      ffw_thumbnailer_img_data_destroy(buf);

      Ok(th)
    }
  }
}
impl Default for Thumbnailer {
  fn default() -> Self { Self::builder().finalize() }
}
impl Drop for Thumbnailer {
  fn drop(&mut self) {
    unsafe {
      ffw_thumbnailer_destroy(self.ptr);
    }
  }
}
