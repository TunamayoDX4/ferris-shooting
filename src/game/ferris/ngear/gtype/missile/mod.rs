//! ミサイルタイプのギア

use tm_wg_wrapper::{
    prelude::*, 
    util::simple2d::{physic::{PhysicBody, self}, entity_holder::EntityRefMut}, 
};

use crate::game::{physic::aabb, ferris::ngear::gcomm::{explode::ExplodeParam, GComm}};
use crate::game::enemy::{self, enemy::EnemyRef};

use super::GTypeTrait;

pub mod choice;

/// 発射するミサイルの形式
pub enum LaunchMissileType {
    LightMissile, 
}

/// ミサイル発射機
pub struct MissileLauncher {
    ct: f32, 
}
impl Default for MissileLauncher {
    fn default() -> Self {
        Self { ct: 0. }
    }
}
impl MissileLauncher {

    /// 射撃処理
    pub fn shoot(
        &mut self, 
        ferris: &crate::game::ferris::ferris::FerrisBody, 
        aim: Option<&crate::game::ferris::aim::Aim>, 
        gears: &mut super::super::array::GearInstances, 
    ) { if self.ct == 0.0 {
        let target = aim.map(|s| match &s.state {
            crate::game::ferris::aim::AimState::Tracking { 
                enemy, .. 
            } => Some(enemy.clone()),
            _ => None, 
        }).flatten();
        let lm = MissileGearType::LightMissile(
            LightMissile { fcs_controlled: target.is_some(), target }
        );
        let gb = crate::RNG.with(|r| {
            let mut rng = r.borrow_mut();
            let gp = lm.vel_calc(
                &mut *rng, 
                ferris.position, 
                ferris.rotation + std::f32::consts::PI * 0.5, 
                ferris.velocity
            );
            let tr = lm.calc_tex_rot(&mut *rng);
            super::super::GearBody {
                phys: gp, 
                tex_rot_speed: tr, 
                tex_rot: 0., 
                gt: super::GType::Missile(lm), 
            }
        });
        gears.push_gb(gb);

        self.ct += 0.25;
    }}

    /// 更新処理
    /// クールタイムの計算をします
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
    ) { if 0. < self.ct {
        self.ct -= cycle.dur
    } else {
        self.ct = 0.
    }}
}

/// ミサイルの誘導方式
pub enum MissileHomingMode {
    /// 単純誘導
    /// ターゲット座標を単純に追尾する
    PureNavigate, 
    /// 比例誘導
    /// 移動ターゲットとの相対速度から追尾する
    ProportionalNavigate, 
}
impl MissileHomingMode {
    pub fn calc_homing_v(
        &self, 
        own: &impl PhysicBody, 
        target: &impl PhysicBody, 
    ) -> nalgebra::Vector2<f32> { match self {
        MissileHomingMode::PureNavigate => {
            target.position() - own.position()
        },
        MissileHomingMode::ProportionalNavigate => {
            physic::deviation_pos(
                own, 
                target, 
                {
                    let ovel = own.velocity();
                    (ovel.x.powi(2) + ovel.y.powi(2)).sqrt()
                }
            ) - own.position()
        },
    }}
}

/// ミサイルギアの形式
#[derive(Clone)]
pub enum MissileGearType {
    LightMissile(LightMissile), 
}
impl MissileGearType {
    pub fn damage(&self) -> f32 { match self {
        MissileGearType::LightMissile(_) => 20.,
    } }

    pub fn mode(&self) -> MissileHomingMode { match self {
        MissileGearType::LightMissile(_) => MissileHomingMode::ProportionalNavigate,
    }}

