//! 砲タイプのギア

use tm_wg_wrapper::{
    prelude::*, util::simple2d::{entity_holder::EntityRefMut, physic::aabb}, 
};
use crate::game::{ferris::ngear::{GPhysWrap, gcomm::{explode::ExplodeParam, GComm}}, enemy::enemy::Enemy};

use super::GTypeTrait;

/// ギアの種類トグル用のデータ
#[derive(Clone)]
pub enum GTToggle {
    Forward, 
    Backward, 
}

/// ギアを発射する砲の形式
pub enum GunType {
    ShotGun, 
    GutlingGun, 
    MachineGun, 
    MachineRifle, 
    LightRifle, 
    MiddleRifle, 
}
impl GunType {
    pub fn toggle(
        &mut self, 
        gt_toggle: GTToggle, 
    ) { *self = match gt_toggle {
        GTToggle::Forward => match self {
            GunType::ShotGun => GunType::GutlingGun,
            GunType::GutlingGun => GunType::MachineGun,
            GunType::MachineGun => GunType::MachineRifle,
            GunType::MachineRifle => GunType::LightRifle,
            GunType::LightRifle => GunType::MiddleRifle,
            GunType::MiddleRifle => GunType::ShotGun, 
        },
        GTToggle::Backward => match self {
            GunType::ShotGun => GunType::MiddleRifle,
            GunType::GutlingGun => GunType::ShotGun,
            GunType::MachineGun => GunType::GutlingGun,
            GunType::MachineRifle => GunType::MachineGun,
            GunType::LightRifle => GunType::MachineRifle,
            GunType::MiddleRifle => GunType::LightRifle, 
        },
    }}

    pub fn cool_time(&self) -> f32 { match self {
        GunType::ShotGun => 1. / 1.25,
        GunType::GutlingGun => 1. / 60.,
        GunType::MachineGun => 1. / 42.,
        GunType::MachineRifle => 1. / 16.,
        GunType::LightRifle => 1. / 2.,
        GunType::MiddleRifle => 1.5, 
    }}

    pub fn shoot_count(&self) -> u32 { match self {
        GunType::ShotGun => 40,
        GunType::GutlingGun => 3,
        GunType::MachineGun => 1,
        GunType::MachineRifle => 1,
        GunType::LightRifle => 1,
        GunType::MiddleRifle => 1, 
    }}

    pub fn shoot_shell(&self) -> GunGearType {
        match self {
            GunType::ShotGun => GunGearType::ShotPellet,
            GunType::GutlingGun => GunGearType::SmallGunBullet,
            GunType::MachineGun => GunGearType::MiddleGunBullet,
            GunType::MachineRifle => GunGearType::LargeGunBullet,
            GunType::LightRifle => GunGearType::SmallRifleShell,
            GunType::MiddleRifle => GunGearType::MiddleRifleShell, 
        }
    }

    pub fn shoot(
        &self, 
        ferris: &crate::game::ferris::ferris::FerrisBody, 
        fuze_time: Option<f32>, 
    ) -> super::super::GearBody {
        let gt = self.shoot_shell();
        let (phys, tex_rot_speed) = crate::RNG.with(|r| {
            let mut rng = r.borrow_mut();
            let gp = gt.vel_calc(
                &mut *rng, 
                ferris.position, 
                ferris.rotation + std::f32::consts::PI * 0.5, 
                ferris.velocity
            ); 
            let tr = gt.calc_tex_rot(&mut *rng);
            (gp, tr)
        });
        super::super::GearBody {
            phys,
            tex_rot_speed,
            tex_rot: 0.,
            gt: super::GType::GunShot(GunGear {
                ty: gt, 
                fuze_time, 
            }),
        }
    }
}

/// ギアを発射する砲
pub struct GearGun {
    pub gt: GunType, 
    ct: f32, 
}
impl Default for GearGun {
    fn default() -> Self {
        Self { gt: GunType::ShotGun, ct: 0. }
    }
}
impl GearGun {

