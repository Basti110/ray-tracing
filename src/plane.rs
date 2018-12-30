use crate::{Node, Ray, Color};
use std::rc::{Weak, Rc};
use cgmath::{InnerSpace, Matrix4, Vector3, Point3, EuclideanSpace};
use std::cell::RefCell;

pub struct Plane {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
    pub origin: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: Color,
}

impl Plane {
    pub fn new(name: String, transform: Matrix4<f64>, origin: Point3<f64>, normal: Vector3<f64>, color: Color) -> Plane {
        Plane {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            world_transform: transform,
            origin: origin,
            normal: normal,
            color: color,
        }
    }
}

impl Node for Plane {
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

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {    
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                let hit_point = ray.origin + (ray.direction * distance);
                let l = hit_point.to_vec() - self.origin.to_vec();
                //let l = l.magnitude();
                if l.x.abs() < 6.0 && l.y.abs() < 6.0 && l.z.abs() < 6.0 {
                    return Some((distance, normal));
                }
            }
        }
        None
    }

    fn get_color(&self) -> Color {
        return self.color.copy();
    }

    fn get_world_transform(&self) -> Matrix4<f64> {
        return self.world_transform;
    }

    fn set_world_transform(&mut self, transform: Matrix4<f64>) -> () {
        self.world_transform = transform;
    }    
}