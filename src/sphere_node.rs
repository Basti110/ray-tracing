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
    pub world_transform: Matrix4<f64>,
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
            world_transform: transform,
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
        value!(node).set_world_transform(&self.world_transform);
        self.childs.push(Rc::clone(&node));
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let origin = Vector3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let l = self.world_transform.w.truncate() - origin;

        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
              return None;
        }
        //println!("test");
        let distance = if t0 < t1 { if t0 > 0.0 { t0 } else { t1 } } else { t1 };

        let hit_point = ray.origin + (ray.direction * distance);
        let normal = hit_point - self.world_transform.w.truncate();
        let normal = -Vector3::new(normal.x, normal.y, normal.z).normalize();
        //println!("{}: {}", self.name, distance);
        Some((distance, normal))
    }

    fn get_color(&self) -> Color {
        return self.color.copy();
    }

    fn get_world_transform(&self) -> Matrix4<f64> {
        return self.world_transform;
    }

    fn set_world_transform(&mut self, transform: &Matrix4<f64>) -> () {
        self.world_transform = transform * self.frame_transform;
    }
}