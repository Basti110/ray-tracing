extern crate image;
extern crate cgmath;
use image::ImageFormat;
use image::{DynamicImage, GenericImage, Pixel, ImageBuffer, Rgba};
use cgmath::{Matrix, VectorSpace, InnerSpace, Point3, Vector3, Matrix4};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};

//const viewing_direction: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);     // Viewing direction (world coordinates)
//const start_point: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);          // Center of the Image Plane (world coordinates)
//const image_width: usize = 1080;
//const image_height: usize = 720; // Size of the image plane (world coordinates)

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

pub struct Scene {
    pub root: Rc<RefCell<Node>>,
    pub name: String,
    pub mainCamera: Rc<RefCell<CameraNode>>,
    pub renderer: Rc<RefCell<RenderSystem>>,
}

impl Scene {
    pub fn new(name: String, root: Rc<RefCell<Node>>, camera: Rc<RefCell<CameraNode>>) -> Scene {
        Scene {
            root: root,
            name: name,
            mainCamera: camera,
            renderer: Rc::new(RefCell::new(RenderSystem {output_path: "output.png".to_string()})),
        }
    }

    pub fn add_root(&mut self, node: Rc<RefCell<Node>>) {
        self.root = Rc::clone(&node);
    }

    pub fn add_camera(&mut self, camera: Rc<RefCell<CameraNode>>) {
        self.mainCamera = Rc::clone(&camera);
    }

    pub fn get_renderer(&self) -> Rc<RefCell<RenderSystem>> {
        return Rc::clone(&self.renderer);
    }

}

pub struct RenderSystem {
    pub output_path: String,
}

impl RenderSystem {
    pub fn render(self, scene: &Scene, camera: Rc<RefCell<CameraNode>>) {
        let width = (*camera).borrow_mut().image_width as u32;
        let height = (*camera).borrow_mut().image_height as u32;
        let mut image = DynamicImage::new_rgb8(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = Ray::create_prime(x, y, width, height);
                let black = Rgba::from_channels(0, 0, 0, 0);

                //****************** IMPORTANT ********************************
                //Only works with the current implemented scene in main.rs*****
                let sphere = match (*scene.root).borrow_mut().get_child(0) {
                    None => return,
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

        let mut image_file = OpenOptions::new().write(true).truncate(true).create(true).open(self.output_path).unwrap();
        image.save(&mut image_file, ImageFormat::PNG).unwrap();
    }


}

pub struct SphereNode {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    //pub center: Point3<f64>,
    pub radius: f64,
    pub color: Color,
}

impl SphereNode {
    pub fn new(name: String, transform: Matrix4<f64>, radius: f64, color: Color) -> SphereNode {
        SphereNode {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            radius: radius,
            color: color,
        }
    }
}

pub struct TriangleNode {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
}



pub struct CameraNode {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    //center: Point3<f64>,
    pub frame_transform: Matrix4<f64>,
    pub viewing_direction: Vector3<f64>,
    pub plane_point: Vector3<f64>,          
    pub image_width: usize,
    pub image_height: usize, 
    plane_with: f64,
    plane_height: f64,
}

impl CameraNode {
    pub fn new(name: String, transform: Matrix4<f64>, direction: Vector3<f64>, width: usize, height: usize) -> CameraNode {
        CameraNode {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            viewing_direction: direction,
            plane_point: Vector3::new(0.0, 0.0, 0.0),
            image_width: width,
            image_height: height,
            plane_with: 0.0,
            plane_height: 0.0,
        }
    }
}

pub struct Node3D {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
}

impl Node3D {
    pub fn new(name: String, transform: Matrix4<f64>) -> Node3D {
        Node3D {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
        }
    }

    pub fn empty() -> Node3D {
        Node3D {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: "".to_string(),
            frame_transform: Matrix4::from_scale(0.0),
        }
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

pub trait Node {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>>;
    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>>;
    fn add_child(&mut self, node: Rc<RefCell<Node>>);
    fn get_size(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> bool;
}

macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(
            impl Node for $t {
                fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
                    let strong = &self.parent.upgrade();
                    let strong = match strong {
                        Some(x) => x,
                        None => return None,
                    };
                    return Some(Rc::clone(&(*strong))); //Some(Rc::clone(&(*(&self.parent))));
                }

                fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
                    if index >= self.size {
                        return None;
                    }
                    return Some(Rc::clone(&(self.childs[index])));
                }

                fn add_child(&mut self, node: Rc<RefCell<Node>>) {
                    self.size += 1;
                    self.childs.push(Rc::clone(&node));
                }

                fn get_size(&self) -> usize {
                    return self.size;
                }

                fn intersect(&self, ray: &Ray) -> bool {
                    return false;
                }
            }
        )*
    }
}

impl_T!(for Node3D, CameraNode);

impl Node for SphereNode {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        let strong = &self.parent.upgrade();
        let strong = match strong {
            Some(x) => x,
            None => return None,
        };
        return Some(Rc::clone(&(*strong))); //Some(Rc::clone(&(*(&self.parent))));
    }

    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
        if index >= self.size {
            return None;
        }
        return Some(Rc::clone(&(self.childs[index])));
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) {
        self.size += 1;
        self.childs.push(Rc::clone(&node));
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn intersect(&self, ray: &Ray) -> bool {
        let origin = Vector3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let l = origin - self.frame_transform.row(3).truncate();
        let adj2 = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj2 * adj2);

        d2 < (self.radius * self.radius)
    }
}