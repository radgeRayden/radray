use std::io::Write;
use glam::{DVec3, dvec3};

const IMAGE_WIDTH:usize = 400;
const IMAGE_HEIGHT:usize = 225;
const BUFFER_SIZE:usize = IMAGE_WIDTH * IMAGE_HEIGHT * 3;
const ASPECT_RATIO:f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

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

struct Ray {
    origin: DVec3,
    direction: DVec3
}

impl Ray {
    fn at(self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}

fn hit_sphere(center: DVec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = DVec3::dot(ray.direction, ray.direction);
    let b = 2.0 * DVec3::dot(oc, ray.direction);
    let c = DVec3::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(ray: Ray) -> DVec3 {
    if hit_sphere(dvec3(0.0,0.0,-1.0), 0.5, &ray) {
        return dvec3(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    DVec3::lerp(dvec3(1.0,1.0,1.0), dvec3(0.5, 0.7, 1.0), t)
}

fn pixel_color(x: usize, y: usize) -> Color {
    let u = (x as f64) / ((IMAGE_WIDTH - 1) as f64);
    // flip v because our coordinate system is y-up
    let v = 1.0 - (y as f64) / ((IMAGE_HEIGHT - 1) as f64);

    // transform to camera space
    let origin = dvec3(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;
    let lower_left = origin - dvec3(viewport_width / 2.0, viewport_height / 2.0, focal_length);

    let direction = lower_left + dvec3(viewport_width * u, viewport_height * v, 0.0) - origin;

    let ray = Ray {origin, direction};
    let c = ray_color(ray);
    Color::from_normalized(c.x, c.y, c.z)
}

fn main() -> std::io::Result<()> {
    let mut image_data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let i = (y * IMAGE_WIDTH + x) * 3;
            let c = pixel_color(x, y);
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
