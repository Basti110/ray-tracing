use crate::{Scene, CameraNode, Node, Ray, Color, DirectionalLight};
use std::rc::{Rc};
use std::cell::RefCell;
use image::{DynamicImage, GenericImage, Pixel, Rgba, ImageFormat};
use std::fs::{OpenOptions};
use std::f64;
use cgmath::{InnerSpace, Vector3};
use std::time::{Duration, Instant};

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
                //let now = Instant::now();
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
                        let c = RenderSystem::get_color(&scene, &ray, &a);
                        image.put_pixel(x, y, Rgba::from_channels(c.red(), c.green(), c.blue(), 0))
                    },
                };
                //let dur = now.elapsed();
                //println!("Find Time: {}.{}.{} sek.", dur.as_secs(), dur.subsec_millis(), dur.subsec_micros());
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
                    //println!("I=0");
                    if cur_obj.is_some() {
                        if cur_distance != f64::MAX {
                        }
                        if x.distance < cur_distance {
                            cur_distance = x.distance;
                            cur_obj = Some(x);
                        }
                    }
                    else {
                        cur_distance = x.distance;
                        cur_obj = Some(x);
                    }
                }
            };
        }

        let intersect = match value!(node).intersect(&ray) {
            None => return cur_obj,
            Some(x) => (x.0, x.1)
        };

        if intersect.0 < cur_distance {
            cur_obj = Some(IntersectionObject {
                distance: intersect.0,
                obj: Rc::clone(&node),
                normal: intersect.1,
            });
        }
        return cur_obj;
    }

    fn get_color(scene: &Scene, ray: &Ray, intersection: &IntersectionObject) -> Color {
        let hit_point = ray.origin + (ray.direction * (intersection.distance - 0.01));
        //let surface_normal = intersection.element.surface_normal(&hit_point);
        let direction_to_light = value!(scene.lights).direction_from(&hit_point).normalize();
        let light_power = (intersection.normal.dot(direction_to_light) as f32);
        //println!("{}", light_power);
        //let light_power = (light_power).max(0.0);
        //println!("{}", light_power);
        let light_power = (light_power).max(0.0) * value!(scene.lights).intensity(&hit_point);
        //
        
        let light_reflected = 1.0; /// std::f32::consts::PI;
        //println!("{}", light_power);

        let shadow_ray = Ray {
            origin: hit_point,
            direction: -direction_to_light,
        };

        let root = Rc::clone(&scene.root);
        let shadow_intersection = RenderSystem::get_intersection_obj(&shadow_ray, root);
        let in_light = shadow_intersection.is_none() ||
                       shadow_intersection.unwrap().distance > value!(scene.lights).distance(&hit_point);
        
        //let light_power = if in_light { light_power } else {  0.0 };
        
        //let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        //let color = value!(intersection.obj).get_color().copy() * value!(scene.lights).color() * light_power * light_reflected;
        let color = value!(intersection.obj).get_color().copy();
        color.clamp()
    }
}

struct IntersectionObject {
    pub distance: f64,
    pub obj: Rc<RefCell<Node>>,
    pub normal: Vector3<f64>,
}
