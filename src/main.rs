use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

fn main() {
    generate_image(255);
}

fn generate_image(size: u32) {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, size, size);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut v: Vec<u8> = Vec::new();
    v.reserve((size*size) as usize);

    for i in 0..(size*size) {
        v.push(i as u8);
    }

    let data = v.as_ref();

    writer.write_image_data(data).unwrap();
}
