use libc::{c_char, c_int, c_void};

extern "C" {
  pub fn ffw_thumbnailer_new() -> *mut c_void;
  pub fn ffw_thumbnailer_destroy(thumbnailer: *mut c_void);
  pub fn ffw_thumbnailer_generate(
    thumbnailer: *mut c_void,
    img_data: *mut c_void,
    input: *const c_char,
  ) -> c_int;
  pub fn ffw_thumbnailer_set_maintain_aspect_ratio(thumbnailer: *mut c_void, val: c_int);
  pub fn ffw_thumbnailer_set_film_overlay(thumbnailer: *mut c_void, val: c_int);
  pub fn ffw_thumbnailer_set_size(thumbnailer: *mut c_void, val: c_int);
  pub fn ffw_thumbnailer_set_time(thumbnailer: *mut c_void, val: c_int);

  pub fn ffw_thumbnailer_img_data_new() -> *mut c_void;
  pub fn ffw_thumbnailer_img_data_destroy(img_data: *mut c_void);
  pub fn ffw_thumbnailer_img_data_get_size(img_data: *mut c_void) -> c_int;
  pub fn ffw_thumbnailer_img_data_get_data(img_data: *mut c_void) -> *mut c_void;
}
