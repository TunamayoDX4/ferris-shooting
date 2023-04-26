use rand::Rng;

use super::*;
pub mod gun;
pub mod missile;

/// ギアの種類
#[derive(Clone)]
pub enum GearType {
    Gun(gun::GearGunType), 
}
impl GearType {
    pub fn weight(&self) -> f32 { match self {
        GearType::Gun(gun) => gun.weight(),
    } }

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GearType::Gun(gun) => gun.size(),
    } }

    /// 初速の計算
    pub fn velocity0(
        &self, 
        speed_ratio: f32, 
        diffuse_ratio: f32, 
    ) -> f32 {
        (match self {
            GearType::Gun(gun) => gun.velocity0(), 
        }) 
            * speed_ratio 
            * (1. + self.diffuse(diffuse_ratio).unwrap_or(0.))
    }

    /// 拡散の計算
    pub fn diffuse(
        &self, 
        diffuse_ratio: f32, 
    ) -> Option<f32> {
        match self {
            GearType::Gun(gun) => gun.diffuse(),
        }
            .map(|diff| diff * diffuse_ratio * 0.01)
            .map(|diff| crate::RNG.with(
                |r| (**r).borrow_mut().gen_range(-diff..diff)
            ))
    }
}

pub mod attack {
    use super::*;

    impl GearType {
        pub fn attack_effect(&self) -> Option<GearAttackEffect> {
            None
        }
    }

    /// 敵に着弾した時のエフェクト
    pub enum GearAttackEffect {
    }
}