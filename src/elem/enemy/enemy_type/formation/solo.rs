use super::*;
pub struct Solo(enemy_type::EnemyType);
impl Default for Solo {
    fn default() -> Self {
        Self(match crate::RNG.with(|r|
            (**r).borrow_mut().gen_range(0..100)
        ) {
            c @ _ if 95 < c => enemy_type::EnemyType::DangPtr, 
            c @ _ if 82 < c => enemy_type::EnemyType::DataRace, 
            c @ _ if 60 < c => enemy_type::EnemyType::NullPtr, 
            _ => enemy_type::EnemyType::UBehavior, 
        })
    }
}
impl super::Formation for Solo {
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
        let mut position = position;
        position.y += self.0.size()[1] * 0.5;
        let enemy = Enemy {
            health: self.0.health(), 
            ident: ident.gen(), 
            position,
            rotation: -std::f32::consts::PI * 0.5,
            render_rotation: self.0.default_render_rot(rng), 
            render_rot_speed: self.0.render_rot_speed(rng), 
            vel: self.0.vel_zero(),
            velocity: [0., 0.].into(),
            enemy_type: self.0.clone(),
        };
        enemies.push(enemy);
        false
    }
}