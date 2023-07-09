use tm_wg_wrapper::{
    prelude::nalgebra::{Vector2, Point2}, 
    util::simple2d::{
        entity_holder::EntityArray, 
        img_obj::ImgObjInstance, 
    }
};

use crate::game::ferris::ngear::{
    GearInstance, 
    array::GearIdentMaster, 
};

/// 爆発能力
pub struct ExplodeParam {
    pub tex_rot: Option<std::ops::Range<f32>>, 
    pub frag_count: u32, 
    pub frag_diff: Option<std::ops::Range<i64>>, 
    pub frvel_base: f32, 
    pub frvel_diff: Option<std::ops::Range<f32>>, 
    pub frsiz_base: f32, 
    pub frsiz_diff: Option<std::ops::Range<f32>>, 
    pub ltime_base: f32, 
    pub ltime_diff: Option<std::ops::Range<f32>>, 
    pub damage_r: f32, 
}
impl ExplodeParam {
    pub fn explode(
        self, 
        ident: &mut GearIdentMaster, 
        rng: &mut impl rand::Rng, 
        gears: &mut EntityArray<
            ImgObjInstance, 
            GearInstance, 
        >, 
        position: Point2<f32>, 
        base_vel: Vector2<f32>, 
    ) {
        let fd = self.frag_diff.map(|fd| rng.gen_range(fd))
            .unwrap_or(0);
        let spawn_count = (self.frag_count as i64 + fd) as usize;
        
        for _ in 0..spawn_count {
            let fa_base = rng.gen_range(
                -std::f32::consts::PI..std::f32::consts::PI
            );
            let fb = self.frvel_diff.clone()
                .map(|fd| rng.gen_range(fd))
                .unwrap_or(0.);
            let fv_base = self.frvel_base + fb;
            let fv = Vector2::from([
                fa_base.cos() * fv_base, 
                fa_base.sin() * fv_base, 
            ]);
            let fv = fv + base_vel;
            let fv0 = (fv.x.powi(2) + fv.y.powi(2)).sqrt();
            let fva = f32::atan2(
                fv.y, 
                fv.x
            );
            let fb = self.frsiz_diff.clone()
                .map(|fd| rng.gen_range(fd))
                .unwrap_or(0.);
            let fs_base = self.frsiz_base + fb;
            let fs = [fs_base, fs_base];

            let gb = super::super::GearBody {
                phys: super::super::GearPhys {
                    position,
                    rotation: fva,
                    vel_a: fv0,
                },
                tex_rot_speed: self.tex_rot.clone()
                    .map(|tr| rng.gen_range(tr))
                    .unwrap_or(0.),
                tex_rot: rng.gen_range(
                    -std::f32::consts::PI..std::f32::consts::PI
                ),
                gt: super::super::gtype::GType::Fragment(
                    super::super::gtype::fragment::FragmentGear {
                        life_time: self.ltime_base + self.ltime_diff.clone()
                            .map(|lt| rng.gen_range(lt))
                            .unwrap_or(0.), 
                        size: fs,
                        damage_r: self.damage_r,
                    }
                ),
            };
            let gear = GearInstance {
                ident: ident.issue(),
                gb,
            };
            gears.push(gear);
        }

    }
}