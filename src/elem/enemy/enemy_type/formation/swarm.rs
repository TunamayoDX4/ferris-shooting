use rand_distr::StandardNormal;

use super::*;

pub struct Swarm{
    enemy_type: EnemyType, 
    count: u32, 
    scale: f32, 
    distr: rand_distr::StandardNormal, 
}
impl Default for Swarm {
    fn default() -> Self { crate::RNG.with(|r| {
        let mut r = (**r).borrow_mut();
        let enem_type = match r.gen_range(0..100) {
            c @ _ if 98 < c => enemy_type::EnemyType::DangPtr, 
            c @ _ if 92 < c => enemy_type::EnemyType::DataRace, 
            c @ _ if 60 < c => enemy_type::EnemyType::NullPtr, 
            _ => enemy_type::EnemyType::UBehavior, 
        };
        let enem_size = enem_type.size();
        let enem_size = (enem_size.x.powi(2) + enem_size.y.powi(2)).sqrt() * 0.3;
        let count = r.gen_range(3..12);
        Self { 
            enemy_type: enem_type, 
            count, 
            scale: enem_size * 2.5, 
            distr: rand_distr::StandardNormal,  
        }
    })}
}
impl Formation for Swarm {
    fn spawn(
        &self, 
        _cycle: &CycleMeasure, 
        _varea: &VisibleField, 
        rng: &mut impl Rng, 
        ident: &mut EnemyIdentGen, 
        enemies: &mut EntityArray<Enemy>, 
        position: nalgebra::Point2<f32>, 
        _cycle_dur: f32, 
    ) -> bool {
        for _ in 0..self.count {
            let x = self.scale * (*rng).sample::<f32, StandardNormal>(self.distr);
            let y = self.scale * (*rng).sample::<f32, StandardNormal>(self.distr);
            let position = position + nalgebra::Vector2::new(
                x, 
                y + (self.scale * 3.0), 
            );
            enemies.push(Enemy {
                health: self.enemy_type.health(), 
                ident: ident.gen(), 
                position,
                rotation: -std::f32::consts::PI * 0.5,
                render_rotation: self.enemy_type.default_render_rot(rng),
                render_rot_speed: self.enemy_type.render_rot_speed(rng),
                vel: self.enemy_type.vel_zero(),
                velocity: [0., 0.].into(),
                enemy_type: self.enemy_type.clone(),
            });
        }
        false
    }
}