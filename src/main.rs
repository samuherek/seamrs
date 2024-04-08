use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    let img = image::open("images/Broadway_tower_edit.jpg").expect("Failed to open image");
    let (width, height) = img.dimensions();
    let mut output: RgbaImage = ImageBuffer::new(width - 100, height);
    for y in 0..height {
        let mut splice_x = 0;
        for x in 0..width {
            if x < 100 {
                continue;
            }
            let pxl = img.get_pixel(x, y);
            output.put_pixel(splice_x, y, pxl);
            splice_x += 1;
        }
    }
    output.save("output.png").expect("Failed to save image");
}
