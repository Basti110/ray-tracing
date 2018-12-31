use crate::{Node, Ray, Color};
use std::rc::{Weak, Rc};
use cgmath::{InnerSpace, Matrix4, Matrix3, Matrix, Vector3, SquareMatrix, EuclideanSpace};
use std::cell::RefCell;

pub struct Plane {
    childs: Vec<Rc<RefCell<Node>>>,
    parent: Weak<Rc<RefCell<Node>>>,
    size: usize,
    pub name: String,
    pub frame_transform: Matrix4<f64>,
    pub world_transform: Matrix4<f64>,
    //pub origin: Point3<f64>,
    //pub normal: Vector3<f64>,
    pub color: Color,
}

impl Plane {
    pub fn new(name: String, transform: Matrix4<f64>, color: Color) -> Plane {
        Plane {
            childs: vec![],
            parent: Weak::new(),
            size: 0,
            name: name,
            frame_transform: transform,
            world_transform: transform,
            //origin: origin,
            //normal: normal,
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
        value!(node).set_world_transform(&self.world_transform);
        self.childs.push(Rc::clone(&node));
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector3<f64>)> {
        let view_transpose = Matrix3::from_cols(
            self.world_transform.x.clone().truncate(),
            self.world_transform.y.clone().truncate(),
            self.world_transform.z.clone().truncate(),
        );

        let normal =  view_transpose.transpose() * Vector3::new(0.0, -1.0, 0.0);
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {    
            let origin = self.world_transform.w.truncate();
            let v = (ray.origin - origin) * -1.0;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                let hit_point = ray.origin + (ray.direction * distance);
                let l = hit_point.to_vec() - origin;
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

    fn set_world_transform(&mut self, transform: &Matrix4<f64>) -> () {

        println!("------- transform -----------");
        println!("[{}, {}, {}, {}]", self.frame_transform.x.x, self.frame_transform.y.x, self.frame_transform.z.x, self.frame_transform.w.x);
        println!("[{}, {}, {}, {}]", self.frame_transform.x.y, self.frame_transform.y.y, self.frame_transform.z.y, self.frame_transform.w.y);
        println!("[{}, {}, {}, {}]", self.frame_transform.x.z, self.frame_transform.y.z, self.frame_transform.z.z, self.frame_transform.w.z);
        println!("[{}, {}, {}, {}]", self.frame_transform.x.w, self.frame_transform.y.w, self.frame_transform.z.w, self.frame_transform.w.w);

        self.world_transform = transform * self.frame_transform;
        println!("------- Model -----------");
        println!("[{}, {}, {}, {}]", self.world_transform.x.x, self.world_transform.y.x, self.world_transform.z.x, self.world_transform.w.x);
        println!("[{}, {}, {}, {}]", self.world_transform.x.y, self.world_transform.y.y, self.world_transform.z.y, self.world_transform.w.y);
        println!("[{}, {}, {}, {}]", self.world_transform.x.z, self.world_transform.y.z, self.world_transform.z.z, self.world_transform.w.z);
        println!("[{}, {}, {}, {}]", self.world_transform.x.w, self.world_transform.y.w, self.world_transform.z.w, self.world_transform.w.w);

        let view_transpose = Matrix3::from_cols(
            self.world_transform.x.clone().truncate(),
            self.world_transform.y.clone().truncate(),
            self.world_transform.z.clone().truncate(),
        ).transpose().invert().unwrap();
        println!("-------view-----------");
        println!("[{}, {}, {}]", view_transpose.x.x, view_transpose.y.x, view_transpose.z.x);
        println!("[{}, {}, {}]", view_transpose.x.y, view_transpose.y.y, view_transpose.z.y);
        println!("[{}, {}, {}]", view_transpose.x.z, view_transpose.y.z, view_transpose.z.z);

        let normal =  view_transpose * Vector3::new(0.0, -1.0, 0.0);

        println!("-------normal-----------");
        println!("[{}, {}, {}]", normal.x, normal.y, normal.z);


    }  
}