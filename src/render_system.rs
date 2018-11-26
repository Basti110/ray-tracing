use crate::{Scene, CameraNode, Node, Ray, Color};
use std::rc::{Rc};
use std::cell::RefCell;
use image::{DynamicImage, GenericImage, Pixel, Rgba, ImageFormat};
use std::fs::{OpenOptions};
use std::f64;

pub struct RenderSystem {
    pub output_path: String,
}

impl RenderSystem {
    pub fn render(self, scene: &Scene, camera: Rc<RefCell<CameraNode>>) {
        let width = value!(camera).image_width as u32;
        let height = value!(camera).image_height as u32;
        let mut image = DynamicImage::new_rgb8(width, height);

        println!("Render Loop");
        for x in 0..width {
            for y in 0..height {
                let ray = Ray::create_prime(x, y, width, height);
                let back = Rgba::from_channels(135, 206, 255, 0);

                //****************** IMPORTANT ********************************
                //Only works with the current implemented scene in main.rs*****
                //println!("Pic: {} {}", x, y);
                
                // if value!(sphere).intersect(&ray) {
                //     image.put_pixel(x, y, Rgba::from_channels(0, 255, 0, 0));
                // }
                let root = Rc::clone(&scene.root);
                match RenderSystem::get_intersection_obj(&ray, root) {
                    None => image.put_pixel(x, y, back),
                    Some(a) => {
                        let c = value!(a.obj).get_color();
                        image.put_pixel(x, y, Rgba::from_channels(c.red as u8, c.green as u8, c.blue as u8, 0))
                    },
                };
            }
        }

        println!("Render Pic: {} {}!", width, height);
        let mut image_file = OpenOptions::new().write(true).truncate(true).create(true).open(self.output_path).unwrap();
        image.save(&mut image_file, ImageFormat::PNG).unwrap();
    }

    fn get_intersection_obj(ray: &Ray, node: Rc<RefCell<Node>>) -> Option<IntersectionObject> {
        let mut cur_obj: Option<IntersectionObject> = None;
        let mut cur_distance: f64 = f64::MAX;
        let size = value!(node).get_size();
        for i in 0..size {
            //Unsafe
            let child = match value!(node).get_child(i) {
                None => continue,
                Some(x) => x,
            };

            match RenderSystem::get_intersection_obj(&ray, child) {
                None => (),
                Some(x) => {
                    if cur_obj.is_some() {
                        if x.distance < cur_distance {
                            cur_distance = x.distance;
                            cur_obj = Some(x);
                        }
                    }
                    else {
                        cur_obj = Some(x);
                    }
                }
            };
        }

        let distance = match value!(node).intersect(&ray) {
            None => cur_distance,
            Some(x) => x
        };

        if distance < cur_distance {
            cur_obj = Some(IntersectionObject {
                distance: distance,
                obj: Rc::clone(&node),
            });
        }
        return cur_obj;
    }
}

struct IntersectionObject {
    pub distance: f64,
    pub obj: Rc<RefCell<Node>>,
}
