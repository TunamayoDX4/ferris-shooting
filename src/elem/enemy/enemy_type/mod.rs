use rand::Rng;

use super::*;
use super::instance::EnemyIdentGen;

pub mod formation;

#[derive(Clone)]
pub enum EnemyType {
    UBehavior, 
    NullPtr, 
    DataRace, 
    DangPtr, 
}
impl EnemyType {

    /// 体力
    pub fn health(&self) -> u32 { match self {
        EnemyType::UBehavior => 2,
        EnemyType::NullPtr => 3,
        EnemyType::DataRace => 12,
        EnemyType::DangPtr => 30,
    } }

    /// 初速
    pub fn vel_zero(&self) -> f32 { match self {
        EnemyType::UBehavior => 240.,
        EnemyType::NullPtr => 480.,
        EnemyType::DataRace => 320.,
        EnemyType::DangPtr => 360.,
    } }

    /// 描画オブジェクトの回転速度
    pub fn render_rot_speed(&self, r: &mut impl Rng) -> f32 { match self {
        EnemyType::UBehavior => None, 
        EnemyType::NullPtr => None, 
        EnemyType::DataRace => Some(180.0..360.0f32), 
        EnemyType::DangPtr => Some(360.0..720.0f32)
    }.map(|v| 
        r.gen_range(v) * if r.gen_bool(0.5) { 1.0 } else { -1.0 }
    ).map_or(0., |v| v * (std::f32::consts::PI / 180.)) }

    /// デフォルトの描画角度
    pub fn default_render_rot(&self, r: &mut impl Rng) -> f32 { match self {
        EnemyType::UBehavior => None, 
        EnemyType::NullPtr => None, 
        EnemyType::DataRace => Some(-180.0..180.0f32), 
        EnemyType::DangPtr => Some(-180.0..180.0f32), 
    }.map(|v|
        r.gen_range(v)
    ).map_or(0., |v| v * (std::f32::consts::PI / 180.)) }

    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        EnemyType::UBehavior => [64., 64.],
        EnemyType::NullPtr => [64., 64.],
        EnemyType::DataRace => [64., 64.],
        EnemyType::DangPtr => [64., 64.],
    }.into()}

    pub fn tex_coord(&self) -> [f32; 2] { match self {
        EnemyType::UBehavior => [0., 0.],
        EnemyType::NullPtr => [64., 0.],
        EnemyType::DataRace => [2. * 64., 0.],
        EnemyType::DangPtr => [3. * 64., 0.],
    } }

    pub fn tex_size(&self) -> [f32; 2] { match self {
        _ => [64., 64.], 
    } }
}