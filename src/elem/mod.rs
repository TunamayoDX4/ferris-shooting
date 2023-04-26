use tm_wg_wrapper::prelude::*;
use cycle_measure::CycleMeasure;
use simple2d::{
    entity_holder::{
        EntityHolder, 
        EntityArray, 
    }, 
    types::{
        VisibleField, 
        InstanceGen, 
    }, 
    img_obj::ImgObjInstance, 
};

pub mod physic_body;
pub mod ferris;
pub mod gear;
pub mod enemy;

/// Ferris Shooting の要素
pub struct FSElement {
    ferris: ferris::instance::FerrisInstance, 
    gear: gear::instance::GearInstance, 
    enemies: enemy::instance::EnemyInstance, 
}
impl FSElement {
    pub fn new() -> Self { Self {
        ferris: ferris::instance::FerrisInstance::new(), 
        gear: gear::instance::GearInstance::new(), 
        enemies: enemy::instance::EnemyInstance::new(), 
    } }

    pub fn input_mouse_motion(
        &mut self, 
        motion: nalgebra::Vector2<f32>, 
        varea: &VisibleField, 
    ) {
        self.ferris.input_mouse_motion(varea, motion);
    }

    pub fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) {
        self.ferris.input_mouse_button(button, state);
    }

    pub fn input_key(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        state: winit::event::ElementState, 
    ) {
        self.ferris.input_key(keycode, state);
    }

    pub fn update(
        &mut self, 
        window: &winit::window::Window, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        ferris: &mut simple2d::img_obj::ImgObjRender, 
        aim: &mut simple2d::img_obj::ImgObjRender, 
        gear: &mut simple2d::img_obj::ImgObjRender, 
        enemies: &mut simple2d::img_obj::ImgObjRender, 
    ) {
        self.ferris.update(window, cycle, varea, &mut self.gear, &self.enemies);
        self.gear.update(cycle, varea, &mut self.enemies);
        self.enemies.update(cycle, varea);
        
        self.enemies.render_update(enemies);
        self.ferris.render_update(ferris, aim);
        self.gear.renderer_update(gear);
    }
}