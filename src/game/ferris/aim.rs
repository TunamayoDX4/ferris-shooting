use tm_wg_wrapper::util::simple2d::physic::PhysicBody;

use crate::game::enemy::enemy::EnemyRef;

use super::*;

pub struct Aim {
    pub pbody: AimPhysicBody, 
    visible: bool, 
    pub state: AimState, 
}
impl Aim {
    pub fn new() -> Self { Self {
        pbody: AimPhysicBody {
            position: [0., 0.].into()
        }, 
        visible: true, 
        state: AimState::Normal, 
    }}

    pub fn input_mouse_motion(
        &mut self, 
        motion: nalgebra::Vector2<f32>, 
    ) {
        self.pbody.position += motion
    }

    pub fn update(
        &mut self, 
        varea: &VisibleField, 
        ferris: &ferris::Ferris, 
        enemies: &enemy::enemy::EnemyArray, 
        track_trigger: bool, 
        gear_vel: f32, 
    ) {
        let va = varea.visible_area();
        if self.pbody.position.x < va[0].x {
            self.pbody.position.x = va[0].x
        } else if va[1].x < self.pbody.position.x {
            self.pbody.position.x = va[1].x
        };

        if self.pbody.position.y < va[0].y {
            self.pbody.position.y = va[0].y
        } else if va[1].y < self.pbody.position.y {
            self.pbody.position.y = va[1].y
        };

        self.state.update(&mut self.pbody, ferris, enemies, track_trigger, gear_vel);
    }
}
impl InstanceGen<ImgObjInstance> for Aim {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance {
            position: self.pbody.position.into(),
            size: self.state.size().into(),
            rotation: 0.,
            tex_coord: self.state.tex_coord(),
            tex_size: self.state.tex_size(),
            tex_rev: [false, false],
        });

        match self.state {
            AimState::Tracking { 
                vec, 
                .. 
            } => {
                instances.push(ImgObjInstance {
                    position: (self.pbody.position + vec).into(),
                    size: [32., 32.],
                    rotation: 0.,
                    tex_coord: [0., 128.],
                    tex_size: [32., 32.],
                    tex_rev: [false, false],
                })
            }, 
            _ => {}, 
        }
    }
}

pub struct AimPhysicBody {
    pub position: nalgebra::Point2<f32>, 
}
impl physic::PhysicBody for AimPhysicBody {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        [64., 64.].into()
    }

    fn rotation(&self) -> f32 {
        0.
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        [0., 0.].into()
    }
}

#[derive(Clone)]
pub enum AimState {
    /// 通常状態
    Normal, 

    /// 追尾中
    Tracking {
        enemy: enemy::enemy::EnemyRef, 
        vec: nalgebra::Vector2<f32>, 
    }, 

    /// 追尾OK
    TrackReady, 

    /// 機能不全
    Failure, 
}
impl AimState {
    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        _ => [64., 64.].into(), 
    }}

    pub fn tex_coord(&self) -> [f32; 2] { match self {
        AimState::Normal => [0., 0.],
        AimState::Tracking{..} => [64., 0.],
        AimState::TrackReady => [0., 64.],
        AimState::Failure => [64., 64.],
    }}

    pub fn tex_size(&self) -> [f32; 2] { match self {
        _ => [64., 64.], 
    }}

    pub fn update(
        &mut self, 
        pbody: &mut AimPhysicBody,  
        ferris: &ferris::Ferris, 
        enemies: &enemy::enemy::EnemyArray, 
        track_trigger: bool, 
        vel: f32, 
    ) { match self {
        AimState::Normal => if let Some(_) = enemies.enemies.iter()
            .filter(|e| physic::aabb(pbody, e.entity))
            .next() {
                *self = Self::TrackReady;
            },
        AimState::Tracking { enemy, vec } => if !track_trigger {
                if let Some(e) = enemies.get(enemy) {
                pbody.position = e.position();
                *vec = physic::deviation_pos(
                    ferris, 
                    e, 
                    vel
                ) - pbody.position;
            } else {
                *self = Self::Normal
            }
        } else {
            *self = Self::Normal
        },
        AimState::TrackReady => match enemies.enemies.iter()
            .filter(|e| physic::aabb(pbody, e.entity))
            .fold(
                None, 
                |
                    init, 
                    e, 
                | match init {
                    None => Some(e), 
                    Some(ie) => {
                        let d = [
                            pbody.position - e.entity.position(), 
                            pbody.position - ie.entity.position(), 
                        ];
                        let d = [
                            f32::sqrt(d[0].x.powi(2) + d[0].y.powi(2)), 
                            f32::sqrt(d[1].x.powi(2) + d[1].y.powi(2)),  
                        ];
                        if d[1] <= d[0] {
                            Some(ie)
                        } else {
                            Some(e)
                        }
                    }, 
                }
            ) {
                None => *self = Self::Normal, 
                Some(e) => if track_trigger {
                    let vec = physic::deviation_pos(
                        ferris, 
                        e.entity, 
                        vel
                    ) - pbody.position;
                    *self = Self::Tracking { 
                        enemy: EnemyRef {
                            ident: e.entity.ident.clone(), 
                            idx: e.idx, 
                        }, 
                        vec 
                    }
                }, 
            },
        AimState::Failure => todo!(),
    }}
}