//! 破片タイプのギア

use tm_wg_wrapper::{
    prelude::*, 
    util::simple2d::{
        entity_holder::EntityRefMut, 
        physic::aabb, 
    }
};
use crate::game::{
    ferris::ngear::GPhysWrap, 
    enemy::enemy::Enemy, 
};
use super::GTypeTrait;

/// 破片タイプギアのデータ
#[derive(Clone)]
pub struct FragmentGear {
    pub life_time: f32, 
    pub size: [f32; 2], 
    pub damage_r: f32, 
}
impl GTypeTrait for FragmentGear {
    fn angle_diff(&self) -> Option<std::ops::Range<f32>> {
        None
    }

    fn vel_default(&self) -> f32 {
        0.
    }

    fn vel_diff(&self) -> Option<std::ops::Range<f32>> {
        None
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.size.into()
    }

    fn tex_rot_diff(&self) -> Option<std::ops::Range<f32>> {
        None
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
            crate::game::ferris::aim::Aim, 
        >, 
        enemies: &mut crate::game::enemy::enemy::EnemyArray, 
        gcomm: &mut super::super::gcomm::GCommQueue, 
    ) -> bool {
        self.life_time -= cycle.dur;
        for enemy in enemies.enemies.iter_mut() {
            if aabb(&GPhysWrap {
                gt: &super::GType::Fragment(self.clone()),
                phys,
            }, enemy.entity) {
                let damage = {
                    let base = (
                        self.size[0]
                        + self.size[1]
                    ).sqrt() * 0.1;
                    base * self.damage_r
                };
                enemy.entity.give_damage(damage);
                return false
            }
        }
        0. < self.life_time 
        && varea.in_visible(phys.position, self.size)
    }
}