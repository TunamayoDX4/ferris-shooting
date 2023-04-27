use super::*;

#[derive(Clone)]
pub struct SwarmMissile {
    target: Option<((usize, u64), f32)>, 
}
impl Default for SwarmMissile {
    fn default() -> Self {
        Self { target: None }
    }
}
impl SwarmMissile {
    const SEEKER_WIDTH: f32 = std::f32::consts::PI / 3.;
    const SEEKER_RANGE: f32 = 750.;
    const TRACK_ROT: f32 = 120. * (std::f32::consts::PI / 180.);

    pub fn seek(
        &mut self, 
        pbody: &GearPhysicBody, 
        enemies: &EntityArray<enemy::Enemy>
    ) {
        self.target = match self.target.map_or_else(
            || enemies.iter()
                .map(
                    |(idx, enemy)| (idx, enemy, enemy.position - pbody.position)
                )
                .map(|(idx, enemy, dist)| (
                    idx, 
                    enemy, 
                    dist, 
                    (dist.x.powi(2) + dist.y.powi(2)).sqrt()
                ))
                .filter(|(_, _, _dist, dist_1d)| 
                    dist_1d.abs() < Self::SEEKER_RANGE
                )
                .filter(|(_, _, dist, _)| (
                    (
                        (f32::atan2(dist.y, dist.x) - pbody.rotation)
                        + std::f32::consts::PI
                    ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI
                ).abs() < Self::SEEKER_WIDTH)
                .fold(
                    None, 
                    |
                        dist: Option<((usize, u64), f32, &enemy::Enemy)>, 
                        (
                            idx, 
                            enemy, 
                            _, 
                            dist_1d, 
                        )
                    | match dist {
                        Some((
                            _, tg_dist_1d, _
                        )) if tg_dist_1d < dist_1d => Some(((idx, enemy.ident), dist_1d, enemy)),
                        v @ Some(_) => v, 
                        None => Some(((idx, enemy.ident), dist_1d, enemy)),
                    }
                )
                .map(|(ident, _dist_1d, enemy)| (ident, enemy)), 
            |
                ((idx, ident), _)
            | enemies.get(idx)
                .filter(|enemy| enemy.ident == ident)
                .map(|e| {
                    ((idx, e.ident), e)
                })
        ) {
            Some((ident, target)) => {
                let tg_vec = physic_body::deviation_pos(
                    pbody, 
                    target, 
                    pbody.vel
                ) - pbody.position;
                let tg_abs_angle = f32::atan2(
                    tg_vec.y, 
                    tg_vec.x, 
                );
                Some((ident, tg_abs_angle))
            }, 
            _ => { None }, 
        };
    }

    pub fn track(
        &mut self, 
        pbody: &mut GearPhysicBody, 
        cycle: &CycleMeasure, 
    ) {
        if let Some((_, angle)) = self.target {
            let angle_diff = (
                angle - pbody.rotation + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.) - std::f32::consts::PI;
            if angle_diff < -Self::TRACK_ROT * cycle.dur {
                pbody.rotation -= Self::TRACK_ROT * cycle.dur
            } else if Self::TRACK_ROT * cycle.dur < angle_diff {
                pbody.rotation += Self::TRACK_ROT * cycle.dur
            } else {
                pbody.rotation = angle
            }
        }
    }
}

#[derive(Clone)]
pub enum GearMissileType {
    SwarmMissile(SwarmMissile), 
}
impl Default for GearMissileType {
    fn default() -> Self {
        Self::SwarmMissile(Default::default())
    }
}
impl GearMissileType {
    pub fn weight(&self) -> f32 { match self {
        GearMissileType::SwarmMissile(_) => 4.,
    } }

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GearMissileType::SwarmMissile(_) => [18., 18.],
    }.into()}

    pub fn seek (
        &mut self, 
        pbody: &GearPhysicBody, 
        enemies: &EntityArray<enemy::Enemy>, 
    ) { match self {
        GearMissileType::SwarmMissile(sm) => sm.seek(pbody, enemies),
    }}

    pub fn track(
        &mut self, 
        pbody: &mut GearPhysicBody, 
        cycle: &CycleMeasure, 
    ) { match self {
        GearMissileType::SwarmMissile(sm) => sm.track(pbody, cycle),
    }}

    pub fn velocity0 (&self) -> f32 { match self {
        GearMissileType::SwarmMissile(_) => 720.,
    } }

    pub(super) fn angle_diffuse(
        &self, 
    ) -> Option<f32> { match self {
        GearMissileType::SwarmMissile(_) => Some(30.), 
    }}

    pub(super) fn vel_diffuse(
        &self, 
    ) -> Option<f32> { match self {
        GearMissileType::SwarmMissile(_) => None,
    } }

    fn shift_next(&mut self) { *self = match self {
        GearMissileType::SwarmMissile(_) => GearMissileType::SwarmMissile(
            SwarmMissile { target: None }
        ),
    } }

    fn shift_back(&mut self) { *self = match self {
        GearMissileType::SwarmMissile(_) => GearMissileType::SwarmMissile(
            SwarmMissile { target: None }
        ),
    } }

    fn cool_time_sec(&self) -> f32 { match self {
        GearMissileType::SwarmMissile(_) => 1. / 3.,
    }}

    fn shoot_count(&self) -> u32 { match self {
        GearMissileType::SwarmMissile(_) => 4,
    } }
}

/// ギアミサイルの発射機
pub struct GearMissile {
    pub gear_type: GearMissileType, 
    cooling_dur: f32, 
}
impl GearMissile {
    pub fn new() -> Self { Self {
        gear_type: GearMissileType::default(),
        cooling_dur: 0.,
    }}

    pub fn shift_mode(
        &mut self, 
        mode: &crate::util::RevMode, 
    ) { match mode {
        crate::util::RevMode::Forward => self.gear_type.shift_next(), 
        crate::util::RevMode::Backward => self.gear_type.shift_back(), 
        _ => {}, 
    }}

    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        position: nalgebra::Point2<f32>, 
        rotation: f32, 
        velocity: nalgebra::Vector2<f32>,  
        shoot: bool, 
        gear: &mut instance::GearInstance, 
    ) {
        if 0. < self.cooling_dur { self.cooling_dur -= cycle.dur }
        else { self.cooling_dur = 0. }

        if self.cooling_dur <= 0. && shoot {
            gear.spawn_gear(
                position, 
                rotation, 
                Some(velocity), 
                super::GearType::Missile(self.gear_type.clone()), 
                1., 
                1., 
                self.gear_type.shoot_count()..=self.gear_type.shoot_count(), 
            );
            self.cooling_dur += self.gear_type.cool_time_sec();
        }
    }

    pub fn is_cooling(
        &self, 
    ) -> bool {
        self.cooling_dur != 0.
    }
}