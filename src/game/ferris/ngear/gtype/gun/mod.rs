//! 砲タイプのギア

use tm_wg_wrapper::{
    prelude::*, util::simple2d::{entity_holder::EntityRefMut, physic::aabb}, 
};
use crate::game::{ferris::ngear::GPhysWrap, enemy::enemy::Enemy};

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
            GunType::LightRifle => GunType::ShotGun,
        },
        GTToggle::Backward => match self {
            GunType::ShotGun => GunType::LightRifle,
            GunType::GutlingGun => GunType::ShotGun,
            GunType::MachineGun => GunType::GutlingGun,
            GunType::MachineRifle => GunType::MachineGun,
            GunType::LightRifle => GunType::MachineRifle,
        },
    }}

    pub fn cool_time(&self) -> f32 { match self {
        GunType::ShotGun => 1. / 3.,
        GunType::GutlingGun => 1. / 60.,
        GunType::MachineGun => 1. / 42.,
        GunType::MachineRifle => 1. / 12.,
        GunType::LightRifle => 1. / 6.,
    }}

    pub fn shoot_count(&self) -> u32 { match self {
        GunType::ShotGun => 60,
        GunType::GutlingGun => 3,
        GunType::MachineGun => 1,
        GunType::MachineRifle => 1,
        GunType::LightRifle => 1,
    }}

    pub fn shoot_shell(&self) -> GunGearType {
        match self {
            GunType::ShotGun => GunGearType::ShotPellet,
            GunType::GutlingGun => GunGearType::SmallGunBullet,
            GunType::MachineGun => GunGearType::MiddleGunBullet,
            GunType::MachineRifle => GunGearType::LargeGunBullet,
            GunType::LightRifle => GunGearType::SmallRifleShell,
        }
    }

    pub fn shoot(
        &self, 
        ferris: &crate::game::ferris::ferris::FerrisBody, 
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
            gt: super::GType::GunShot(gt),
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
    ) { if self.ct == 0.0 {
        for _ in 0..self.gt.shoot_count() {
            let gear = self.gt.shoot(ferris);
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

/// 砲タイプのギアの形式
#[derive(Clone)]
pub enum GunGearType {
    ShotPellet, 
    SmallGunBullet, 
    MiddleGunBullet, 
    LargeGunBullet, 
    SmallRifleShell, 
}
impl GunGearType {
    pub fn damage(&self) -> f32 { match self {
        GunGearType::ShotPellet => 1.,
        GunGearType::SmallGunBullet => 1.25,
        GunGearType::MiddleGunBullet => 3.,
        GunGearType::LargeGunBullet => 12.,
        GunGearType::SmallRifleShell => 48.,
    } }
}
impl super::GTypeTrait for GunGearType {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GunGearType::ShotPellet => Some(
            -std::f32::consts::PI * 0.3 .. std::f32::consts::PI * 0.3
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
    }}

    fn vel_default(&self) -> f32 { match self {
        GunGearType::ShotPellet => 720.,
        GunGearType::SmallGunBullet => 1830.,
        GunGearType::MiddleGunBullet => 1920.,
        GunGearType::LargeGunBullet => 1800.,
        GunGearType::SmallRifleShell => 1650.,
    }}

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GunGearType::ShotPellet => Some(-280.0..280.0),
        GunGearType::SmallGunBullet => Some(-120.0..120.0),
        GunGearType::MiddleGunBullet => Some(-95.0..95.0),
        GunGearType::LargeGunBullet => Some(-80.0..80.0),
        GunGearType::SmallRifleShell => None,
    }}

    fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GunGearType::ShotPellet => [8., 8.].into(),
        GunGearType::SmallGunBullet => [10., 10.].into(),
        GunGearType::MiddleGunBullet => [14., 14.].into(),
        GunGearType::LargeGunBullet => [24., 24.].into(),
        GunGearType::SmallRifleShell => [32., 32.].into(),
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
    }}

    fn update(
        &mut self, 
        _cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        _ident: &crate::game::ferris::ngear::array::GearIdent, 
        phys: &mut crate::game::ferris::ngear::GearPhys, 
        _ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        _aim: &simple2d::entity_holder::EntityHolder<
            simple2d::img_obj::ImgObjInstance, 
            crate::game::ferris::aim::Aim, 
        >, 
        enemies: &mut crate::game::enemy::enemy::EnemyArray, 
    ) -> bool {
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
        
        if let Some(e) = eref {
            e.give_damage(self.damage());
            false
        } else {
            varea.in_visible(
                phys.position, 
                self.size()
            )
        }
    }
}