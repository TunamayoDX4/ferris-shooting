use super::*;

impl InstanceGen<ImgObjInstance> for Ferris {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance {
        position: self.position.into(),
        size: [64., 64.],
        rotation: self.rotation,
        tex_coord: [0., 0.],
        tex_size: [64., 64.],
        tex_rev: [false, false],
    }}
}
impl physic_body::PhysicBody for Ferris {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        [64., 64.].into()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}