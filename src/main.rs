use image::*;

const BLOOM_PASSES: u8 = 8;
const COLOR_TO_BLOOM: [[u8; 3]; 3] = [
    [133, 153, 0],
    [181, 137, 0],
    [42, 161, 152]
];

fn main() {
    let img = open("img.png").unwrap();
    let mut bloom_mask = RgbaImage::new(img.width(), img.height());
    for y in 0..img.height() {
        for x in 0..img.width() {
            if COLOR_TO_BLOOM.contains(&img.get_pixel(x, y).to_rgb().0) {
                bloom_mask.put_pixel(x, y, img.get_pixel(x, y));
            }
        }
    }

    let mut result_img = img.clone().into_rgba8();
    for i in 0..BLOOM_PASSES {
        add_to_img(&mut result_img, do_a_bloom_pass(&bloom_mask, i + 2))
    }
    result_img.save("res.png").unwrap();
}

fn add_to_img(dest: &mut RgbaImage, source: RgbaImage) {
    for (dpx, spx) in dest.pixels_mut().zip(source.pixels()) {
        dpx.0[0] = std::cmp::min(spx.0[0] as u32 + dpx.0[0] as u32, 255) as u8;
        dpx.0[1] = std::cmp::min(spx.0[1] as u32 + dpx.0[1] as u32, 255) as u8;
        dpx.0[2] = std::cmp::min(spx.0[2] as u32 + dpx.0[2] as u32, 255) as u8;
    }
}

fn do_a_bloom_pass(img: &RgbaImage, depth: u8) -> RgbaImage {
    imageops::resize(
        &imageops::resize(
            img,
            img.width() / (2_u32.pow(depth as u32)),
            img.height() / (2_u32.pow(depth as u32)),
            imageops::FilterType::Triangle,
        ),
        img.width(),
        img.height(),
        imageops::FilterType::Gaussian,
    )
}
