use super::*;

pub mod ferris;
pub mod aim;
pub mod gear;

pub struct FerrisInstances {
    ferris: EntityHolder<ImgObjInstance, ferris::Ferris>, 
    aim: EntityHolder<ImgObjInstance, aim::Aim>, 
    gear: gear::GearInstances, 
}
impl FerrisInstances {
    pub fn new() -> Self { Self {
        ferris: EntityHolder::new(ferris::Ferris::new()), 
        aim: EntityHolder::new(aim::Aim::new()), 
        gear: gear::GearInstances::new(), 
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
    ) {
        self.ferris.manip_mut(|f| f.update(cycle, &mut self.gear));
        self.gear.update(cycle, varea);
        self.aim.manip_mut(|a| a.update(varea));
    }

    pub fn rendering(
        &self, 
        renderer: &mut crate::renderer::FSRenderer, 
    ) {
        renderer.ferris.push_instance(&self.ferris);
        renderer.gear.push_instance(&self.gear.gears); 
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