use kurbo::Line;

use piet::{ImageFormat, RenderContext};
use piet_common::Device;

fn main() {
    let device = Device::new().unwrap();
    let width = 640;
    let height = 480;
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
    let mut rc = bitmap.render_context();
    rc.clear(0xffffff).unwrap();
    let brush = rc.solid_brush(0x00_00_80_FF).unwrap();
    rc.stroke(Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0, None)
        .unwrap();
    rc.finish().unwrap();
    let raw_pixels = bitmap.into_raw_pixels(ImageFormat::RgbaPremul).unwrap();
    image::save_buffer(
        "temp-image.png",
        &raw_pixels,
        width as u32,
        height as u32,
        image::ColorType::RGBA(8),
    )
    .unwrap();
}
