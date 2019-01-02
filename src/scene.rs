extern crate image;
extern crate cgmath;

use crate::{CameraNode, Node, RenderSystem, DirectionalLight, SphericalLight, Light};
use std::rc::{Rc};
use std::cell::RefCell;

//const viewing_direction: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);     // Viewing direction (world coordinates)
//const start_point: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);          // Center of the Image Plane (world coordinates)
//const image_width: usize = 1080;
//const image_height: usize = 720; // Size of the image plane (world coordinates)

pub struct Scene {
    pub root: Rc<RefCell<Node>>,
    pub name: String,
    //pub mainCamera: Rc<RefCell<CameraNode>>,
    pub lights: Rc<RefCell<Light>>,
    pub renderer: Rc<RefCell<RenderSystem>>,
}

impl Scene {
    pub fn new(name: String, root: Rc<RefCell<Node>>, camera: Rc<RefCell<CameraNode>>) -> Scene {
        Scene {
            root: root,
            name: name,
            //mainCamera: camera,
            lights: Rc::new(RefCell::new(Light::Directional(DirectionalLight::off_light()))),
            renderer: Rc::new(RefCell::new(RenderSystem {output_path: "output.png".to_string()})),
        }
    }

    pub fn add_root(&mut self, node: Rc<RefCell<Node>>) {
        self.root = Rc::clone(&node);
    }

    // pub fn add_camera(&mut self, camera: Rc<RefCell<CameraNode>>) {
    //     self.mainCamera = Rc::clone(&camera);
    // }

    pub fn get_renderer(&self) -> Rc<RefCell<RenderSystem>> {
        return Rc::clone(&self.renderer);
    }

}

// macro_rules! impl_T {
//     (for $($t:ty),+) => {
//         $(
//             impl Node for $t {
//                 fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
//                     let strong = &self.parent.upgrade();
//                     let strong = match strong {
//                         Some(x) => x,
//                         None => return None,
//                     };
//                     return Some(Rc::clone(&(*strong))); //Some(Rc::clone(&(*(&self.parent))));
//                 }

//                 fn get_child(&self, index: usize) -> Option<Rc<RefCell<Node>>> {
//                     if index >= self.size {
//                         return None;
//                     }
//                     return Some(Rc::clone(&(self.childs[index])));
//                 }

//                 fn add_child(&mut self, node: Rc<RefCell<Node>>) {
//                     self.size += 1;
//                     self.childs.push(Rc::clone(&node));
//                 }

//                 fn get_size(&self) -> usize {
//                     return self.size;
//                 }

//                 fn intersect(&self, ray: &Ray) -> bool {
//                     return false;
//                 }
//             }
//         )*
//     }
// }