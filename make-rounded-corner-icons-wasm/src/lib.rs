use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn add_rounded_corners(image_data: &[u8], width: u32, height: u32, radius: u32) -> Vec<u8> {
  console_log!("Received data length: {}", image_data.len());
  console_log!("Image dimensions: {}x{}", width, height);
  // console_log!("Radius: {}", radius);

  let image = match image::load_from_memory_with_format(image_data, image::ImageFormat::Png) {
    Ok(img) => img,
    Err(err) => {
      console_log!("Failed to load image: {:?}", err);
      return vec![];
    }
  };
  // console_log!("Image loaded: {:?}", image.dimensions());

  let rounded_image = process_image(&image, width, height, radius);
  console_log!("Image processed: {:?}", rounded_image.dimensions());
  // console_log!("Image processed len: {}", rounded_image.len());

  let mut rounded_image_data = Vec::new();
  if let Err(err) = rounded_image.write_to(
    &mut Cursor::new(&mut rounded_image_data),
    image::ImageFormat::Png,
  ) {
    console_log!("Failed to write image: {:?}", err);
    return vec![];
  }
  console_log!("Image written");

  rounded_image_data
}

fn process_image(image: &DynamicImage, width: u32, height: u32, radius: u32) -> RgbaImage {
  let mut rounded_image = RgbaImage::new(width, height);

  // Copy the original image to the new image
  for y in 0..height {
    for x in 0..width {
      let pixel = image.get_pixel(x, y);
      rounded_image.put_pixel(x, y, pixel);
    }
  }

  // Create a mask with rounded corners
  let mut mask = RgbaImage::new(width, height);

  // Draw rectangles
  draw_filled_rect_mut(
    &mut mask,
    Rect::at(radius as i32, 0).of_size(width - 2 * radius, height),
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_rect_mut(
    &mut mask,
    Rect::at(0, radius as i32).of_size(width, height - 2 * radius),
    Rgba([0, 0, 0, 255]),
  );

  // Draw corner circles
  draw_filled_circle_mut(
    &mut mask,
    (radius as i32, radius as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    ((width - radius) as i32, radius as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    (radius as i32, (height - radius) as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );
  draw_filled_circle_mut(
    &mut mask,
    ((width - radius) as i32, (height - radius) as i32),
    radius as i32,
    Rgba([0, 0, 0, 255]),
  );

  // Apply the mask to the rounded image
  for y in 0..height {
    for x in 0..width {
      let pixel = rounded_image.get_pixel_mut(x, y);
      let mask_pixel = mask.get_pixel(x, y);
      pixel.0[3] = mask_pixel.0[3];
    }
  }

  rounded_image
}
