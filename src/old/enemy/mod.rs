use rand::Rng;
use tm_wg_wrapper::prelude::*;
use simple2d::{
    types::InstanceGen, 
    img_obj::ImgObjInstance, 
};

pub mod types;

pub struct EnemySpawner {
    pub position: nalgebra::Point2<f32>, 
    pub rotation: f32, 
    pub speed_ratio: f32, 
    pub enemy_type: types::EnemyType, 
}
impl EnemySpawner {
    pub fn spawn(
        area: &simple2d::types::VisibleField, 
        speed_ratio: f32, 
    ) -> Self { 
        let enemy_type = types::EnemyType::spawn();
        let position = {
            let area = area.visible_area();
            let x = crate::RNG.with(|r| (**r).borrow_mut().gen_range(
                area[0].x..area[1].x
            ));
            nalgebra::Point2::new(x, area[1].y + enemy_type.size() * 0.5)
        }; 
        Self {
            position, 
            rotation: 0., 
            speed_ratio,
            enemy_type, 
        } 
    }
}

pub struct Enemy {
    pub position: nalgebra::Point2<f32>, 
    velocity: nalgebra::Vector2<f32>, 
    rotation: f32, 
    speed: f32, 
    render_rot: f32, 
    render_rot_speed: f32, 
    pub enemy_type: types::EnemyType, 
}
impl Enemy {
    pub fn build(spawner: &EnemySpawner) -> Self { Self {
        position: spawner.position,
        velocity: [0., 0.].into(),
        rotation: spawner.rotation - std::f32::consts::PI / 2.,
        speed: spawner.enemy_type.default_speed(spawner.speed_ratio),
        render_rot: 0.,
        render_rot_speed: spawner.enemy_type.default_render_rot_speed(),
        enemy_type: spawner.enemy_type,
    }}
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        area: &simple2d::types::VisibleField, 
    ) -> bool {
        let prev_position = self.position;
        self.position += nalgebra::Vector2::new(
            self.speed * self.rotation.cos(), 
            self.speed * self.rotation.sin(), 
        ) * cycle.dur;
        self.velocity = self.position - prev_position;
        self.render_rot += self.render_rot_speed * cycle.dur;

        area.visible_area()[0].y < self.position.y + self.enemy_type.size() * 0.5
    }
}
impl InstanceGen<ImgObjInstance> for Enemy {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance { 
        position: self.position.into(), 
        size: [self.enemy_type.size(), self.enemy_type.size()], 
        rotation: self.render_rot, 
        tex_coord: self.enemy_type.tex_coord(), 
        tex_size: self.enemy_type.tex_size(), 
        tex_rev: [false, false], 
    }}
}

/// 敵配列
pub struct Enemies(pub simple2d::entity_holder::EntityArray<Enemy>);
impl Enemies {
    pub fn spawn(
        &mut self, 
        area: &simple2d::types::VisibleField, 
        cycle: &cycle_measure::CycleMeasure, 
        speed_ratio: f32, 
        spawn_chance: impl Into<f64>, 
        spawn_count_min: u32, 
        spawn_count_max: u32, 
    ) { 
        if let Some(range) = crate::RNG.with(|r| {
            let mut r = (**r).borrow_mut();
            let range = r.gen_range(spawn_count_min..=spawn_count_max);
            if r.gen_bool(spawn_chance.into() * cycle.dur as f64) {
                Some(range)
            } else {
                None
            }
        }) {
            (0..range)
                .map(|_| EnemySpawner::spawn(area, speed_ratio))
                .map(|es| Enemy::build(&es))
                .for_each(|enemy| {
                    self.0.push(enemy);
                })
        }
    }
}