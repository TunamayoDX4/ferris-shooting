use rand::Rng;
use strum::EnumIter;
use tm_wg_wrapper::util::cycle_measure;

#[derive(Debug, Clone, Default)]
#[derive(EnumIter)]
pub enum GearType {
    #[default]
    MachineGun, 
    MachineCannon, 
    RifleCannon, 
    ShotGun, 
    Fragment{
        size: f32, 
        lifetime: f32, 
    }, 
}
impl GearType {
    pub fn size(&self) -> f32 { match self {
        Self::MachineGun => 12.,
        Self::MachineCannon => 20.,
        Self::RifleCannon => 32.,
        Self::ShotGun => 10.,
        Self::Fragment{size, ..} => *size, 
    } }

    pub fn toggle_forward(&self) -> Self { match self {
        Self::MachineGun => Self::MachineCannon,
        Self::MachineCannon => Self::RifleCannon,
        Self::RifleCannon => Self::ShotGun,
        Self::ShotGun => Self::MachineGun,
        s @ Self::Fragment{..} => s.clone(), 
    } }

    pub fn toggle_backward(&self) -> Self { match self {
        Self::MachineGun => Self::ShotGun,
        Self::MachineCannon => Self::MachineGun,
        Self::RifleCannon => Self::MachineCannon,
        Self::ShotGun => Self::RifleCannon,
        s @ Self::Fragment{..} => s.clone(), 
    } }

    pub fn default_speed(
        &self, 
        speed_ratio: f32, 
        diffusion_ratio: f32, 
    ) -> f32 { 
        speed_ratio * match self {
            Self::MachineGun => 1920.,
            Self::MachineCannon => 1720.,
            Self::RifleCannon => 1680.,
            Self::ShotGun => 720.,
            Self::Fragment{..} => 540., 
        } + crate::RNG.with(|r| (**r).borrow_mut().gen_range({
            let diffusion = match self {
                Self::MachineGun => 120.,
                Self::MachineCannon => 90.,
                Self::RifleCannon => 5.,
                Self::ShotGun => 320.,
                Self::Fragment{..} => 230., 
            } * diffusion_ratio;
            -diffusion..diffusion
        }))
    }

    pub fn default_diffusion(
        &self, 
        diffusion_ratio: f32, 
    ) -> f32 { crate::RNG.with(|r| (**r).borrow_mut().gen_range({
        let diffusion = match self {
            Self::MachineGun => 5., 
            Self::MachineCannon => 3.5, 
            Self::RifleCannon => 0.025, 
            Self::ShotGun => 15., 
            Self::Fragment{..} => 180., 
        };
        -diffusion..diffusion
    })) * (std::f32::consts::PI / 180.) * diffusion_ratio}

    pub fn render_rot(
        &self, default_angle: f32, 
    ) -> f32 { crate::RNG.with(|r| (**r).borrow_mut().gen_range({
        let range = match self {
            Self::MachineGun => 360., 
            Self::MachineCannon => 180., 
            Self::RifleCannon => 0.001, 
            Self::ShotGun => 360., 
            Self::Fragment { .. } => 25., 
        };
        -range..range
    })) * (std::f32::consts::PI / 180.) + default_angle }

    pub fn render_rot_speed(
        &self
    ) -> f32 { crate::RNG.with(|r| (**r).borrow_mut().gen_range({
        let range = match self {
            Self::MachineGun => 360., 
            Self::MachineCannon => 240., 
            Self::RifleCannon => 90., 
            Self::ShotGun => 720., 
            Self::Fragment { .. } => 320., 
        } * (std::f32::consts::PI / 180.);
        -range..range
    }))}

    pub fn spawn_count(&self, spawn_count_ratio: u32) -> u32 { spawn_count_ratio * match self {
        Self::MachineGun => 2,
        Self::MachineCannon => 1,
        Self::RifleCannon => 1,
        Self::ShotGun => 64,
        Self::Fragment { .. } => 1, 
    } }

    pub fn spawn_interval(&self) -> f32 { 1. / match self {
        Self::MachineGun => 60.,
        Self::MachineCannon => 20.,
        Self::RifleCannon => 2.,
        Self::ShotGun => 2.,
        Self::Fragment { .. } => 0., 
    } }

    pub fn update(&mut self, cycle: &cycle_measure::CycleMeasure) { match self {
        Self::Fragment { lifetime, .. } => {
            if lifetime.is_sign_positive() { 
                *lifetime -= cycle.dur; 
            }
        }, 
        _ => {}, 
    }}

    pub fn alive(&self) -> bool { match self {
        Self::Fragment { lifetime, .. } => lifetime.is_sign_positive(), 
        _ => true, 
    }}

    pub fn terminate(
        instance: &super::Gear, 
    ) -> Option<GearTerminateMode> { match instance.gear_type {
        Self::MachineCannon => Some(GearTerminateMode::Explode(
            super::explode::Explode {
                position: instance.position,
                velocity: instance.velocity, 
                power: 2.,
                fragment_count_ratio: 12,
                fragment_size: 10.,
                fragment_lifetime: 0.1,
            }
        )), 
        Self::RifleCannon => {
            Some(GearTerminateMode::Explode(
            super::explode::Explode {
                position: instance.position,
                velocity: instance.velocity, 
                power: 3.5,
                fragment_count_ratio: 32,
                fragment_size: 12.,
                fragment_lifetime: 0.1,
            }
        ))}, 
        _ => None, 
    } }
}

pub enum GearTerminateMode {
    Explode(super::explode::Explode), 
}
impl GearTerminateMode {
    pub fn execute(self, gears: &mut super::simple2d::entity_holder::EntityArray<super::Gear>) {
        match self {
            GearTerminateMode::Explode(exp) => exp.explode(gears),
        }
    }
}