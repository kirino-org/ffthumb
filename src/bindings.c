/*
  ffthumb
*/

#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>

#include <libffmpegthumbnailer/imagetypes.h>
#include <libffmpegthumbnailer/videothumbnailerc.h>

void *ffw_thumbnailer_new()
{
  return video_thumbnailer_create();
}

void ffw_thumbnailer_destroy(video_thumbnailer *thumbnailer)
{
  video_thumbnailer_destroy(thumbnailer);
}

int ffw_thumbnailer_generate(video_thumbnailer *thumbnailer, image_data *img_data, const char *input)
{
  return video_thumbnailer_generate_thumbnail_to_buffer(thumbnailer, input, img_data);
}

void ffw_thumbnailer_set_maintain_aspect_ratio(video_thumbnailer *thumbnailer, int val)
{
  thumbnailer->maintain_aspect_ratio = val;
}

void ffw_thumbnailer_set_film_overlay(video_thumbnailer *thumbnailer, int val)
{
  thumbnailer->overlay_film_strip = val;
}

void ffw_thumbnailer_set_size(video_thumbnailer *thumbnailer, int val)
{
  // It seems thumbnail_size is deprecated with the version in Gentoo repos.
  //
  // Not sure about Debian/Ubuntu's support for video_thumbnailer_set_size(),
  // so leaving it for now.
  thumbnailer->thumbnail_size = val;
}

void ffw_thumbnailer_set_time(video_thumbnailer *thumbnailer, int time)
{
  thumbnailer->seek_percentage = time;
}



void *ffw_thumbnailer_img_data_new()
{
  return video_thumbnailer_create_image_data();
}

void ffw_thumbnailer_img_data_destroy(image_data *img_data)
{
  video_thumbnailer_destroy_image_data(img_data);
}

int ffw_thumbnailer_img_data_get_size(image_data *img_data)
{
  return img_data->image_data_size;
}

uint8_t *ffw_thumbnailer_img_data_get_data(image_data *img_data)
{
  return img_data->image_data_ptr;
}
