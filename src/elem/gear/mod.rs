use rand::Rng;

use super::*;

pub mod gear_type;
pub mod body;
pub mod instance;

pub struct GearPhysicBody {
    position: nalgebra::Point2<f32>, 
    velocity: nalgebra::Vector2<f32>, 
    vel: f32, 
    rotation: f32, 
    render_rotation: f32, 
    render_rotation_speed: f32, 
}
impl GearPhysicBody {

    fn moving(
        &mut self, 
        cycle: &CycleMeasure, 
    ) {
        let prev_position = self.position;
        self.position += nalgebra::Vector2::new(
            self.vel * self.rotation.cos(), 
            self.vel * self.rotation.sin(), 
        ) * cycle.dur;
        self.velocity = self.position - prev_position;
    }

    fn alive(
        &self, 
        varea: &VisibleField, 
        gear_type: &gear_type::GearType, 
    ) -> bool {
        varea.in_visible(self.position, gear_type.size())
    }

    fn render_update(
        &mut self, 
        cycle: &CycleMeasure, 
    ) {
        self.render_rotation += self.render_rotation_speed * cycle.dur
    }
}
impl physic_body::PhysicBody for GearPhysicBody {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        [32., 32.].into()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}

pub struct Gear {
    pbody: GearPhysicBody, 
    gear_type: gear_type::GearType, 
}
impl Gear {

    /// 更新処理
    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        enemies: &mut enemy::instance::EnemyInstance, 
    ) -> bool {
        self.pbody.moving(cycle);
        self.pbody.render_update(cycle);
        self.gear_type.seek(&self.pbody, &enemies.enemies);
        self.gear_type.tracking(&mut self.pbody, cycle);

        self.pbody.alive(varea, &self.gear_type) && !self.coll_enemy(enemies)
    }

    fn coll_enemy(
        &self, 
        enemies: &mut enemy::instance::EnemyInstance, 
    ) -> bool {
        crate::RNG.with(|r| {
            let mut r = (**r).borrow_mut();
            for enemy in enemies.enemies.iter_mut()
                .map(|(_, e)| e)
                .filter(|e| e.health != 0)
            {
                if physic_body::aabb(self, enemy) {
                    let damage = (
                        self.gear_type.weight().round() * r.gen_range(0.9..1.8)
                    ) as u32;
                    enemy.health = enemy.health.checked_sub(damage).unwrap_or(0);
                    return true;
                }
            }
            false
        })
    }

}