use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    let img = image::open("images/Broadway_tower_edit.jpg").expect("Failed to open image");
    let (width, height) = img.dimensions();

    for c in 0..100 {
        let mut temp = ImageBuffer::new(width - (c + 1), height);
        for y in 0..height {
            let mut splice_x = 0;
            for x in 0..width {
                if x < 100 {
                    continue;
                }
                let pxl = img.get_pixel(x, y);
                temp.put_pixel(splice_x, y, pxl);
                splice_x += 1;
            }
        }
        if c == 99 {
            temp.save("output.png").expect("Failed to save image");
        }
    }
}
