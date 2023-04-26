use super::*;

pub struct Explode {
    pub position: nalgebra::Point2<f32>, 
    pub velocity: nalgebra::Vector2<f32>, 
    pub power: f32, 
    pub fragment_count_ratio: u32, 
    pub fragment_size: f32, 
    pub fragment_lifetime: f32, 
}
impl Explode {
    pub fn explode(self, gears: &mut simple2d::entity_holder::EntityArray<
        super::Gear
    >) {
        let exp = GearSpawner {
            position: self.position,
            velocity0: Some(self.velocity), 
            rotation: 0.,
            speed_ratio: self.power,
            diffusion_ratio: 1.,
            gear_type: types::GearType::Fragment { 
                size: self.fragment_size, 
                lifetime: self.fragment_lifetime 
            },
            spawn_count_ratio: self.fragment_count_ratio,
        };
        (0..exp.gear_type.spawn_count(exp.spawn_count_ratio))
            .for_each(|_| {
                { gears.push(super::Gear::build(&exp)); }
            })
    }
}