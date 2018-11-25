use crate::{Node, Color, Ray};
use std::rc::{Weak, Rc};
use cgmath::{InnerSpace, Vector3, Matrix4};
use std::cell::RefCell;

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
        let l = origin - self.frame_transform.w.truncate();
        let adj2 = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj2 * adj2);

        d2 < (self.radius * self.radius)
    }
}