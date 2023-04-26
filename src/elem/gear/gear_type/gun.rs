use super::*;

#[derive(Default, Clone)]
pub enum GearGunType {
    #[default]
    MachineGun, 
    MachineCannon, 
    GutlingGun, 
    ShotGun, 
    RifleCannon, 
    Howitzer, 
}
impl GearGunType {
    pub fn weight(&self) -> f32 { match self {
        GearGunType::MachineGun => 1.5,
        GearGunType::MachineCannon => 4.0,
        GearGunType::GutlingGun => 1.0,
        GearGunType::ShotGun => 1.0,
        GearGunType::RifleCannon => 12.0,
        GearGunType::Howitzer => 32.0, 
    } }

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GearGunType::MachineGun => [14., 14.],
        GearGunType::MachineCannon => [20., 20.],
        GearGunType::GutlingGun => [10., 10.],
        GearGunType::ShotGun => [10., 10.],
        GearGunType::RifleCannon => [32., 32.],
        GearGunType::Howitzer => [96., 96.], 
    }.into()}

    /// 初速の計算
    pub fn velocity0(
        &self, 
    ) -> f32 { 
        match self {
            Self::MachineGun => 1920., 
            Self::MachineCannon => 1800., 
            Self::GutlingGun => 1800., 
            Self::ShotGun => 620., 
            Self::RifleCannon => 1800., 
            Self::Howitzer => 480., 
        } 
    }

    /// 拡散の演算
    pub(super) fn diffuse(
        &self, 
    ) -> Option<f32> { 
        Some(match self {
            Self::MachineGun => 5., 
            Self::MachineCannon => 2.5, 
            Self::GutlingGun => 12.8, 
            Self::ShotGun => 30., 
            Self::RifleCannon => 1.25, 
            Self::Howitzer => 0.625, 
        })
    }

    /// 次にシフト
    fn shift_next(&mut self) { *self = match self {
        Self::MachineGun => Self::MachineCannon, 
        Self::MachineCannon => Self::GutlingGun, 
        Self::GutlingGun => Self::ShotGun, 
        Self::ShotGun => Self::RifleCannon, 
        Self::RifleCannon => Self::Howitzer, 
        Self::Howitzer => Self::MachineGun, 
    }; }

    /// 前にシフト
    fn shift_back(&mut self) { *self = match self {
        Self::MachineGun => Self::Howitzer, 
        Self::MachineCannon => Self::MachineGun, 
        Self::GutlingGun => Self::MachineCannon, 
        Self::ShotGun => Self::GutlingGun, 
        Self::RifleCannon => Self::ShotGun, 
        Self::Howitzer => Self::RifleCannon, 
    }; }

    /// クールタイム
    fn cool_time_sec(&self) -> f32 { match self {
        Self::MachineGun => 1. / 45., 
        Self::MachineCannon => 1. / 15., 
        Self::GutlingGun => 1. / 60., 
        Self::ShotGun => 1. / 2., 
        Self::RifleCannon => 1. / 3., 
        Self::Howitzer => 1. / 3., 
    } }

    /// 発射数
    fn shoot_count(&self) -> u32 { match self {
        GearGunType::MachineGun => 1,
        GearGunType::MachineCannon => 1,
        GearGunType::GutlingGun => 3,
        GearGunType::ShotGun => 40,
        GearGunType::RifleCannon => 1,
        GearGunType::Howitzer => 1, 
    } }
}

/// ギアの発射機
pub struct GearGun {
    pub gear_type: GearGunType, 
    cooling_dur: f32, 
}
impl GearGun {
    pub fn new() -> Self { Self {
        gear_type: GearGunType::default(),
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
                super::GearType::Gun(self.gear_type.clone()), 
                1., 
                1., 
                self.gear_type.shoot_count()..=self.gear_type.shoot_count(), 
            );
            self.cooling_dur += self.gear_type.cool_time_sec();
        }
    }
}