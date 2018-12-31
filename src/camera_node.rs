use crate::{Node3D, Node, Ray, Color};
use std::rc::{Weak, Rc};
use cgmath::{Vector3, Matrix4};
use std::cell::RefCell;

pub struct CameraNode {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    //center: Point3<f64>,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
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
            world_transform: transform,
            viewing_direction: direction,
            plane_point: Vector3::new(0.0, 0.0, 0.0),
            image_width: width,
            image_height: height,
            plane_with: 0.0,
            plane_height: 0.0,
        }
    }
}

impl Node for CameraNode {
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
        value!(node).set_world_transform(&self.world_transform);
        self.childs.push(Rc::clone(&node));
    } 

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        return None;
    }

    fn get_color(&self) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }

    fn get_world_transform(&self) -> Matrix4<f64> {
        return self.world_transform;
    }

    fn set_world_transform(&mut self, transform: &Matrix4<f64>) -> () {
        self.world_transform = transform * self.frame_transform;
    }
}