use rand::Rng;

use super::*;
pub mod gun;
pub mod missile;

/// ギアの種類
#[derive(Clone)]
pub enum GearType {
    Gun(gun::GearGunType), 
    Missile(missile::GearMissileType), 
}
impl GearType {
    pub fn weight(&self) -> f32 { match self {
        GearType::Gun(gun) => gun.weight(),
        GearType::Missile(miss) => miss.weight(), 
    } }

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GearType::Gun(gun) => gun.size(),
        GearType::Missile(miss) => miss.size(), 
    } }

    /// 初速の計算
    pub fn velocity0(
        &self, 
        speed_ratio: f32, 
        diffuse_ratio: f32, 
    ) -> f32 {
        (match self {
            GearType::Gun(gun) => gun.velocity0(), 
            GearType::Missile(miss) => miss.velocity0(), 
        }) 
            * speed_ratio 
            * (1. + self.vel_diffuse(diffuse_ratio).unwrap_or(0.))
    }

    /// 拡散の計算
    pub fn angle_diffuse(
        &self, 
        diffuse_ratio: f32, 
    ) -> Option<f32> {
        match self {
            GearType::Gun(gun) => gun.angle_diffuse(),
            GearType::Missile(miss) => miss.angle_diffuse(), 
        }
            .map(|diff| diff * diffuse_ratio * 0.01)
            .map(|diff| crate::RNG.with(
                |r| (**r).borrow_mut().gen_range(-diff..diff)
            ))
    }

    /// 拡散の計算
    pub fn vel_diffuse(
        &self, 
        diffuse_ratio: f32, 
    ) -> Option<f32> {
        match self {
            GearType::Gun(gun) => gun.vel_diffuse(),
            GearType::Missile(miss) => miss.vel_diffuse(), 
        }
            .map(|diff| diff * diffuse_ratio * 0.01)
            .map(|diff| crate::RNG.with(
                |r| (**r).borrow_mut().gen_range(-diff..diff)
            ))
    }

    /// 索敵処理
    pub fn seek(
        &mut self, 
        pbody: &GearPhysicBody, 
        enemies: &EntityArray<enemy::Enemy>, 
    ) {
        if let Self::Missile(ms) = self {
            ms.seek(pbody, enemies)
        }
    }

    /// 追尾処理
    pub fn tracking(
        &mut self, 
        pbody: &mut GearPhysicBody, 
        cycle: &CycleMeasure, 
    ) {
        if let Self::Missile(ms) = self {
            ms.track(pbody, cycle)
        }
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