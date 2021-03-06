use crate::{Node, Ray, Color};
use std::rc::{Weak, Rc};
use cgmath::{Matrix4, Vector3, Point3, EuclideanSpace, InnerSpace, MetricSpace};
use std::cell::RefCell;

pub struct DirectionalLight  {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
    pub direction: Vector3<f64>,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => Color::new(d.color.red, d.color.green, d.color.blue),
            Light::Spherical(ref s) => Color::new(s.color.red, s.color.green, s.color.blue),
        }
    }

    pub fn direction_from(&self, hit_point: &Point3<f64>) -> Vector3<f64> {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (Point3::from_vec(s.world_transform.w.truncate()) - *hit_point).normalize(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        match *self {
            Light::Directional(ref _d) => false,
            Light::Spherical(ref s) => s.intersect(ray),
        }
    }

    pub fn intensity(&self, hit_point: &Point3<f64>) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = Point3::from_vec(s.world_transform.w.truncate()).distance(*hit_point) as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point3<f64>) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => Point3::from_vec(s.world_transform.w.truncate()).distance(*hit_point),
        }
    }
}

impl DirectionalLight {
    pub fn new(name: String, transform: Matrix4<f64>, direction: Vector3<f64>, color: Color, intensity: f32) -> DirectionalLight {
        DirectionalLight {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            world_transform: transform,
            direction: direction,
            color: color,
            intensity: intensity,
        }
    }

    pub fn off_light() -> DirectionalLight {
        DirectionalLight {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: "empty".to_string(),
            frame_transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0)),
            world_transform: Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0)),
            direction: Vector3::new(0.0, 0.0, 1.0),
            color: Color::new_rgb(0, 0, 0),
            intensity: 0.0,
        }
    }
}

impl Node for DirectionalLight {
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

    fn intersect(&self, _ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        return None;
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

impl SphericalLight {
    pub fn new(name: String, transform: Matrix4<f64>, color: Color, intensity: f32) -> SphericalLight {
        SphericalLight {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            world_transform: transform,
            color: color,
            intensity: intensity,
        }
    }

    fn intersect(&self, ray: &Ray) -> bool {
        let origin = Vector3::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let l = self.world_transform.w.truncate() - origin;

        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = 0.25;
        if d2 > radius2 {
            return false;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
              return false;
        }

        return true;
    }
}

impl Node for SphericalLight {
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

    fn intersect(&self, _ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        return None;
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

impl Node for Light {
    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        match *self {
            Light::Directional(ref d) => d.get_parent(),
            Light::Spherical(ref s) => s.get_parent(),
        }
    }

    fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
        match *self {
            Light::Directional(ref d) => d.get_child(index),
            Light::Spherical(ref s) => s.get_child(index),
        }
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) {
        match *self {
            Light::Directional(ref mut d) => d.add_child(node),
            Light::Spherical(ref mut s) => s.add_child(node),
        }
    }

    fn get_size(&self) -> usize {
        match *self {
            Light::Directional(ref d) => d.get_size(),
            Light::Spherical(ref s) => s.get_size(),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        match *self {
            Light::Directional(ref d) => None,
            Light::Spherical(ref s) => None,
        }
    }

    fn get_color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.get_color(),
            Light::Spherical(ref s) => s.get_color(),
        }
    }

    fn get_world_transform(&self) -> Matrix4<f64> {
        match *self {
            Light::Directional(ref d) => d.get_world_transform(),
            Light::Spherical(ref s) => s.get_world_transform(),
        }
    }

    fn set_world_transform(&mut self, transform: &Matrix4<f64>) -> () {
        match *self {
            Light::Directional(ref mut d) => d.set_world_transform(transform),
            Light::Spherical(ref mut s) => s.set_world_transform(transform),
        }
    }
}