use tm_wg_wrapper::{prelude::nalgebra::{Vector2, Point2}, util::cycle_measure::{self, CycleMeasure}};

use crate::game::enemy::{enemy::{EnemyRef, EnemyArray}};

use super::GearInstances;

pub struct MissileShooter {
    selector: MissileSelector, 
}
impl MissileShooter {
    pub fn shoot(
        &mut self, 
        position: Point2<f32>, 
        velocity: Vector2<f32>, 
        rotation: f32, 
        gears: &mut GearInstances, 
        
    ) {}
}

/// 発射するミサイルを選択するためのセレクタ
#[derive(Clone)]
pub enum MissileSelector {
    SwarmMissile, 
}
impl MissileSelector {
    pub fn name(&self) -> &str { match self {
        Self::SwarmMissile => "スウォーム・ミサイル", 
    } }

    pub fn shift(&mut self, shift_con: MissileShiftCon) {
        *self = match shift_con {
            MissileShiftCon::Forward => match self {
                Self::SwarmMissile => Self::SwarmMissile,
            }, 
            MissileShiftCon::Backward => match self {
                Self::SwarmMissile => Self::SwarmMissile, 
            }, 
        }
    }
}

/// 発射されたミサイルのインスタンス
#[derive(Clone)]
pub enum MissileGear {
    SwarmMissile(SwarmMissile), 
}
#[derive(Clone, Copy)]
pub enum MissileShiftCon {
    Forward, 
    Backward, 
}
impl super::gear_type::TrGearType<MissileShiftCon, ()> for MissileGear {
    fn name(&self) -> &str { match self {
        Self::SwarmMissile(_) => "スウォーム・ミサイル", 
    }}

    fn shift(&mut self, shift_con: MissileShiftCon) -> () {
        match shift_con {
            MissileShiftCon::Forward => *self = match self {
                Self::SwarmMissile(sm) => Self::SwarmMissile(
                    sm.clone()
                ), 
            },
            MissileShiftCon::Backward => *self = match self {
                Self::SwarmMissile(sm) => Self::SwarmMissile(
                    sm.clone()
                ), 
            },
        }
    }

    fn size(&self) -> tm_wg_wrapper::prelude::nalgebra::Vector2<f32> { match self {
        MissileGear::SwarmMissile(_) => [24., 24.].into(),
    }}

    fn vel_const(&self) -> f32 { match self {
        MissileGear::SwarmMissile(_) => 600.
    }}

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        MissileGear::SwarmMissile(_) => None, 
    }}

    fn angle_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        MissileGear::SwarmMissile(_) => None, 
    }}
}
impl MissileGear {
    pub fn name(&self) -> &str { match self {
        Self::SwarmMissile(_) => "スウォームミサイル", 
    } }

    pub fn search(
        &mut self, 
        enemies: &EnemyArray, 
        position: &Point2<f32>, 
        _velocity: &Vector2<f32>, 
        rotation: &f32, 
    ) -> bool { match self {
        Self::SwarmMissile(sm) => sm.search(
            enemies, position, rotation
        ), 
    }}

    pub fn homing(
        &mut self, 
        cycle: &CycleMeasure, 
        enemies: &EnemyArray, 
        position: &mut Point2<f32>, 
        _velocity: &mut Vector2<f32>, 
        rotation: &mut f32, 
    ) { match self {
        MissileGear::SwarmMissile(sm) => sm.homing(
            cycle, 
            enemies, 
            position, 
            rotation, 
        ),
    }}
}

#[derive(Clone)]
pub struct SwarmMissile {
    target: Option<EnemyRef>, 
}
impl SwarmMissile {
    pub fn search(
        &mut self, 
        enemies: &EnemyArray, 
        position: &Point2<f32>, 
        rotation: &f32, 
    ) -> bool { match &self.target {
        None => {
            self.target = enemies.enemies.iter()
                .map(|enem| {
                    let en = enem.entity;
                    let enpos = en.position;
                    let distv = enpos - position;
                    let dist = f32::sqrt(
                        distv.x.powi(2) + distv.y.powi(2)
                    );
                    ((enem.idx, en.ident.clone()), (en, distv, dist.abs()))
                })
                .filter(
                    // 距離の遠いやつはこの時点で篩にかける
                    |(_, (_, _, dist))| *dist <= 400.
                )
                .map(|(idx, (
                    enem, 
                    distv, 
                    dist
                ))| {
                    let angle = f32::atan2(distv.y, distv.x);
                    let angle_diff = (
                        angle - (*rotation - std::f32::consts::PI * 0.5) + std::f32::consts::PI
                    ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;
                    (idx, (enem, (angle_diff, dist)))
                })
                .filter(
                    // 角度的に離れすぎてるやつも篩にかける
                    // 前方30度の400px以内にいるやつだけ狙う
                    |(_, (_, (ad, _)))| ad.abs() <= (
                        30. * (std::f32::consts::PI / 180.)
                    )
                )
                .map(|(
                    (idx, ident), 
                    enem
                )| {
                    let eref = EnemyRef {
                        idx, 
                        ident, 
                    };
                    (eref, enem)
                })
                .fold(
                    // 一番近いやつだけ残す
                    None::<(EnemyRef, (f32, f32))>, 
                    |
                        init, 
                        (eref, enem)
                    | match init {
                        None => Some((eref, enem.1)), 
                        e @ Some(_) 
                        if enem.1.1 < e.as_ref().unwrap().1.1 => Some((
                            eref, enem.1
                        )), 
                        e @ Some(_) => e, 
                    }
                )
                .map(|e| e.0);
            true
        },
        Some(e) => if let Some(e) = enemies.get(&e) {
            let distv = e.position - position;
            let dist = f32::sqrt(
                distv.x.powi(2) + distv.y.powi(2)
            );
            let angle = f32::atan2(
                distv.y, distv.x
            );
            let diff = (
                angle - (*rotation - std::f32::consts::PI * 0.5) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;
            if dist.abs() <= 400. && diff.abs() <= 30. * (std::f32::consts::PI / 180.) {
                false
            } else {
                self.target = None;
                true
            }
        } else {
            false
        }
    }}

    pub fn homing(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        enemies: &EnemyArray, 
        position: &Point2<f32>, 
        rotation: &mut f32, 
    ) { match &mut self.target {
        erf @ Some(_) => if let Some(
            e
        ) = enemies.get(erf.as_ref().unwrap()){
            // 敵が追尾できるなら追尾する。

            // 敵の距離とワールド角度、ローカル角度を取得する。
            let (dist, angle, angle_diff) = {
                let tg = e.position - position;
                let dist = (tg.x.powi(2) + tg.y.powi(2)).sqrt();
                let angle = f32::atan2(tg.y, tg.x);
                let angle_diff = (
                    (angle - (*rotation - std::f32::consts::PI * 0.5)) + std::f32::consts::PI
                ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;
                (dist, angle, angle_diff)
            };

            // 相対角度が30度以内、かつ距離が400以内であれば追尾継続
            if angle_diff.abs() <= (30. * (std::f32::consts::PI / 180.)) 
            && dist <= 400. { 
                // 旋回限界の計算(1秒で4周くらい)
                let max_rot = cycle.dur * (std::f32::consts::PI * 8.);
                if angle_diff.abs() <= max_rot {
                    *rotation = angle;
                } else if angle_diff.is_sign_positive() {
                    *rotation -= max_rot;
                } else {
                    *rotation += max_rot;
                }
            } else {
                // 角度・距離が大きすぎたら追尾解除
                *erf = None
            }
        } else { *erf = None }, 
        None => {}, 
    }}
}