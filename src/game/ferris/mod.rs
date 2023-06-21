use super::*;

pub mod ferris;
pub mod aim;
//pub mod gear;
pub mod phys;
pub mod ngear;
use ngear::gtype::GTypeTrait;

pub struct FerrisInstances {
    ferris: EntityHolder<ImgObjInstance, ferris::Ferris>, 
    aim: EntityHolder<ImgObjInstance, aim::Aim>, 
    gear2: ngear::array::GearInstances, 
}
impl FerrisInstances {
    pub fn new() -> Self { Self {
        ferris: EntityHolder::new(ferris::Ferris::new()), 
        aim: EntityHolder::new(aim::Aim::new()), 
        gear2: ngear::array::GearInstances::new(), 
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        enemies: &mut enemy::enemy::EnemyArray, 
    ) {
        self.ferris.manip_mut(|f| f.update(
            cycle, 
            varea, 
            &mut self.gear2, 
            self.aim.get()
        ));
        self.gear2.update(
            cycle, 
            varea, 
            self.ferris.get().map(|f| &f.body), 
            &self.aim, 
            enemies
        );
        if let Some(ferris) = self.ferris.get() {
            self.aim.manip_mut(|a| a.update (
                varea, 
                ferris, 
                enemies, 
                ferris.control.auto_aim.get_trig_count() == 1, 
                ferris.gg2.gt.shoot_shell().vel_default(), 
            ));
        }
    }

    pub fn rendering(
        &self, 
        renderer: &mut crate::renderer::FSRenderer, 
    ) {
        renderer.ferris.push_instance(&self.ferris);
        self.gear2.rendering(renderer);
        renderer.aim.push_instance(&self.aim);
    }

    pub fn input_key(&mut self, keycode: VirtualKeyCode, state: ElementState) {
        self.ferris.manip_mut(|f| f.control.input_key(
            keycode, 
            state
        ));
    }

    pub fn input_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        self.ferris.manip_mut(|f| f.control.input_mouse_button(
            button, 
            state
        ));
    }

    pub fn input_mouse_motion(&mut self, motion: nalgebra::Vector2<f32>) {
        self.ferris.manip_mut(|f| 
            f.control.input_mouse_motion(motion)
        );
        self.aim.manip_mut(|a| 
            a.input_mouse_motion(motion)
        );
    }
}