//! 新しいギアの実装

use tm_wg_wrapper::{
    prelude::*, 
    util::simple2d::{
        InstanceGen, 
        instance::buffer::InstanceArray, 
        img_obj::ImgObjInstance, physic::PhysicBody, entity_holder::EntityHolder, 
    }
};
use crate::game::enemy::enemy::EnemyArray;
pub mod array;
pub mod gtype;
use gtype::GTypeTrait;

/// ギアの物理的な値
pub struct GearPhys {
    pub position: nalgebra::Point2<f32>,  
    pub rotation: f32, 
    pub vel_a: f32, 
}
impl GearPhys {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
    ) {
        let vel = self.vel_a * cycle.dur;
        self.position += nalgebra::Vector2::from([
            vel * self.rotation.cos(), 
            vel * self.rotation.sin(), 
        ]);
    }
}

/// ギアの内部
pub struct GearBody {
    phys: GearPhys, 
    tex_rot_speed: f32, 
    tex_rot: f32, 
    gt: gtype::GType, 
}

/// ギアの実体
pub struct GearInstance {
    ident: array::GearIdent, 
    gb: GearBody, 
}
impl InstanceGen<ImgObjInstance> for GearInstance {
    fn generate(
        &self, 
        instances: &mut InstanceArray<ImgObjInstance>, 
    ) {
        instances.push(ImgObjInstance { 
            position: self.gb.phys.position.into(), 
            size: self.gb.gt.size().into(), 
            rotation: self.gb.tex_rot, 
            tex_coord: [0., 0.], 
            tex_size: [32., 32.], 
            tex_rev: [false, false] 
        })
    }
}
impl PhysicBody for GearInstance {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.gb.phys.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.gb.gt.size()
    }

    fn rotation(&self) -> f32 {
        self.gb.phys.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        [
            self.gb.phys.vel_a * self.gb.phys.rotation.cos(), 
            self.gb.phys.vel_a * self.gb.phys.rotation.sin(), 
        ].into()
    }
}
impl GearInstance {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        aim: &EntityHolder<ImgObjInstance, super::aim::Aim>, 
        enemies: &mut EnemyArray, 
    ) -> bool {
        let res = self.gb.gt.update(
            cycle, 
            varea, 
            &self.ident, 
            &mut self.gb.phys, 
            ferris, 
            aim, 
            enemies
        );
        self.gb.phys.update(cycle);
        self.gb.tex_rot += self.gb.tex_rot_speed * cycle.dur;
        varea.in_visible(
            self.gb.phys.position, 
            self.gb.gt.size()
        ) && res
    }
}

/// ギアの物理演算用ラップ
pub struct GPhysWrap<'a, 'b> {
    gt: &'a gtype::GType, 
    phys: &'b GearPhys, 
}
impl PhysicBody for GPhysWrap<'_, '_> {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.phys.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.gt.size()
    }

    fn rotation(&self) -> f32 {
        self.phys.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        [
            self.phys.vel_a * self.phys.rotation.cos(), 
            self.phys.vel_a * self.phys.rotation.sin()
        ].into()
    }
}

/// ギアの物理演算用ラップ(可変)
pub struct GPhysWrapMut<'a, 'b> {
    gt: &'a gtype::GType, 
    phys: &'b mut GearPhys, 
}
impl PhysicBody for GPhysWrapMut<'_, '_> {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.phys.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.gt.size()
    }

    fn rotation(&self) -> f32 {
        self.phys.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        [
            self.phys.vel_a * self.phys.rotation.cos(), 
            self.phys.vel_a * self.phys.rotation.sin()
        ].into()
    }
}