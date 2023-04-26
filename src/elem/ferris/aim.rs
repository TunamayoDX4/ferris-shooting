use crate::elem::physic_body::PhysicBody;

use super::*;

pub enum AimRO<'a> {
    Aim(&'a Aim), 
    SubAim(&'a SubAim), 
}
impl<'a> InstanceGen<ImgObjInstance> for AimRO<'a> {
    fn generate(&self) -> ImgObjInstance { match self {
        AimRO::Aim(a) => a.generate(),
        AimRO::SubAim(sa) => sa.generate(),
    }}
}
pub struct Aim {
    pub prev_position: nalgebra::Point2<f32>, 
    pub position: nalgebra::Point2<f32>, 
    pub velocity: nalgebra::Vector2<f32>, 
    pub push_autoaim: crate::util::Trigger, 
    pub aiming_target: Option<(usize, u64, SubAim)>, 
}
impl Aim {
    pub fn moving(
        &mut self, 
        varea: &VisibleField, 
        motion: impl Into<nalgebra::Vector2<f32>>, 
    ) {
        if self.aiming_target.is_none() {
            self.position += motion.into();

            // 画面外処理
            let varea = varea.visible_area();
            if self.position.x < varea[0].x {
                self.position.x = varea[0].x 
            } else if varea[1].x < self.position.x {
                self.position.x = varea[1].x
            }
            if self.position.y < varea[0].y {
                self.position.y = varea[0].y 
            } else if varea[1].y < self.position.y {
                self.position.y = varea[1].y
            }
        }
    }

    pub fn update(
        &mut self, 
        ferris: &Ferris, 
        cycle: &CycleMeasure, 
        enemies: &EntityArray<enemy::Enemy>, 
    ) {
        if self.push_autoaim.get_trig_count() == 1 && self.aiming_target.is_none() {
            enemies.iter()
                .map(|(idx, e)| (idx, e.ident, e))
                .fold(None, |
                    i: Option<(usize, u64, f32, &enemy::Enemy)>, (
                        idx, 
                        ident, 
                        enemy
                    )
                | {
                    let v = self.position - enemy.position();
                    let dist = (v.x.powi(2) + v.y.powi(2)).sqrt();
                    if let Some(p) = i {
                        if dist < p.2 {
                            Some((idx, ident, dist, enemy))
                        } else {
                            Some(p)
                        }
                    } else {
                        return Some((idx, ident, dist, enemy))
                    }
                })
                .map(|v| (
                    v.0, v.1, SubAim{position: physic_body::deviation_pos(
                        ferris, 
                        v.3, 
                        ferris.gear.gear_type.velocity0(), 
                    )}
                ))
                .map(|v| self.aiming_target = Some(v));
        } else if self.push_autoaim.get_trig_count() == 1 {
            self.aiming_target = None
        }
        if let Some((target, sub_aim)) = self.aiming_target
            .as_mut()
            .map(|(idx, ident, sub_aim)| enemies.get(*idx)
                .map(|tg| if tg.ident == *ident { 
                    Some((tg, sub_aim)) 
                } else { None })
                .flatten())
            .flatten()
        {
            self.position = target.position();
            sub_aim.position = physic_body::deviation_pos(
                ferris, 
                target, 
                ferris.gear.gear_type.velocity0(), 
            );
        } else {
            self.aiming_target = None
        }
        self.push_autoaim.update();
        self.velocity = (self.position - self.prev_position) * cycle.cps;
        self.prev_position = self.position;
    }
}
impl physic_body::PhysicBody for Aim {
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
        self.velocity
    }
}
impl InstanceGen<ImgObjInstance> for Aim {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance {
        position: self.position.into(),
        size: self.size().into(),
        rotation: self.rotation(),
        tex_coord: if self.aiming_target.is_none() { [0., 0.] } else { [64., 0.] },
        tex_size: self.size().into(),
        tex_rev: [false, false],
    }}
}

pub struct SubAim {
    pub position: nalgebra::Point2<f32>, 
}
impl InstanceGen<ImgObjInstance> for SubAim {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance {
        position: self.position.into(),
        size: [32., 32.],
        rotation: 0.,
        tex_coord: [0., 64.],
        tex_size: [32., 32.],
        tex_rev: [false, false],
    }}
}