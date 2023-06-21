use rand::Rng;
use tm_wg_wrapper::util::simple2d::{entity_holder::EntityRefMut, physic::{aabb, PhysicBody}};

use crate::game::enemy::enemy::Enemy;

use super::*;

pub mod gear_type;
pub mod missile;

pub struct GearIdentMaster(u64);
impl GearIdentMaster {
    pub fn issue(&mut self) -> GearIdent {
        let r = GearIdent(self.0);
        self.0 = self.0.checked_add(1).unwrap_or(0);
        r
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearIdent(u64);

pub struct GearRef {
    pub index: usize, 
    pub ident: GearIdent, 
}

#[derive(Clone)]
pub enum GearType {
    MachineGun, 
    MachineCannon, 
    Gutling, 
    ShotGun, 
    RifleCannon, 
}
impl GearType {
    pub fn shift_next(&mut self) { *self = match self {
        Self::MachineGun => Self::MachineCannon,
        Self::MachineCannon => Self::Gutling, 
        Self::Gutling => Self::ShotGun, 
        Self::ShotGun => Self::RifleCannon, 
        Self::RifleCannon => Self::MachineGun, 
    }}

    pub fn shift_back(&mut self) { *self = match self {
        Self::MachineGun => Self::RifleCannon,
        Self::MachineCannon => Self::MachineGun, 
        Self::Gutling => Self::MachineCannon, 
        Self::ShotGun => Self::Gutling, 
        Self::RifleCannon => Self::ShotGun, 
    }}

    /// 大きさを得る。
    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        Self::MachineGun => [12., 12.].into(),
        Self::MachineCannon => [18., 18.].into(), 
        Self::Gutling => [12., 12.].into(), 
        Self::ShotGun => [8., 8.].into(), 
        Self::RifleCannon => [48., 48.].into(), 
    }}

    /// 初速を得る。
    pub fn vel_0(&self) -> f32 { match self {
        Self::MachineGun => 1200.,
        Self::MachineCannon => 1800., 
        Self::Gutling => 1350., 
        Self::ShotGun => 850., 
        Self::RifleCannon => 1580., 
    }}

    /// 初速拡散を得る。
    pub fn vel_0_diff(&self) -> f32 { match self {
        Self::MachineGun => 240.,
        Self::MachineCannon => 120., 
        Self::Gutling => 380., 
        Self::ShotGun => 300., 
        Self::RifleCannon => 40., 
    }}

    /// 初速の計算
    pub fn vel_0_calc(&self) -> f32 {
        self.vel_0() + crate::RNG.with(|r| 
            (**r).borrow_mut().gen_range(-1.0..1.0)
        ) * self.vel_0_diff()
    }

    /// 散布角を得る。
    pub fn angle(&self) -> f32 { match self {
        Self::MachineGun => 5.,
        Self::MachineCannon => 3.75, 
        Self::Gutling => 12., 
        Self::ShotGun => 25., 
        Self::RifleCannon => 1.2, 
    }}

    /// 散布角の計算
    pub fn angle_diff_calc(&self) -> f32 {
        crate::RNG.with(
            |r| (**r).borrow_mut().gen_range(-1.0..1.0)
        ) * self.angle()
    }

    /// 一度に射出する数
    pub fn shot_count(&self) -> u32 { match self {
        Self::MachineGun => 1,
        Self::MachineCannon => 1, 
        Self::Gutling => 3, 
        Self::ShotGun => 50, 
        Self::RifleCannon => 1, 
    } }

    /// 射出サイクルの時間
    pub fn cycle_dur(&self) -> f32 { match self {
        Self::MachineGun => 1. / 45.,
        Self::MachineCannon => 1. / 8., 
        Self::Gutling => 1. / 40., 
        Self::ShotGun => 1. / 1.5, 
        Self::RifleCannon => 1. / 1.5, 
    } }

    /// ダメージ値
    pub fn damage(&self) -> f32 { match self {
        GearType::MachineGun => 1.,
        GearType::MachineCannon => 3.,
        GearType::Gutling => 0.8,
        GearType::ShotGun => 0.75,
        GearType::RifleCannon => 32.,
    } }

    /// ギアのスポーン
    pub fn spawn(
        &self, 
        ident: GearIdent, 
        position: nalgebra::Point2<f32>, 
        velocity: nalgebra::Vector2<f32>, 
        rotation: f32, 
    ) -> Gear {
        let vel0 = self.vel_0_calc();
        let angle = rotation + self.angle_diff_calc() * (std::f32::consts::PI / 180.);
        let vel0 = nalgebra::Vector2::new(
            vel0 * angle.cos(), 
            vel0 * angle.sin() 
        ) + velocity;
        let rotation = f32::atan2(vel0.y, vel0.x);
        let vel = f32::sqrt(vel0.x.powi(2) + vel0.y.powi(2));
        Gear {
            ident,
            position,
            vel,
            rotation,
            velocity,
            render_rot: crate::RNG.with(|r|
                (**r).borrow_mut().gen_range(-1.0..1.0)
            ) * std::f32::consts::PI,
            render_rot_speed: crate::RNG.with(|r|
                (**r).borrow_mut().gen_range(-3.0..3.0)
            ) * std::f32::consts::PI,
            gtype: self.clone(),
        }
    }
}

