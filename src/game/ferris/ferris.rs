use super::*;

pub struct Ferris {
    pub control: Control, 
    position: nalgebra::Point2<f32>, 
    rotation: f32, 
    velocity: nalgebra::Vector2<f32>, 
    pub gg: gear::GearGun, 
}
impl physic::PhysicBody for Ferris {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        [64., 64.].into()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}
impl InstanceGen<ImgObjInstance> for Ferris {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance { 
            position: self.position.into(), 
            size: [64., 64.], 
            rotation: self.rotation, 
            tex_coord: [0., 0.], 
            tex_size: [64., 64.], 
            tex_rev: [false, false], 
        })
    }
}
impl Ferris {
    pub fn new() -> Self { Self {
        control: Control::default(),
        position: [0., 0.].into(),
        rotation: 0.,
        velocity: [0., 0.].into(),
        gg: gear::GearGun::new(gear::GearType::MachineGun), 
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &VisibleField, 
        gears: &mut gear::GearInstances, 
        aim: Option<&super::aim::Aim>, 
    ) {
        self.control.update();
        let v = (match self.control.mov_fwd.get_mode() {
            RevMode::Forward => [0., 320.].into(), 
            RevMode::Backward => [0., -320.].into(), 
            _ => nalgebra::Vector2::new(0., 0.), 
        } + match self.control.mov_right.get_mode() {
            RevMode::Forward => [320., 0.].into(), 
            RevMode::Backward => [-320., 0.].into(), 
            _ => nalgebra::Vector2::new(0., 0.), 
        });
        self.velocity = nalgebra::Vector2::new(
            v.x * self.rotation.cos() - v.y * self.rotation.sin(), 
            v.x * self.rotation.sin() + v.y * self.rotation.cos(), 
        );
        self.position += self.velocity * cycle.dur;
        self.rotation += match self.control.rot_left.get_mode() {
            RevMode::Forward => 180., 
            RevMode::Backward => -180., 
            _ => 0., 
        } * (std::f32::consts::PI / 180.) * cycle.dur;

        if self.control.shoot_kb.is_triggered() || self.control.shoot_mb.is_triggered() {
            self.gg.shoot(
                self.position, 
                self.velocity, 
                self.rotation + std::f32::consts::PI * 0.5, 
                gears, 
                None, 
            );
        }
        if self.control.sg_ch.get_trig_count() == 1 { match self.control.sg_ch.get_mode() {
            RevMode::Forward => self.gg.shift_next(),
            RevMode::Backward => self.gg.shift_back(),
            _ => {}, 
        }}

        if let Some(aim) = aim {
            let angle = {
                let distance = self.position - (aim.pbody.position + if let aim::AimState::Tracking { 
                    vec, 
                    .. 
                } = aim.state { vec } else { [0., 0.].into() });
                f32::atan2(distance.y, distance.x)
            };

            let angle_diff = (
                (angle - (self.rotation - std::f32::consts::PI * 0.5)) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;

            if angle_diff < (-120. * (std::f32::consts::PI / 180.)) * cycle.dur {
                self.rotation += -120. * (std::f32::consts::PI / 180.) * cycle.dur;
            } else if (120. * (std::f32::consts::PI / 180.)) * cycle.dur < angle_diff {
                self.rotation += 120. * (std::f32::consts::PI / 180.) * cycle.dur;
            } else {
                self.rotation = angle + std::f32::consts::PI * 0.5;
            }
        }

        let va = varea.visible_area();
        if self.position.x < va[0].x {
            self.position.x = va[0].x
        } else if va[1].x < self.position.x {
            self.position.x = va[1].x
        };

        if self.position.y < va[0].y {
            self.position.y = va[0].y
        } else if va[1].y < self.position.y {
            self.position.y = va[1].y
        };

        self.gg.update(cycle);
    }
}

#[derive(Default)]
pub struct Control {
    pub mov_fwd: RevCtrl, 
    pub mov_right: RevCtrl, 
    pub rot_left: RevCtrl, 
    pub shoot_kb: Trigger, 
    pub shoot_mb: Trigger, 
    pub mouse_pointer: nalgebra::Point2<f32>, 
    /// 撃つギアの切り替え
    pub sg_ch: RevCtrl, 
    pub auto_aim: Trigger, 

}
impl Control {
    pub fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) { match keycode {
        VirtualKeyCode::W => self.mov_fwd.input(RevMode::Forward, state), 
        VirtualKeyCode::S => self.mov_fwd.input(RevMode::Backward, state), 
        VirtualKeyCode::A => self.mov_right.input(RevMode::Backward, state), 
        VirtualKeyCode::D => self.mov_right.input(RevMode::Forward, state), 
        VirtualKeyCode::Q => self.rot_left.input(RevMode::Forward, state), 
        VirtualKeyCode::E => self.rot_left.input(RevMode::Backward, state), 
        VirtualKeyCode::Z => self.sg_ch.input(RevMode::Backward, state), 
        VirtualKeyCode::X => {}, 
        VirtualKeyCode::C => self.sg_ch.input(RevMode::Forward, state), 
        VirtualKeyCode::R => {}, 
        VirtualKeyCode::F => {}, 
        VirtualKeyCode::V => {}, 
        VirtualKeyCode::Space => self.shoot_kb.trigger(state), 
        _ => {}, 
    }}

    pub fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) { match button {
        MouseButton::Left => self.shoot_mb.trigger(state),
        MouseButton::Right => self.auto_aim.trigger(state),
        MouseButton::Middle => {},
        MouseButton::Other(_) => {},
    }}

    pub fn input_mouse_motion(
        &mut self, 
        motion: nalgebra::Vector2<f32>, 
    ) {
        self.mouse_pointer += motion;
    }

    pub fn update(
        &mut self, 
    ) {
        self.mov_fwd.update();
        self.mov_right.update();
        self.rot_left.update();
        self.shoot_kb.update();
        self.shoot_mb.update();
        self.sg_ch.update();
        self.auto_aim.update();
    }
}