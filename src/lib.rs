use image::{imageops, ImageBuffer, RgbaImage};

fn flag_transparent(image: RgbaImage) -> RgbaImage {
    let mut image = image;
    let mut length = image.height();
    if image.height() > image.width() {
        length = image.width();
    }

    let mut sub_image = imageops::crop(&mut image, 0, 0, length, length).to_image();
    let mut transparent_image = ImageBuffer::new(length, length);
    for (x, y, pixel) in sub_image.enumerate_pixels_mut() {
        let transparency =
            pixel[3] - (255 as f64 * ((x * x + y * y) as f64).sqrt() / (length as f64)) as u8;
        transparent_image.put_pixel(
            x,
            y,
            image::Rgba([pixel[0], pixel[1], pixel[2], transparency]),
        );
    }

    transparent_image
}

pub fn overlay(avatar: RgbaImage, flag: RgbaImage) -> RgbaImage {
    let transparent_flag = flag_transparent(flag);
    let mut length = avatar.height();
    if avatar.height() > avatar.width() {
        length = avatar.width();
    }

    let transparent_flag_resized = imageops::resize(
        &transparent_flag,
        length,
        length,
        imageops::FilterType::Nearest,
    );

    let mut avatar = avatar;

    imageops::overlay(&mut avatar, &transparent_flag_resized, 0, 0);

    avatar
}
