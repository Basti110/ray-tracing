use cgmath::{InnerSpace, Vector3, Point3};
use image::{Pixel, Rgba};

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.red as u8,
                        self.green as u8,
                        self.blue as u8,
                        255)
    }
}

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
     pub fn create_prime(x: u32, y: u32, width: u32, height: u32) -> Ray {

        let sensor_x = ((x as f64 + 0.5) / width as f64) * 2.0 - 1.0;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / height as f64) * 2.0;

        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3 {
                    x: sensor_x,
                    y: sensor_y,
                    z: -1.0,
                }.normalize(),
        }
    }
}