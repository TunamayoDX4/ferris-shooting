use tm_wg_wrapper::prelude::*;
use simple2d::{
    types::InstanceGen, 
    img_obj::ImgObjInstance, 
};

pub mod types;
pub mod explode;

pub struct GearSpawner {
    pub position: nalgebra::Point2<f32>, 
    pub velocity0: Option<nalgebra::Vector2<f32>>, 
    pub rotation: f32, 
    pub speed_ratio: f32, 
    pub diffusion_ratio: f32, 
    pub gear_type: types::GearType, 
    pub spawn_count_ratio: u32, 
}

pub struct Gear {
    position: nalgebra::Point2<f32>, 
    velocity: nalgebra::Vector2<f32>, 
    rotation: f32, 
    speed: f32, 
    render_rot: f32, 
    render_rot_speed: f32, 
    gear_type: types::GearType, 
}
impl Gear {
    pub fn build(spawner: &GearSpawner) -> Self { 
        let speed = spawner.gear_type.default_speed(
            spawner.speed_ratio, 
            spawner.diffusion_ratio
        );
        let rotation = spawner.rotation + spawner.gear_type.default_diffusion(
            spawner.diffusion_ratio
        );
        let (rotation, speed) = if let Some(velocity0) = spawner.velocity0 {
            let velocity0 = velocity0 + nalgebra::Vector2::new(
                speed * rotation.cos(), 
                speed * rotation.sin(), 
            );
            let rotation = f32::atan2(
                velocity0.y, 
                velocity0.x
            );
            let speed = f32::sqrt(velocity0.x.powi(2) + velocity0.y.powi(2));
            (rotation, speed)
        } else {
            (rotation, speed)
        };
        Self {
            position: spawner.position,
            velocity: if let Some(vel0) = spawner.velocity0 {
                vel0
            } else {
                [0., 0.].into()
            },
            rotation,
            speed, 
            render_rot: spawner.gear_type.render_rot(spawner.rotation),
            render_rot_speed: spawner.gear_type.render_rot_speed(), 
            gear_type: spawner.gear_type.clone(),
        }
    }
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
    ) {
        let prev_position = self.position;
        self.position += nalgebra::Vector2::new(
            self.speed * self.rotation.cos(), 
            self.speed * self.rotation.sin(), 
        ) * cycle.dur;
        self.velocity = (self.position - prev_position) * cycle.cps;
        self.render_rot += self.render_rot_speed * cycle.dur;
        self.gear_type.update(cycle);
    }
    pub fn alive_check(
        &self, 
        area: &simple2d::types::VisibleField, 
    ) -> bool {
        area.in_visible(self.position, [
            self.gear_type.size(), self.gear_type.size()
        ]) && self.gear_type.alive()
    }
    pub fn terminate(
        &self, 
    ) -> Option<types::GearTerminateMode> { 
        types::GearType::terminate(self)
    }
    pub fn collision_check(
        &self, 
        target_pos: nalgebra::Point2<f32>, 
        target_size: nalgebra::Vector2<f32>, 
    ) -> bool {
        let distance = (target_pos - self.position).abs();
        let size_sum = target_size * 0.5 + nalgebra::Vector2::new(
            self.gear_type.size() * 0.5, 
            self.gear_type.size() * 0.5, 
        ).abs();
        distance.x <= size_sum.x && distance.y <= size_sum.y
    }
}
impl InstanceGen<ImgObjInstance> for Gear {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance {
        position: self.position.into(),
        size: [self.gear_type.size(), self.gear_type.size()],
        rotation: self.render_rot,
        tex_coord: [0., 0.],
        tex_size: [32., 32.],
        tex_rev: [false, false],
    }}
}

/// ギア配列
pub struct Gears {
    pub instances: simple2d::entity_holder::EntityArray<Gear>, 
    pub terminate_mode: Vec<Option<types::GearTerminateMode>>, 
}
impl Gears {
    pub fn spawn(
        &mut self, 
        spawner: &GearSpawner, 
    ) { (0..spawner.gear_type.spawn_count(spawner.spawn_count_ratio)).for_each(
        |_| { self.instances.push(Gear::build(spawner)); }
    )}
    
    pub fn terminate_gears(
        &mut self
    ) {
        self.terminate_mode.iter_mut()
            .filter_map(|t| t.take())
            .for_each(|t| t.execute(&mut self.instances));
        self.terminate_mode.clear();
    }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        area: &simple2d::types::VisibleField, 
        enemies: &mut super::enemy::Enemies, 
    ) {
        self.instances.retain(|_idx, gear| {
            gear.update(cycle);
            let (enemy_idx, alive) = if let Some(
                (index, _enemy)
            ) = enemies.0.iter_mut()
                .filter(|(_, enemy)| {
                    gear.collision_check(
                        enemy.position, 
                        [
                        enemy.enemy_type.size(), 
                        enemy.enemy_type.size(),
                        ].into()
                    )
                })
                .next()
            {
                if let Some(term) = gear.terminate() {
                    self.terminate_mode.push(Some(term))
                }
                (Some(index), false)
            } else {
                (None, gear.alive_check(area))
            };
            if let Some(enemy_idx) = enemy_idx {
                enemies.0.remove(enemy_idx)
            }
            alive
        });
    }
}