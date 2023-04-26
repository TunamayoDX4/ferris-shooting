use tm_wg_wrapper::prelude::*;
use simple2d::{
    types::InstanceGen, 
    img_obj::ImgObjInstance, 
};

pub mod input;

#[derive(Default)]
pub struct UserControl {
    pub forward: input::RevCtrl, 
    pub left: input::RevCtrl, 
    pub turn_left: input::RevCtrl, 
    pub shot: input::Trigger, 
    pub gun_toggle: input::RevCtrl, 
}
impl UserControl {
    pub fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        state: winit::event::ElementState, 
    ) { match keycode {
        winit::event::VirtualKeyCode::W => self.forward.input(
            input::RevMode::Forward, state, 
        ), 
        winit::event::VirtualKeyCode::S => self.forward.input(
            input::RevMode::Backward, state, 
        ), 
        winit::event::VirtualKeyCode::A => self.left.input(
            input::RevMode::Forward, state, 
        ), 
        winit::event::VirtualKeyCode::D => self.left.input(
            input::RevMode::Backward, state, 
        ), 
        winit::event::VirtualKeyCode::Q => self.turn_left.input(
            input::RevMode::Forward, state, 
        ), 
        winit::event::VirtualKeyCode::E => self.turn_left.input(
            input::RevMode::Backward, state, 
        ), 
        winit::event::VirtualKeyCode::Space => self.shot.trigger(state), 
        winit::event::VirtualKeyCode::Z => self.gun_toggle.input(
            input::RevMode::Backward, state
        ), 
        winit::event::VirtualKeyCode::C => self.gun_toggle.input(
            input::RevMode::Forward, state
        ), 
        _ => {}, 
    }}

    pub fn update(&mut self) {
        self.forward.update();
        self.left.update();
        self.turn_left.update();
        self.shot.update();
        self.gun_toggle.update();
    }
}

pub mod motion;

pub struct Ferris {
    control: UserControl, 
    position: nalgebra::Point2<f32>, 
    velocity: nalgebra::Vector2<f32>, 
    rotation: f32, 
    speed: f32, 
    rot_speed: f32, 
    size: [f32; 2], 
    gun: GearGun, 
}
impl Ferris {
    pub fn new() -> Self { Ferris {
        control: Default::default(),
        position: [0., 0.].into(),
        velocity: [0., 0.].into(), 
        rotation: 0.,
        speed: 240., 
        rot_speed: 360. * 0.5, 
        size: [64., 64.], 
        gun: GearGun::new(), 
    }}
    pub fn key_input(
        &mut self, 
        keycode: winit::event::VirtualKeyCode, 
        state: winit::event::ElementState, 
    ) { self.control.key_input(keycode, state) }
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        area: &simple2d::types::VisibleField, 
        gears: &mut super::gear::Gears, 
    ) {
        self.control.update();
        self.motion(cycle, area);
        self.gun.update(cycle);
        if self.control.gun_toggle.get_trig_count() == 1 { match self.control.gun_toggle.get_mode() {
            input::RevMode::Forward => self.gun.toggle_forward(),
            input::RevMode::Brake => {},
            input::RevMode::Backward => self.gun.toggle_backward(),
        } }
        if self.control.shot.is_triggered() { self.gun.shot(
            self.position, 
            self.rotation, 
            gears, 
        ) }
    }
}
impl InstanceGen<ImgObjInstance> for Ferris {
    fn generate(&self) -> ImgObjInstance { ImgObjInstance { 
        position: self.position.into(), 
        size: self.size, 
        rotation: self.rotation, 
        tex_coord: [0., 0.], 
        tex_size: [64., 64.], 
        tex_rev: [false, false],  
    }}
}


/// ギア発射機
pub struct GearGun {
    gear_type: super::gear::types::GearType, 
    interval: f32, 
}
impl GearGun {
    pub fn new() -> Self { Self {
        gear_type: Default::default(), 
        interval: 0., 
    }}

    pub fn shot(
        &mut self, 
        position: nalgebra::Point2<f32>, 
        rotation: f32, 
        gears: &mut super::gear::Gears, 
    ) { if self.interval == 0. {
        self.interval = self.gear_type.spawn_interval();
        gears.spawn(&super::gear::GearSpawner {
            position,
            velocity0: None, 
            rotation: rotation + std::f32::consts::FRAC_PI_2,
            speed_ratio: 1.,
            diffusion_ratio: 1.,
            gear_type: self.gear_type.clone(),
            spawn_count_ratio: 1, 
        })
    }}

    pub fn update(&mut self, cycle: &cycle_measure::CycleMeasure) {
        if 0. != self.interval {
            self.interval -= cycle.dur;
            if self.interval < 0. { self.interval = 0. }
        }
    }

    pub fn toggle_forward(&mut self) {
        self.gear_type = self.gear_type.toggle_forward()
    }
    pub fn toggle_backward(&mut self) {
        self.gear_type = self.gear_type.toggle_backward()
    }
}