    pub fn explode(&self) -> Option<ExplodeParam> { match self {
        MissileGearType::LightMissile(_) => Some(ExplodeParam { 
            tex_rot: Some(
                -360.0 * (std::f32::consts::PI / 180.)
                .. 360.0 * (std::f32::consts::PI / 180.)
            ), 
            frag_count: 96, 
            frag_diff: Some(-12..12), 
            frvel_base: 320., 
            frvel_diff: Some(-640.0..640.0), 
            frsiz_base: 8., 
            frsiz_diff: Some(-3.2..3.2), 
            ltime_base: 1. / 5., 
            ltime_diff: Some(-1./7.5..1./7.5), 
            damage_r: 2.
        }),
    }}
}
impl super::super::GTypeTrait for MissileGearType {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> {
        None
    }

    fn vel_default(&self) -> f32 {
        120.
    }

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> {
        None
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        [20., 20.].into()
    }

    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>> {
        Some(-std::f32::consts::PI * 2.0 .. std::f32::consts::PI * 2.0)
    }

    fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        _ident: &crate::game::ferris::ngear::array::GearIdent, 
        phys: &mut crate::game::ferris::ngear::GearPhys, 
        ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        _aim: &simple2d::entity_holder::EntityHolder<
            simple2d::img_obj::ImgObjInstance, crate::game::ferris::aim::Aim, 
        >, 
        enemies: &mut enemy::enemy::EnemyArray, 
        gcomm: &mut super::super::gcomm::GCommQueue, 
    ) -> bool { 
        let s = self.clone();
        match self {
            Self::LightMissile(lm) => {
                lm.remove_none_target(enemies);
                if let Some(ferris) = ferris {
                    lm.seek_target(
                        super::super::GPhysWrap {
                            gt: &super::GType::Missile(s.clone()),
                            phys,
                        }, 
                        ferris, 
                        enemies
                    );
                }

                // 追尾機能
                lm.homing(
                    cycle, 
                    enemies, 
                    super::super::GPhysWrapMut {
                        gt: &super::GType::Missile(s.clone()), 
                        phys
                    }
                );

                // 速度の更新
                let (vup, vmax) = {
                    let (vup, vmax) = match self {
                        MissileGearType::LightMissile(lm) => lm.speed_boost_max(),
                    };
                    (vup * cycle.dur, vmax)
                };
                
                // 加速
                if phys.vel_a < vmax {
                    let tmp = phys.vel_a + vup;
                    if tmp < vmax {
                        phys.vel_a = tmp
                    } else {
                        phys.vel_a = vmax
                    }
                }

                let eref = enemies.enemies.iter_mut()
                    .map(|EntityRefMut { entity, .. }| entity)
                    .filter(|entity| aabb(*entity, &super::super::GPhysWrap {
                        gt: &super::GType::Missile(s.clone()),
                        phys: phys,
                    }))
                    .map(|entity| {
                        let dist = entity.position - phys.position;
                        ((dist.x.powi(2) + dist.y.powi(2)).sqrt(), entity)
                    })
                    .fold(
                        None::<(f32, &mut enemy::enemy::Enemy)>, 
                        |
                            init, 
                            tg, 
                        | match init {
                            None => Some(tg), 
                            Some(e) if tg.0 < e.0 => Some(tg), 
                            a @ Some(_) => a, 
                        }
                    )
                    .map(|(_, e)| e);
            
                if let Some(e) = eref {
                    e.give_damage(self.damage());
                    if let Some(exp) = self.explode()
                        .map(|param| GComm::Explode { 
                            param, 
                            position: phys.position, 
                            base_vel: [
                                phys.vel_a / 2. * phys.rotation.cos(), 
                                phys.vel_a / 2. * phys.rotation.sin(), 
                            ].into() 
                        }
                    ) {
                        gcomm.push(exp)
                    }
                    false
                } else {
                    varea.in_visible(
                        phys.position, 
                        self.size()
                    )
                }
            },
        }
    }
}

/// 軽量ミサイル
#[derive(Clone)]
pub struct LightMissile {
    target: Option<enemy::enemy::EnemyRef>, 
    fcs_controlled: bool, 
}
impl MissileTrait for LightMissile {
    fn mode(&self) -> MissileHomingMode {
        MissileHomingMode::ProportionalNavigate
    }

