use super::*;

pub struct Ferris {
    position: nalgebra::Point2<f32>, 
    rotation: f32, 
    velocity: nalgebra::Vector2<f32>, 
}
impl InstanceGen<ImgObjInstance> for Ferris {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance { 
            position: self.position.into(), 
            size: [64., 64.], 
            rotation: self.rotation, 
            tex_coord: [0., 0.], 
            tex_size: [64., 64.], 
            tex_rev: [false, false], 
        })
    }
}