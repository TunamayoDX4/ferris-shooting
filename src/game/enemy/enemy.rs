use rand::Rng;

use super::*;

pub struct EnemyArray {
    ident: EnemyIdentMaster, 
    pub enemies: EntityArray<ImgObjInstance, Enemy>, 
}
impl EnemyArray {
    pub fn new() -> Self { Self {
        ident: EnemyIdentMaster(0), 
        enemies: EntityArray::new([]), 
    }}

    pub fn rendering(&self, renderer: &mut crate::renderer::FSRenderer) {
        renderer.enemy.push_instance(&self.enemies);
    }

    pub fn spawn(
        &mut self, 
        enemy: enemy::EnemyType, 
        position: nalgebra::Point2<f32>, 
        rotation: f32, 
        vel_diffuse: bool, 
    ) -> enemy::EnemyRef {
        let ident = self.ident.issue();
        let idx = self.enemies.push(enemy.spawn(
            ident.clone(), 
            position, 
            rotation, 
            vel_diffuse, 
        ));
        enemy::EnemyRef {
            ident,
            idx,
        }
    }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        spawner: &mut super::spawn::EnemySpawnerArray, 
    ) {
        self.enemies.retain(|
            _idx, entity, 
        | entity.update(cycle, varea, spawner));
    }

    pub fn get(
        &self, 
        enemy_ref: &EnemyRef, 
    ) -> Option<&Enemy> {
        self.enemies.get(enemy_ref.idx)
            .filter(|e| e.ident == enemy_ref.ident)
    }
}

pub struct EnemyIdentMaster(u64);
impl EnemyIdentMaster {
    pub fn issue(&mut self) -> enemy::EnemyIdent {
        let r = enemy::EnemyIdent(self.0);
        self.0 = self.0.checked_add(1).unwrap_or(0);
        r
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnemyIdent(pub(super) u64);

#[derive(Clone)]
pub enum EnemyType {
    UndefBeh, 
    NullPtr, 
    DataRace, 
    DangPtr, 
}
impl EnemyType {
    pub fn tex_size(&self) -> [f32; 2] { match self {
        Self::UndefBeh => [64., 64.], 
        Self::NullPtr => [64., 64.], 
        Self::DataRace => [64., 64.], 
        Self::DangPtr => [64., 64.], 
    }}

    pub fn tex_coord(&self) -> [f32; 2] { match self {
        Self::UndefBeh => [0., 0.], 
        Self::NullPtr => [64., 0.], 
        Self::DataRace => [128., 0.], 
        Self::DangPtr => [192., 0.], 
    }}

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        Self::UndefBeh => [64., 64.].into(), 
        Self::NullPtr => [64., 64.].into(), 
        Self::DataRace => [64., 64.].into(), 
        Self::DangPtr => [64., 64.].into(), 
    }}

    pub fn vel_0(&self) -> f32 { match self {
        Self::UndefBeh => 180., 
        Self::NullPtr => 360., 
        Self::DataRace => 160., 
        Self::DangPtr => 280., 
    } }

    pub fn vel0_diffuse(
        &self, 
    ) -> Option<std::ops::Range<f32>> { match self {
        Self::UndefBeh => None, 
        Self::NullPtr => Some(-10.0..10.0), 
        Self::DataRace => Some(-40.0..40.0), 
        Self::DangPtr => None, 
    } }

    pub fn render_rot_speed_range(
        &self
    ) -> Option<std::ops::Range<f32>> { match self {
        Self::UndefBeh => None, 
        Self::NullPtr => None, 
        Self::DataRace => Some(30.0..45.0), 
        Self::DangPtr => Some(45.0..60.0), 
    }}

    pub fn default_render_rot_range(
        &self, 
    ) -> Option<std::ops::Range<f32>> { match self {
        Self::UndefBeh => None, 
        Self::NullPtr => Some(-180.0..180.0), 
        Self::DataRace => Some(-180.0..180.0), 
        Self::DangPtr => Some(-180.0..180.0), 
    } }

    pub fn health(&self) -> f32 { match self {
        Self::UndefBeh => 1., 
        Self::NullPtr => 2.5, 
        Self::DataRace => 4.5, 
        Self::DangPtr => 12., 
    } }

    pub fn health_diffuse(
        &self
    ) -> Option<std::ops::Range<f32>> { match self {
        Self::UndefBeh => Some(-0.3..0.3), 
        Self::NullPtr => Some(-0.5..0.5), 
        Self::DataRace => Some(-1.25..1.25), 
        Self::DangPtr => Some(-3.0..3.0), 
    }}

    pub fn spawn(
        self, 
        ident: EnemyIdent,
        position: nalgebra::Point2<f32>, 
        rotation: f32, 
        vel_diffuse: bool, 
    ) -> Enemy { 
        let (
            render_rot, 
            render_rot_speed, 
            vel, 
            health, 
        ) = crate::RNG.with(
            |r| {
                let mut rng = (**r).borrow_mut();
                let render_rot = self.default_render_rot_range()
                    .map(|r| rng.gen_range(r))
                    .unwrap_or(0.);
                let render_rot_speed = self.render_rot_speed_range()
                    .map(|r| 
                        rng.gen_range(r) 
                        * if rng.gen_bool(1. / 2.) { -1. } else { 1. }
                    )
                    .unwrap_or(0.);
                let vel = self.vel_0() + if vel_diffuse {
                    self.vel0_diffuse().map_or(
                        0., 
                        |r| rng.gen_range(r)
                    )
                } else { 0. };
                let health = self.health() + self.health_diffuse()
                    .map_or(
                        0., 
                        |r| rng.gen_range(r)
                    );
                (
                    render_rot * std::f32::consts::PI / 180., 
                    render_rot_speed * std::f32::consts::PI / 180., 
                    vel, 
                    health, 
                )
            }
        );    
        Enemy {
            ident,
            killed: false, 
            enemy_type: self, 
            position,
            rotation,
            render_rot,
            render_rot_speed,
            vel,
            velocity: nalgebra::Vector2::new(
                vel * rotation.cos(), 
                vel * rotation.sin(), 
            ),
            health, 
        }
    }
}

pub struct Enemy {
    pub ident: EnemyIdent, 
    pub killed: bool, 
    enemy_type: EnemyType, 
    position: nalgebra::Point2<f32>, 
    rotation: f32, 
    render_rot: f32, 
    render_rot_speed: f32, 
    vel: f32, 
    velocity: nalgebra::Vector2<f32>, 
    health: f32, 
}
impl Enemy {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        spawner: &mut super::spawn::EnemySpawnerArray, 
    ) -> bool {
        self.velocity = [
            self.vel * self.rotation.cos(), 
            self.vel * self.rotation.sin(), 
        ].into();
        self.position += self.velocity * cycle.dur;
        self.render_rot += self.render_rot_speed * cycle.dur;
        varea.in_visible(self.position, self.enemy_type.size())
        && !self.killed
        && 0. <= self.health
    }

    pub fn give_damage(
        &mut self, 
        damage: f32, 
    ) {
        self.health -= damage;
    }
}
impl physic::PhysicBody for Enemy {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.enemy_type.size()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}
impl InstanceGen<ImgObjInstance> for Enemy {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance {
            position: self.position.into(),
            size: self.enemy_type.size().into(),
            rotation: self.render_rot,
            tex_coord: self.enemy_type.tex_coord(),
            tex_size: self.enemy_type.tex_size(),
            tex_rev: [false, false],
        })
    }
}

#[derive(Clone)]
pub struct EnemyRef {
    pub ident: EnemyIdent, 
    pub idx: usize, 
}