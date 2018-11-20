extern crate image;
extern crate cgmath;
use cgmath::{Point3, Vector3, Matrix4};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
}

pub struct Scene {
    root: Rc<RefCell<Node>>,
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
            renderer: Rc::new(RefCell::new(RenderSystem {})),
        }
    }

    pub fn add_root(&mut self, node: Rc<RefCell<Node>>) {
        self.root = Rc::clone(&node);
    }

    pub fn add_camera(&mut self, camera: Rc<RefCell<CameraNode>>) {
        self.mainCamera = Rc::clone(&camera);
    }
}

pub struct RenderSystem {

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
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Node {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>>;
    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>>;
    fn add_child(&mut self, node: Rc<RefCell<Node>>);
    fn get_size(&self) -> usize;
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
            }
        )*
    }
}

impl_T!(for Node3D, CameraNode, SphereNode);