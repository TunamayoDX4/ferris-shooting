//! ギアの種類データの実装

use tm_wg_wrapper::{
    prelude::*, 
    util::{
        cycle_measure, 
        simple2d::{types::VisibleField, entity_holder::EntityHolder, img_obj::ImgObjInstance}, 
    }, 
};

use crate::game::enemy::enemy::EnemyArray;

pub mod gun;
pub mod missile;

/// ギアの種類データ
pub enum GType {
    /// ガン・ギア
    /// 砲タイプ。単純な徹甲弾や榴弾
    GunShot(gun::GunGearType), 

    /// ミサイル・ギア
    /// ミサイルタイプ。追尾性のギア
    Missile(missile::MissileGearType), 

    /// エフェクト・ギア
    /// なんかしらのエフェクト
    Effect, 
}
impl GTypeTrait for GType {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GType::GunShot(gs) => gs.angle_diff(),
        GType::Missile(gm) => gm.angle_diff(),
        GType::Effect => todo!(),
    }}

    fn vel_default(&self) -> f32 { match self {
        GType::GunShot(gs) => gs.vel_default(),
        GType::Missile(gm) => gm.vel_default(),
        GType::Effect => todo!(),
    }}

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GType::GunShot(gs) => gs.vel_diff(),
        GType::Missile(gm) => gm.vel_diff(),
        GType::Effect => todo!(),
    }}

    fn size(&self) -> nalgebra::Vector2<f32> { match self {
        GType::GunShot(gs) => gs.size(),
        GType::Missile(gm) => gm.size(),
        GType::Effect => todo!(),
    }}

    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>> { match self {
        GType::GunShot(gs) => gs.tex_rot_diff(),
        GType::Missile(gm) => gm.tex_rot_diff(),
        GType::Effect => todo!(),
    }}

    fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &VisibleField, 
        ident: &super::array::GearIdent, 
        phys: &mut super::GearPhys, 
        ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        aim: &EntityHolder<
            ImgObjInstance, super::super::aim::Aim, 
        >, 
        enemies: &mut EnemyArray, 
    ) -> bool { match self {
        GType::GunShot(gs) => gs.update(
            cycle, 
            varea, 
            ident, 
            phys, 
            ferris, 
            aim, 
            enemies
        ),
        GType::Missile(gm) => gm.update(
            cycle, 
            varea, 
            ident, 
            phys, 
            ferris, 
            aim, 
            enemies
        ),
        GType::Effect => todo!(),
    }}
}

/// ギア種類特有の実装
pub trait GTypeTrait {
    /// 角度の拡散
    fn angle_diff(&self) -> Option<std::ops::Range<f32>>;
    
    /// 角度の計算
    fn angle_calc(
        &self, 
        rng: &mut impl rand::Rng, 
        angle: f32, 
    ) -> f32 {
        angle
        + self.angle_diff().map_or(
            0., 
            |dr| rng.gen_range(dr)
        )
    }

    /// 標準的な初速
    fn vel_default(&self) -> f32;

    /// 初速拡散
    fn vel_diff(&self) -> Option<std::ops::Range<f32>>;
    
    /// 初速の計算
    fn vel_calc(
        &self, 
        rng: &mut impl rand::Rng, 
        position: nalgebra::Point2<f32>, 
        angle: f32, 
        base_vel: nalgebra::Vector2<f32>, 
    ) -> super::GearPhys {
        let vel0 = self.vel_default()
            + self.vel_diff().map_or(
                0., 
                |dr| rng.gen_range(dr)
            );
        let angle = self.angle_calc(rng, angle);
        let vel = nalgebra::Vector2::new(
            vel0 * angle.cos() + base_vel.x, 
            vel0 * angle.sin() + base_vel.y, 
        );

        // 初速の計算
        let vel_a = f32::sqrt(
            vel.x.powi(2) + vel.y.powi(2)
        );
        
        // 角度の計算
        let rotation = f32::atan2(
            vel.y, vel.x
        );
        super::GearPhys {
            position, 
            vel_a, 
            rotation, 
        }
    }

    /// 弾丸の大きさ
    fn size(&self) -> nalgebra::Vector2<f32>;

    /// テクスチャの回転速度
    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>>;

    /// テクスチャ回転速度の計算
    fn calc_tex_rot(
        &self, 
        rng: &mut impl rand::Rng, 
    ) -> f32 {
        self.tex_rot_diff()
            .map_or(
                0., 
            |tr| rng.gen_range(tr)
        )
    }

    /// 更新処理
    fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &VisibleField, 
        ident: &super::array::GearIdent, 
        phys: &mut super::GearPhys, 
        ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        aim: &EntityHolder<
            ImgObjInstance, super::super::aim::Aim, 
        >, 
        enemies: &mut EnemyArray, 
    ) -> bool;
}