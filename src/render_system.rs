use crate::{Scene, CameraNode, Ray};
use std::rc::{Rc};
use std::cell::RefCell;
use image::{DynamicImage, GenericImage, Pixel, Rgba, ImageFormat};
use std::fs::{OpenOptions};

pub struct RenderSystem {
    pub output_path: String,
}

impl RenderSystem {
    pub fn render(self, scene: &Scene, camera: Rc<RefCell<CameraNode>>) {
        let width = (*camera).borrow_mut().image_width as u32;
        let height = (*camera).borrow_mut().image_height as u32;
        let mut image = DynamicImage::new_rgb8(width, height);

        println!("Render Loop");
        for x in 0..width {
            for y in 0..height {
                let ray = Ray::create_prime(x, y, width, height);
                let black = Rgba::from_channels(0, 0, 0, 0);

                //****************** IMPORTANT ********************************
                //Only works with the current implemented scene in main.rs*****
                //println!("Pic: {} {}", x, y);
                let sphere = match (*scene.root).borrow_mut().get_child(0) {
                    None => {
                        println!("Return az Pixel: {} {}", width, height);
                        return
                    },
                    Some(x) => x,
                };
                
                if (*sphere).borrow_mut().intersect(&ray) {
                    image.put_pixel(x, y, Rgba::from_channels(0, 255, 0, 0));
                }
                else {
                    image.put_pixel(x, y, black);
                }
            }
        }

        println!("Render Pic: {} {}!", width, height);
        let mut image_file = OpenOptions::new().write(true).truncate(true).create(true).open(self.output_path).unwrap();
        image.save(&mut image_file, ImageFormat::PNG).unwrap();
    }


}