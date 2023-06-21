use crate::RNG;

use super::*;

#[derive(Clone)]
pub enum GearType {
    GunGear(GunGear), 
    MissileGear(missile::MissileGear), 
}

pub trait TrGearType<Sc, ScR> {
    fn name(&self) -> &str;
    fn shift(&mut self, shift_con: Sc) -> ScR;
    fn size(&self) -> nalgebra::Vector2<f32>;
    fn vel_const(&self) -> f32;
    fn vel_diff(&self) -> Option<std::ops::Range<f32>>;
    fn vel_0(&self) -> f32 {
        self.vel_const() + self.vel_diff()
            .map(|fr| RNG.with(
                |r| r.borrow_mut().gen_range(fr)
            ))
            .unwrap_or(0.)
    }
    fn angle_diff(&self) -> Option<std::ops::Range<f32>>;
    fn angle_0(&self, set_angle: f32) -> f32 {
        set_angle + self.angle_diff()
            .map(|fr| RNG.with(
                |r| r.borrow_mut().gen_range(fr)
            ))
            .unwrap_or(0.)
    }
}

/// 発射する弾丸を選択するためのセレクタ
#[derive(Clone)]
pub enum GunGearSelector {
    MachineGun, 
    MachineCannon, 
    Gutling, 
    ShotGun, 
    RifleCannon, 
}

/// 発射された弾丸のインスタンス
#[derive(Clone)]
pub enum GunGear {
    MachineGun, 
    MachineCannon, 
    Gutling, 
    ShotGun, 
    RifleCannon, 
}
impl GunGear {
    pub fn name(&self) -> &str { match self {
        Self::MachineGun => "機関銃", 
        Self::MachineCannon => "連発砲", 
        Self::Gutling => "ガトリング銃", 
        Self::ShotGun => "ショット砲", 
        Self::RifleCannon => "カノン砲", 
    }}

    pub fn shift_next(&mut self) { *self = match self {
        Self::MachineGun => Self::MachineCannon,
        Self::MachineCannon => Self::Gutling, 
        Self::Gutling => Self::ShotGun, 
        Self::ShotGun => Self::RifleCannon, 
        Self::RifleCannon => Self::MachineGun, 
    }}

    pub fn shift_back(&mut self) { *self = match self {
        Self::MachineGun => Self::RifleCannon,
        Self::MachineCannon => Self::MachineGun, 
        Self::Gutling => Self::MachineCannon, 
        Self::ShotGun => Self::Gutling, 
        Self::RifleCannon => Self::ShotGun, 
    }}

    /// 大きさを得る。
    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        Self::MachineGun => [12., 12.].into(),
        Self::MachineCannon => [18., 18.].into(), 
        Self::Gutling => [12., 12.].into(), 
        Self::ShotGun => [8., 8.].into(), 
        Self::RifleCannon => [48., 48.].into(), 
    }}

    /// 初速を得る。
    pub fn vel_0(&self) -> f32 { match self {
        Self::MachineGun => 1200.,
        Self::MachineCannon => 1800., 
        Self::Gutling => 1350., 
        Self::ShotGun => 850., 
        Self::RifleCannon => 1580., 
    }}

    /// 初速拡散を得る。
    pub fn vel_0_diff(&self) -> f32 { match self {
        Self::MachineGun => 240.,
        Self::MachineCannon => 120., 
        Self::Gutling => 380., 
        Self::ShotGun => 300., 
        Self::RifleCannon => 40., 
    }}

    /// 初速の計算
    pub fn vel_0_calc(&self) -> f32 {
        self.vel_0() + crate::RNG.with(|r| 
            (**r).borrow_mut().gen_range(-1.0..1.0)
        ) * self.vel_0_diff()
    }

    /// 散布角を得る。
    pub fn angle(&self) -> f32 { match self {
        Self::MachineGun => 5.,
        Self::MachineCannon => 3.75, 
        Self::Gutling => 12., 
        Self::ShotGun => 25., 
        Self::RifleCannon => 1.2, 
    }}

    /// 散布角の計算
    pub fn angle_diff_calc(&self) -> f32 {
        crate::RNG.with(
            |r| (**r).borrow_mut().gen_range(-1.0..1.0)
        ) * self.angle()
    }

    /// 一度に射出する数
    pub fn shot_count(&self) -> u32 { match self {
        Self::MachineGun => 1,
        Self::MachineCannon => 1, 
        Self::Gutling => 3, 
        Self::ShotGun => 50, 
        Self::RifleCannon => 1, 
    } }

    /// 射出サイクルの時間
    pub fn cycle_dur(&self) -> f32 { match self {
        Self::MachineGun => 1. / 45.,
        Self::MachineCannon => 1. / 8., 
        Self::Gutling => 1. / 40., 
        Self::ShotGun => 1. / 1.5, 
        Self::RifleCannon => 1. / 1.5, 
    } }

    /*
    /// ギアのスポーン
    pub fn spawn(
        &self, 
        ident: GearIdent, 
        position: nalgebra::Point2<f32>, 
        velocity: nalgebra::Vector2<f32>, 
        rotation: f32, 
    ) -> Gear {
        let vel0 = self.vel_0_calc();
        let angle = rotation + self.angle_diff_calc() * (std::f32::consts::PI / 180.);
        let vel0 = nalgebra::Vector2::new(
            vel0 * angle.cos(), 
            vel0 * angle.sin() 
        ) + velocity;
        let rotation = f32::atan2(vel0.y, vel0.x);
        let vel = f32::sqrt(vel0.x.powi(2) + vel0.y.powi(2));
        Gear {
            ident,
            position,
            vel,
            rotation,
            velocity,
            render_rot: crate::RNG.with(|r|
                (**r).borrow_mut().gen_range(-1.0..1.0)
            ) * std::f32::consts::PI,
            render_rot_speed: crate::RNG.with(|r|
                (**r).borrow_mut().gen_range(-3.0..3.0)
            ) * std::f32::consts::PI,
            gtype: self.clone(),
        }
    }
    */
}