    fn target(&self) -> Option<&enemy::enemy::EnemyRef> {
        self.target.as_ref()
    }

    fn target_mut(&mut self) -> &mut Option<enemy::enemy::EnemyRef> {
        &mut self.target
    }

    fn rotation_speed(&self) -> f32 {
        120.
    }

    fn speed_boost_max(&self) -> (f32, f32) {
        (620., 1280.)
    }

    fn seek_target(
        &mut self, 
        phys: super::super::GPhysWrap, 
        _ferris: &crate::game::ferris::ferris::FerrisBody, 
        enemies: &enemy::enemy::EnemyArray, 
    ) {
        let fnc = |phys: &super::super::GPhysWrap, e: &enemy::enemy::Enemy| {
            let dist = e.position - phys.position();

            let angle = f32::atan2(dist.y, dist.x);
            // 相対角度を計算
            let angle_diff = (
                (angle - phys.rotation()) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs()
                - std::f32::consts::PI;

            // 相対距離の計算
            let dist = (dist.x.powi(2) + dist.y.powi(2)).sqrt();

            dist.abs() < 800. && angle_diff.abs() < std::f32::consts::PI * 0.167
        };
        match &mut self.target {
            e @ None => *e = choice::choice_simple_neerest(
                &phys, 
                enemies, 
                fnc, 
            ), 
            e @ Some(_) if !self.fcs_controlled => if let Some(
                tgt
            ) = enemies.get(
                e.as_ref().unwrap()
            ) {
                let dist = tgt.position - phys.phys.position;
                let angle = f32::atan2(dist.y, dist.x);
                let angle_diff = (
                    (angle - phys.phys.rotation) + std::f32::consts::PI
                ).rem_euclid(std::f32::consts::PI * 2.).abs()
                    - std::f32::consts::PI;
                if !(angle_diff.abs() < std::f32::consts::PI * 0.33) {
                    *e = None
                }
            }
            _ => {}, 
        }
    }
}

/// ミサイルの実装
pub trait MissileTrait {
    fn mode(&self) -> MissileHomingMode;
    fn target(&self) -> Option<&enemy::enemy::EnemyRef>;
    fn target_mut(&mut self) -> &mut Option<enemy::enemy::EnemyRef>;
    fn rotation_speed(&self) -> f32;
    fn speed_boost_max(&self) -> (f32, f32);

    fn seek_target(
        &mut self, 
        phys: super::super::GPhysWrap, 
        ferris: &crate::game::ferris::ferris::FerrisBody, 
        enemies: &enemy::enemy::EnemyArray, 
    );

    fn remove_none_target(
        &mut self, 
        enemies: &enemy::enemy::EnemyArray, 
    ) {
        if self.target()
            .map(|t| enemies.get(t))
            .flatten()
            .is_none()
        {
            *self.target_mut() = None
        }
    }

    fn homing(
        &self, 
        cycle: &cycle_measure::CycleMeasure, 
        enemies: &enemy::enemy::EnemyArray, 
        phys: super::super::GPhysWrapMut, 
    ) {
        if let Some(tgt) = self.target()
            .map(|t| enemies.get(t))
            .flatten()
        {

            let target = self.mode()
                .calc_homing_v(&phys, tgt);

            // 対象の絶対角度を計算
            let angle = f32::atan2(
                target.y, target.x
            );

            // 相対角度を計算
            let angle_diff = (
                (angle - phys.phys.rotation) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs()
                - std::f32::consts::PI;
            
            // 回転速度
            let rs = self.rotation_speed() 
                * (std::f32::consts::PI / 180.) 
                * cycle.dur;

            if angle_diff <= rs {
                phys.phys.rotation -= rs;
            } else if rs <= angle_diff {
                phys.phys.rotation += rs;
            } else {
                phys.phys.rotation = angle
            }
        }
    }
}