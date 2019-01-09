use cgmath::{InnerSpace, Vector3, Point3};
use image::{Pixel, Rgba};
use std::ops::{Mul};

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

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: (r as f32)  / 255.0,
            green: (g as f32) / 255.0,
            blue: (b as f32) / 255.0,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((self.red * 255.0) as u8,
                        (self.green * 255.0) as u8,
                        (self.blue * 255.0) as u8,
                        255)
    }

    pub fn copy(&self) -> Color {
        return Color::new(self.red, self.green, self.blue);
    }

    pub fn red(&self) -> u8 {
        return (self.red * 255.0) as u8;
    }
    
    pub fn green(&self) -> u8 {
        return (self.green * 255.0) as u8;
    }
    
    pub fn blue(&self) -> u8 {
        return (self.blue * 255.0) as u8;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, width: u32, height: u32) -> Ray {
    let fov: f64 = 45.0;
    let fov_adjustment = (3.1415926 * 0.5 * fov / 180.).tan(); //(fov.to_radians() / 2.0).tan();
    let aspect_ratio = (width as f64) / (height as f64);
    let sensor_x = ((((x as f64 + 0.5) / width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
    let sensor_y = (1.0 - ((y as f64 + 0.5) / height as f64) * 2.0) * fov_adjustment;
    
    //let sensor_x = ((x as f64 + 0.5) / width as f64) * 2.0 - 1.0;
    //let sensor_y = 1.0 - ((y as f64 + 0.5) / height as f64) * 2.0;

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