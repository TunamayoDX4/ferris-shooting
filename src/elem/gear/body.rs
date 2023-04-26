use super::*;

impl InstanceGen<ImgObjInstance> for Gear {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance {
        position: self.position.into(),
        size: self.gear_type.size().into(),
        rotation: self.render_rotation,
        tex_coord: [0., 0.],
        tex_size: [32., 32.],
        tex_rev: [false, false],
    }}
}
impl physic_body::PhysicBody for Gear {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }
    fn size(&self) -> nalgebra::Vector2<f32> {
        self.gear_type.size()
    }
    fn rotation(&self) -> f32 {
        self.rotation
    }
    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}