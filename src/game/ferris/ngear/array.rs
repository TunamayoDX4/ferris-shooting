use tm_wg_wrapper::util::{simple2d::{
    physic::PhysicBody, 
    entity_holder, 
    img_obj::ImgObjInstance, self, 
}, cycle_measure};

/// ギアの識別子生成構造
#[derive(Debug)]
pub struct GearIdentMaster(u64);
impl GearIdentMaster {
    /// 識別子の発行
    pub fn issue(&mut self) -> GearIdent {
        let r = GearIdent(self.0);
        self.0 = self.0.checked_add(1).unwrap_or(0);
        r
    }
}

/// ギアの識別子
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GearIdent(u64);

/// ギアの参照子
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearRef {
    pub index: usize, 
    pub ident: GearIdent, 
}

/// ギアを格納する配列
pub struct GearInstances {
    ident: GearIdentMaster, 
    gears: entity_holder::EntityArray<
        ImgObjInstance, 
        super::GearInstance, 
    >, 
    gcomm: super::gcomm::GCommQueue, 
}
impl GearInstances {
    pub fn new() -> Self { Self {
        ident: GearIdentMaster(0),
        gears: entity_holder::EntityArray::new([]),
        gcomm: super::gcomm::GCommQueue::new(), 
    }}

    pub fn push_gb(&mut self, gb: super::GearBody) -> GearRef {
        let ident = self.ident.issue();
        let gear = super::GearInstance {
            ident: ident.clone(),
            gb,
        };
        GearRef { 
            index: self.gears.push(gear), 
            ident, 
        }
    }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        ferris: Option<&crate::game::ferris::ferris::FerrisBody>, 
        aim: &entity_holder::EntityHolder<
            ImgObjInstance, crate::game::ferris::aim::Aim, 
        >, 
        enemies: &mut crate::game::enemy::enemy::EnemyArray, 
    ) {
        self.gcomm.execute(
            &mut self.ident, 
            &mut self.gears, 
            ferris, 
            aim.get(), 
            enemies, 
        );
        self.gears.retain(|_, gear| gear.update(
            cycle, 
            varea, 
            ferris, 
            aim, 
            enemies, 
            &mut self.gcomm, 
        ));
    }

    pub fn rendering(
        &self, 
        renderer: &mut crate::renderer::FSRenderer, 
    ) {
        renderer.gear.push_instance(&self.gears)
    }
}

/// ギアを生成するためのスポナー
pub trait GearSpawner<PB: PhysicBody> {
    fn spawn(
        &mut self, 
        base: &PB, 
        gears: &mut GearInstances, 
    );
}