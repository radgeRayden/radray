use std::io::Write;

const IMAGE_WIDTH:usize = 400;
const IMAGE_HEIGHT:usize = 200;
const BUFFER_SIZE:usize = IMAGE_WIDTH * IMAGE_HEIGHT * 3;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn color(x: f32, y: f32) -> Color {
    let r = (x * 255.0) as u8;
    let g = (y * 255.0) as u8;
    let b = (x * y * 255.0) as u8;
    return Color {r, g, b};
}

fn main() {
    let mut image_data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let i = (y * IMAGE_WIDTH + x) * 3;
            let c = color((x as f32) / (IMAGE_WIDTH as f32), (y as f32) / (IMAGE_HEIGHT as f32));
            image_data[i] = c.r;
            image_data[i+1] = c.g;
            image_data[i+2] = c.b;
        }
    }

    let mut buffer = std::fs::File::create("output.ppm").unwrap();

    // write PPM header
    buffer.write(format!("P6 {width} {height} 255\n", width=IMAGE_WIDTH, height=IMAGE_HEIGHT).as_bytes());
    // and then the data!
    buffer.write(&image_data);
}
