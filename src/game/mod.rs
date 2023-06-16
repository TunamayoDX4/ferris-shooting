use tm_wg_wrapper::{
    prelude::*, 
    util::control::{
        Latch, 
        RevCtrl, 
        RevMode, 
        Trigger, TrigTimeWrap, 
    }, 
    util::simple2d::{
        entity_holder::{
            EntityArray, 
            EntityHolder, 
        }, 
        InstanceGen, 
        img_obj::ImgObjInstance, 
        physic, 
        types::VisibleField, 
    }, 
};
pub mod ferris;
pub mod enemy;

pub struct Game {
    input_esc: TrigTimeWrap<Trigger>,  
    is_top_prev: bool, 
    elements: Elements, 
}
impl Game {
    pub fn new() -> Self { Self {
        input_esc: TrigTimeWrap { ctrl: Trigger::default(), input_dur: 0. },  
        is_top_prev: false, 
        elements: Elements::new(),
    }}

    pub fn update(
        &mut self, 
        is_top: bool, 
        window: &winit::window::Window, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
    ) -> Result<
        scene_frame::SceneProcOp<super::FSFrame>, 
        Box<dyn std::error::Error>
    > {
        let diff = self.is_top_prev != is_top;
        self.is_top_prev = is_top;
        if is_top {
            if diff {
                window.set_cursor_grab(
                    winit::window::CursorGrabMode::Confined
                )?;
                window.set_cursor_visible(false);
            }
            self.elements.update(cycle, varea);
        }
        self.input_esc.update(cycle);

        if 0.5 < self.input_esc.input_dur() {
            Ok(scene_frame::SceneProcOp::StkCtl(
                scene_frame::SceneStackCtrlOp::Exit
            ))
        } else {
            Ok(scene_frame::SceneProcOp::Nop)
        }
    }

    pub fn rendering(&self, renderer: &mut crate::renderer::FSRenderer) {
        self.elements.rendering(renderer)
    }

    pub fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) {
        match keycode {
            VirtualKeyCode::Escape => self.input_esc.ctrl.trigger(state), 
            _ => {}, 
        }
        self.elements.ferris.input_key(keycode, state)
    }

    pub fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) {
        self.elements.ferris.input_mouse_button(button, state)
    }

    pub fn input_mouse_motion(
        &mut self, 
        motion: impl Into<nalgebra::Vector2<f32>>, 
    ) {
        self.elements.ferris.input_mouse_motion(motion.into())
    }
}

pub struct Elements {
    ferris: ferris::FerrisInstances, 
    enemies: enemy::EnemyInstances, 
}
impl Elements {
    pub fn new() -> Self { Self {
        ferris: ferris::FerrisInstances::new(), 
        enemies: enemy::EnemyInstances::new(), 
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
    ) {
        self.ferris.update(cycle, varea, &mut self.enemies.enemy);
        self.enemies.update(cycle, varea);
    }

    pub fn rendering(&self, renderer: &mut crate::renderer::FSRenderer) {
        self.ferris.rendering(renderer);
        self.enemies.rendering(renderer);
    }
}
