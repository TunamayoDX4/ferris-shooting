use super::*;

#[derive(Default)]
pub struct FerrisControl {
    pub forward: crate::util::RevCtrl, 
    pub right: crate::util::RevCtrl, 
    pub rot_left: crate::util::RevCtrl, 
    pub rot_angle_moving: crate::util::Latch, 
    pub mouse_track: crate::util::Latch, 
    pub shoot: crate::util::Trigger, 
    pub shoot_mb: crate::util::Trigger, 
    pub shoot_ms: crate::util::Trigger, 
    pub change_gunmode: crate::util::RevCtrl, 
}
impl FerrisControl {
    pub fn update(&mut self) {
        self.forward.update();
        self.right.update();
        self.rot_left.update();
        self.rot_angle_moving.update();
        self.mouse_track.update();
        self.shoot.update();
        self.change_gunmode.update();
        self.shoot_mb.update();
        self.shoot_ms.update();
    }
}

impl Ferris {
    pub fn key_input(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) { match keycode {
        VirtualKeyCode::W => self.control.forward.input(
            crate::util::RevMode::Forward, state
        ), 
        VirtualKeyCode::S => self.control.forward.input(
            crate::util::RevMode::Backward, state
        ), 
        VirtualKeyCode::D => self.control.right.input(
            crate::util::RevMode::Forward, state
        ), 
        VirtualKeyCode::A => self.control.right.input(
            crate::util::RevMode::Backward, state
        ), 
        VirtualKeyCode::Q => self.control.rot_left.input(
            crate::util::RevMode::Forward, state
        ), 
        VirtualKeyCode::E => self.control.rot_left.input(
            crate::util::RevMode::Backward, state
        ), 
        VirtualKeyCode::Space => self.control.shoot.trigger(state), 
        VirtualKeyCode::Z => self.control.change_gunmode.input(
            crate::util::RevMode::Backward, state
        ), 
        VirtualKeyCode::C => self.control.change_gunmode.input(
            crate::util::RevMode::Forward, state
        ), 
        VirtualKeyCode::X => self.control.mouse_track.trigger(state), 
        _ => {}, 
    }}

    pub fn mouse_input(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) { match button {
        MouseButton::Left => self.control.shoot_mb.trigger(state),
        MouseButton::Middle => self.control.shoot_ms.trigger(state), 
        _ => {}, 
    }}
}