use rand::Rng;

use super::*;

pub mod gear_type;
pub mod body;
pub mod instance;

pub struct Gear {
    position: nalgebra::Point2<f32>, 
    velocity: nalgebra::Vector2<f32>, 
    vel: f32, 
    rotation: f32, 
    render_rotation: f32, 
    render_rotation_speed: f32, 
    gear_type: gear_type::GearType, 
}
impl Gear {

    /// 移動処理
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

    /// 生存チェック処理
    fn alive(
        &self, 
        varea: &VisibleField, 
    ) -> bool {
        varea.in_visible(self.position, self.gear_type.size())
    }

    /// 描画部の更新
    fn render_update(
        &mut self, 
        cycle: &CycleMeasure, 
    ) {
        self.render_rotation += self.render_rotation_speed * cycle.dur;
    }

    /// 敵への衝突処理
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

    /// 更新処理
    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        enemies: &mut enemy::instance::EnemyInstance, 
    ) -> bool {
        self.moving(cycle);
        self.render_update(cycle);

        self.alive(varea) && !self.coll_enemy(enemies)
    }

}