    /// 射撃処理
    pub fn shoot(
        &mut self, 
        ferris: &crate::game::ferris::ferris::FerrisBody, 
        gears: &mut super::super::array::GearInstances, 
        fuze_time: Option<f32>, 
    ) { if self.ct == 0.0 {
        for _ in 0..self.gt.shoot_count() {
            let gear = self.gt.shoot(ferris, fuze_time);
            gears.push_gb(gear);
        }

        self.ct += self.gt.cool_time();
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

/// 砲タイプのギアの構造体
#[derive(Clone)]
pub struct GunGear {
    pub ty: GunGearType, 
    fuze_time: Option<f32>, 
}
impl super::GTypeTrait for GunGear {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> {
        self.ty.angle_diff()
    }

    fn vel_default(&self) -> f32 {
        self.ty.vel_default()
    }

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> {
        self.ty.vel_diff()
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.ty.size()
    }

    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>> {
        self.ty.tex_rot_diff()
    }

    fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        _ident: &crate::game::ferris::ngear::array::GearIdent, 
        phys: &mut crate::game::ferris::ngear::GearPhys, 
        _ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        _aim: &simple2d::entity_holder::EntityHolder<
            simple2d::img_obj::ImgObjInstance, 
            super::super::super::aim::Aim, 
        >, 
        enemies: &mut crate::game::enemy::enemy::EnemyArray, 
        gcomm: &mut crate::game::ferris::ngear::gcomm::GCommQueue, 
    ) -> bool {
        self.fuze_time.as_mut()
            .map(|ft| *ft -= cycle.dur);

        let eref = enemies.enemies.iter_mut()
            .map(|EntityRefMut { entity, .. }| entity)
            .filter(|entity| aabb(*entity, &GPhysWrap {
                gt: &super::GType::GunShot(self.clone()),
                phys: phys,
            }))
            .map(|entity| {
                let dist = entity.position - phys.position;
                ((dist.x.powi(2) + dist.y.powi(2)).sqrt(), entity)
            })
            .fold(
                None::<(f32, &mut Enemy)>, 
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
        
        let mut explode = || {
            if let Some(exp) = self.ty.explode()
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
        };
        if let Some(e) = eref {
            e.give_damage(self.ty.damage());
            explode();
            false
        } else {
            if 0. < self.fuze_time.unwrap_or(1.) {
                varea.in_visible(
                    phys.position, 
                    self.size()
                )
            } else {
                explode();
                false
            }
        }
    }
}

/// 砲タイプのギアの形式
#[derive(Clone)]
pub enum GunGearType {
    ShotPellet, 
    SmallGunBullet, 
    MiddleGunBullet, 
    LargeGunBullet, 
    SmallRifleShell, 
    MiddleRifleShell, 
}
impl GunGearType {
    pub fn damage(&self) -> f32 { match self {
        GunGearType::ShotPellet => 1.,
        GunGearType::SmallGunBullet => 1.25,
        GunGearType::MiddleGunBullet => 3.,
        GunGearType::LargeGunBullet => 12.,
        GunGearType::SmallRifleShell => 48.,
        GunGearType::MiddleRifleShell => 128., 
    } }

    pub fn explode(&self) -> Option<
        ExplodeParam
    > { match self {
        GunGearType::MiddleGunBullet => Some(ExplodeParam { 
            tex_rot: Some(
                -120.0 * (std::f32::consts::PI / 180.)
                .. 120.0 * (std::f32::consts::PI / 180.)
            ), 
            frag_count: 24, 
            frag_diff: Some(-4..8), 
            frvel_base: 480., 
            frvel_diff: Some(-300.0..300.0), 
            frsiz_base: 8., 
            frsiz_diff: Some(-3.0..3.0), 
            ltime_base: 1. / 10., 
            ltime_diff: Some(-1./20.0..1./20.), 
            damage_r: 1.
        }),
        GunGearType::LargeGunBullet => Some(ExplodeParam { 
            tex_rot: Some(
                -240.0 * (std::f32::consts::PI / 180.)
                .. 240.0 * (std::f32::consts::PI / 180.)
            ), 
            frag_count: 64, 
            frag_diff: Some(-16..32), 
            frvel_base: 600., 
            frvel_diff: Some(-400.0..400.0), 
            frsiz_base: 10., 
            frsiz_diff: Some(-4.5..4.5), 
            ltime_base: 1. / 6., 
            ltime_diff: Some(-1./15.0..1./15.), 
            damage_r: 2.
        }), 
        GunGearType::SmallRifleShell => Some(ExplodeParam { 
            tex_rot: Some(
                -360.0 * (std::f32::consts::PI / 180.)
                .. 360.0 * (std::f32::consts::PI / 180.)
            ), 
            frag_count: 128, 
            frag_diff: Some(-32..64), 
            frvel_base: 640., 
            frvel_diff: Some(-480.0..480.0), 
            frsiz_base: 12., 
            frsiz_diff: Some(-7.5..7.5), 
            ltime_base: 1. / 4., 
            ltime_diff: Some(-1./7.5..1./7.5), 
            damage_r: 3.
        }), 
        GunGearType::MiddleRifleShell => Some(ExplodeParam { 
            tex_rot: Some(
                -360.0 * (std::f32::consts::PI / 180.)
                .. 360.0 * (std::f32::consts::PI / 180.)
            ), 
            frag_count: 384, 
            frag_diff: Some(-128..256), 
            frvel_base: 720., 
            frvel_diff: Some(-520.0..520.0), 
            frsiz_base: 14., 
            frsiz_diff: Some(-8.0..8.0), 
            ltime_base: 1. / 3.2, 
            ltime_diff: Some(-1./7.5..1./7.5), 
            damage_r: 4.
        }), 
        
        _ => None, 
    }}
}
impl super::GTypeTrait for GunGearType {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GunGearType::ShotPellet => Some(
            -std::f32::consts::PI * 0.15 .. std::f32::consts::PI * 0.15
        ),
        GunGearType::SmallGunBullet => Some(
            -std::f32::consts::PI * 0.075 .. std::f32::consts::PI * 0.075
        ),
        GunGearType::MiddleGunBullet => Some(
            -std::f32::consts::PI * 0.075 .. std::f32::consts::PI * 0.075
        ),
        GunGearType::LargeGunBullet => Some(
            -std::f32::consts::PI * 0.05 .. std::f32::consts::PI * 0.05
        ),
        GunGearType::SmallRifleShell => Some(
            -std::f32::consts::PI * 0.025 .. std::f32::consts::PI * 0.025
        ),
        GunGearType::MiddleRifleShell => Some(
            -std::f32::consts::PI * 0.0125 .. std::f32::consts::PI * 0.0125
        ), 
    }}

    fn vel_default(&self) -> f32 { match self {
        GunGearType::ShotPellet => 720.,
        GunGearType::SmallGunBullet => 1830.,
        GunGearType::MiddleGunBullet => 1920.,
        GunGearType::LargeGunBullet => 1800.,
        GunGearType::SmallRifleShell => 1650.,
        GunGearType::MiddleRifleShell => 1400., 
    }}

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GunGearType::ShotPellet => Some(-280.0..280.0),
        GunGearType::SmallGunBullet => Some(-120.0..120.0),
        GunGearType::MiddleGunBullet => Some(-95.0..95.0),
        GunGearType::LargeGunBullet => Some(-80.0..80.0),
        GunGearType::SmallRifleShell => None,
        GunGearType::MiddleRifleShell => None, 
    }}

    fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GunGearType::ShotPellet => [8., 8.].into(),
        GunGearType::SmallGunBullet => [10., 10.].into(),
        GunGearType::MiddleGunBullet => [14., 14.].into(),
        GunGearType::LargeGunBullet => [24., 24.].into(),
        GunGearType::SmallRifleShell => [32., 32.].into(),
        GunGearType::MiddleRifleShell => [48., 48.].into(), 
    }}

    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GunGearType::ShotPellet => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
        GunGearType::SmallGunBullet => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
        GunGearType::MiddleGunBullet => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
        GunGearType::LargeGunBullet => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
        GunGearType::SmallRifleShell => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
        GunGearType::MiddleRifleShell => Some(
            -std::f32::consts::PI * 4. .. std::f32::consts::PI * 4.
        ),
    }}

    fn update(
        &mut self, 
        _cycle: &cycle_measure::CycleMeasure, 
        _varea: &simple2d::types::VisibleField, 
        _ident: &crate::game::ferris::ngear::array::GearIdent, 
        _phys: &mut crate::game::ferris::ngear::GearPhys, 
        _ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        _aim: &simple2d::entity_holder::EntityHolder<
            simple2d::img_obj::ImgObjInstance, 
            crate::game::ferris::aim::Aim, 
        >, 
        _enemies: &mut crate::game::enemy::enemy::EnemyArray, 
        _gcomm: &mut super::super::gcomm::GCommQueue, 
    ) -> bool { unreachable!() }
}