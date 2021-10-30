use std::io::Write;

const IMAGE_WIDTH:usize = 400;
const IMAGE_HEIGHT:usize = 200;
const BUFFER_SIZE:usize = IMAGE_WIDTH * IMAGE_HEIGHT * 3;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn from_normalized(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8
        }
    }
}

fn color(x: f64, y: f64) -> Color {
    let r = x;
    let g = y;
    let b = x * y;
    Color::from_normalized(r, g, b)
}

fn main() -> std::io::Result<()> {
    let mut image_data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let i = (y * IMAGE_WIDTH + x) * 3;
            let c = color((x as f64) / (IMAGE_WIDTH as f64), (y as f64) / (IMAGE_HEIGHT as f64));
            image_data[i] = c.r;
            image_data[i+1] = c.g;
            image_data[i+2] = c.b;
        }
    }

    let mut buffer = std::fs::File::create("output.ppm")?;

    // write PPM header
    buffer.write(format!("P6 {width} {height} 255\n", width=IMAGE_WIDTH, height=IMAGE_HEIGHT).as_bytes())?;
    // and then the data!
    buffer.write(&image_data)?;
    Ok(())
}
