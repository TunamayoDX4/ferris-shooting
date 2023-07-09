use tm_wg_wrapper::{
    util::simple2d::{
        entity_holder::EntityArray, 
        img_obj::ImgObjInstance, 
    }, 
    prelude::nalgebra::{Vector2, Point2}
};

use super::array::GearIdentMaster;
pub mod explode;

/// ギアのインスタンス特殊操作用コマンド
pub enum GComm {
    Explode{
        param: explode::ExplodeParam, 
        position: Point2<f32>, 
        base_vel: Vector2<f32>, 
    }, 
}
impl GComm {
    pub fn execute(
        self, 
        ident: &mut GearIdentMaster, 
        gears: &mut EntityArray<
            ImgObjInstance, 
            super::GearInstance, 
        >, 
        _ferris: Option<&super::super::ferris::FerrisBody>, 
        _aim: Option<&super::super::aim::Aim>, 
        _enemies: &super::super::super::enemy::enemy::EnemyArray, 
    ) { match self {
        GComm::Explode {
            param, 
            position, 
            base_vel, 
        } => crate::RNG.with(
            |r| param.explode(
                ident, 
                &mut *r.borrow_mut(), 
                gears, 
                position, 
                base_vel
            )
        ),
    }}
}

/// ギアコマンドのキュー
pub struct GCommQueue (Vec<Option<GComm>>);
impl GCommQueue {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn push(&mut self, gcomm: GComm) { self.0.push(Some(gcomm)) }

    pub fn execute(
        &mut self, 
        ident: &mut GearIdentMaster, 
        gears: &mut EntityArray<
            ImgObjInstance, 
            super::GearInstance, 
        >, 
        ferris: Option<&super::super::ferris::FerrisBody>, 
        aim: Option<&super::super::aim::Aim>, 
        enemies: &super::super::super::enemy::enemy::EnemyArray, 
    ) {
        self.0.iter_mut()
            .filter_map(|gc| gc.take())
            .for_each(|gc| gc.execute(
                ident, 
                gears, 
                ferris, 
                aim, 
                enemies
            ));
        self.0.clear();
    }
}