pub struct Gear {
    ident: GearIdent, 
    position: nalgebra::Point2<f32>, 
    vel: f32, 
    rotation: f32, 
    velocity: nalgebra::Vector2<f32>, 
    render_rot: f32, 
    render_rot_speed: f32, 
    gtype: GearType, 
}
impl InstanceGen<ImgObjInstance> for Gear {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance {
            position: self.position.into(),
            size: self.gtype.size().into(),
            rotation: self.rotation,
            tex_coord: [0., 0.],
            tex_size: [32., 32.],
            tex_rev: [false, false],
        })
    }
}
impl PhysicBody for Gear {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.gtype.size()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}
impl Gear {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        enemies: &mut enemy::enemy::EnemyArray, 
    ) -> bool {
        self.velocity = nalgebra::Vector2::new(
            self.vel * self.rotation.cos(), 
            self.vel * self.rotation.sin(), 
        );
        self.position += self.velocity * cycle.dur;
        self.render_rot += self.render_rot_speed * cycle.dur;

        // 接触した敵の中から最も近い敵を選ぶ。
        // そもそも居なかったらギアの単純な存続処理を、
        // 居たら最も近い敵にダメージを与え、自分は死ぬ
        if enemies.enemies.iter_mut()
            .map(|EntityRefMut{ entity, .. }| entity)
            .filter(|entity| aabb(*entity, self))
            .map(|entity| {
                let dist = entity.position() - self.position;
                ((dist.x.powi(2) + dist.y.powi(2)).sqrt(), entity)
            })
            .fold(
                None::<(f32, &mut Enemy)>, 
                |
                    init, 
                    tg, 
                | match init {
                    None => Some(tg), 
                    Some(e) if tg.0 < e.0 => { Some(tg) }, 
                    a @ Some(_) => a, 
                }
            )
            .map(|(_, target)| target)
            .map_or(true, |target| { 
                target.give_damage(self.gtype.damage()); 
                false 
            })
        { varea.in_visible(self.position, self.gtype.size()) }
        else { false }
    }
}

pub struct GearInstances {
    ident: GearIdentMaster, 
    pub(super) gears: EntityArray<ImgObjInstance, Gear>, 
}
impl GearInstances {
    pub fn new() -> Self { Self {
        ident: GearIdentMaster(0), 
        gears: EntityArray::new([]), 
    }}

    pub fn spawn(
        &mut self, 
        gtype: &GearType, 
        position: nalgebra::Point2<f32>, 
        velocity: nalgebra::Vector2<f32>, 
        rotation: f32, 
    ) -> GearRef {
        let ident = self.ident.issue();
        let g = gtype.spawn(
            ident.clone(), 
            position, 
            velocity, 
            rotation
        );
        GearRef {
            index: self.gears.push(g),
            ident,
        }
    }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        enemies: &mut enemy::enemy::EnemyArray, 
    ) {
        self.gears.retain(|_, gear| gear.update(
            cycle, 
            varea, 
            enemies, 
        ));
    }

    pub fn rendering(&self, renderer: &mut crate::renderer::FSRenderer) {
        renderer.gear.push_instance(&self.gears);
    }

    pub fn get(&self, gear_ref: GearRef) -> Option<&Gear> {
        self.gears.get(gear_ref.index)
            .filter(|g| g.ident == gear_ref.ident)
    }

    pub fn get_mut(&mut self, gear_ref: GearRef) -> Option<&mut Gear> {
        self.gears.get_mut(gear_ref.index)
            .filter(|g| g.ident == gear_ref.ident)
    }

    pub fn manip<R>(&self, gear_ref: GearRef, f: impl FnOnce(&Gear) -> R) -> Option<R> {
        self.get(gear_ref).map(|g| f(g))
    }

    pub fn manip_mut<R>(&mut self, gear_ref: GearRef, f: impl FnOnce(&mut Gear) -> R) -> Option<R> {
        self.get_mut(gear_ref).map(|g| f(g))
    }
    
}

pub struct GearGun {
    pub gtype: GearType, 
    cool_time: f32, 
}
impl GearGun {
    pub fn new(gtype: GearType) -> Self { Self {
        gtype, 
        cool_time: 0., 
    }}
    
    pub fn shoot(
        &mut self, 
        position: nalgebra::Point2<f32>, 
        velocity: nalgebra::Vector2<f32>, 
        rotation: f32, 
        gears: &mut GearInstances, 
        mut gref_table: Option<&mut Vec<Option<GearRef>>>, 
    ) { if self.cool_time == 0.0 {
        for _ in 0..self.gtype.shot_count() {
            let gr = gears.spawn(&self.gtype, position, velocity, rotation);
            if let Some(gt) = gref_table.as_mut() { gt.push(Some(gr)) }
        }
        self.cool_time += self.gtype.cycle_dur();
    }}

    pub fn update(&mut self, cycle: &cycle_measure::CycleMeasure) {
        if 0. < self.cool_time { self.cool_time -= cycle.dur; }
        else { self.cool_time = 0. }
    }

    pub fn shift_next(&mut self) {
        self.gtype.shift_next()
    }

    pub fn shift_back(&mut self) {
        self.gtype.shift_back()
    }
}