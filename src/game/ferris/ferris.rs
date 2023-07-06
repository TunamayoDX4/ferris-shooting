use super::*;

pub struct FerrisBody {
    pub position: nalgebra::Point2<f32>, 
    pub rotation: f32, 
    pub velocity: nalgebra::Vector2<f32>, 
    pub size: nalgebra::Vector2<f32>, 
}
impl physic::PhysicBody for FerrisBody {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.size
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}
impl FerrisBody {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &VisibleField, 
    ) {
        self.position += self.velocity * cycle.dur;
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
    }
}

pub struct Ferris {
    pub control: Control, 
    pub body: FerrisBody, 
    pub gg2: ngear::gtype::gun::GearGun, 
    pub ml: ngear::gtype::missile::MissileLauncher, 
    pub rotate_speed: f32, 
}
impl physic::PhysicBody for Ferris {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.body.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.body.size
    }

    fn rotation(&self) -> f32 {
        self.body.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.body.velocity
    }
}
impl InstanceGen<ImgObjInstance> for Ferris {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance { 
            position: self.body.position.into(), 
            size: self.body.size.into(), 
            rotation: self.body.rotation, 
            tex_coord: [0., 0.], 
            tex_size: [64., 64.], 
            tex_rev: [false, false], 
        })
    }
}
impl Ferris {
    pub fn new() -> Self { Self {
        control: Control::default(),
        body: FerrisBody { 
            position: [0., -240.].into(), 
            rotation: 0., 
            velocity: [0., 0.].into(), 
            size: [64., 64.].into() 
        }, 
        gg2: ngear::gtype::gun::GearGun::default(), 
        ml: ngear::gtype::missile::MissileLauncher::default(), 
        rotate_speed: 360., 
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &VisibleField, 
        gears2: &mut ngear::array::GearInstances, 
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
        self.body.velocity = nalgebra::Vector2::new(
            v.x * self.body.rotation.cos() - v.y * self.body.rotation.sin(), 
            v.x * self.body.rotation.sin() + v.y * self.body.rotation.cos(), 
        );
        self.body.rotation += match self.control.rot_left.get_mode() {
            RevMode::Forward => 180., 
            RevMode::Backward => -180., 
            _ => 0., 
        } * (std::f32::consts::PI / 180.) * cycle.dur;

        if self.control.shoot_kb.is_triggered() || self.control.shoot_mb.is_triggered() {
            /*self.gg.shoot(
                self.body.position, 
                self.body.velocity, 
                self.body.rotation + std::f32::consts::PI * 0.5, 
                gears, 
                None, 
            );*/
            self.gg2.shoot(
                &self.body, 
                gears2
            );
        }
        if self.control.sg_ch.get_trig_count() == 1 { match self.control.sg_ch.get_mode() {
            RevMode::Forward => self.gg2.gt.toggle(
                crate::game::ferris::ngear::gtype::gun::GTToggle::Forward
            ),
            RevMode::Backward => self.gg2.gt.toggle(
                crate::game::ferris::ngear::gtype::gun::GTToggle::Backward
            ),
            _ => {}, 
        }}

        if self.control.shoot_ms.is_triggered() {
            self.ml.shoot(
                &self.body, 
                aim, 
                gears2
            )
        }

        if let Some(aim) = aim { if self.control.auto_track.is_latch_on() {
            let angle = {
                let distance = self.body.position - (aim.pbody.position + if let aim::AimState::Tracking { 
                    vec, 
                    .. 
                } = aim.state { vec } else { [0., 0.].into() });
                f32::atan2(distance.y, distance.x)
            };

            let angle_diff = (
                (angle - (self.body.rotation - std::f32::consts::PI * 0.5)) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;

            if angle_diff < (-self.rotate_speed * (std::f32::consts::PI / 180.)) * cycle.dur {
                self.body.rotation += -self.rotate_speed * (std::f32::consts::PI / 180.) * cycle.dur;
            } else if (self.rotate_speed * (std::f32::consts::PI / 180.)) * cycle.dur < angle_diff {
                self.body.rotation += self.rotate_speed * (std::f32::consts::PI / 180.) * cycle.dur;
            } else {
                self.body.rotation = angle + std::f32::consts::PI * 0.5;
            }
        }}

        self.body.update(cycle, varea);
        //self.gg.update(cycle);
        self.gg2.update(cycle);
        self.ml.update(cycle);
    }
}

#[derive(Default)]
pub struct Control {
    pub mov_fwd: RevCtrl, 
    pub mov_right: RevCtrl, 
    pub rot_left: RevCtrl, 
    pub shoot_kb: Trigger, 
    pub shoot_mb: Trigger, 
    pub shoot_ms: Trigger, 
    pub mouse_pointer: nalgebra::Point2<f32>, 
    /// 撃つギアの切り替え
    pub sg_ch: RevCtrl, 
    pub auto_aim: Trigger, 
    pub auto_track: Latch, 

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
        VirtualKeyCode::X => self.auto_track.trigger(state), 
        VirtualKeyCode::C => self.sg_ch.input(RevMode::Forward, state), 
        VirtualKeyCode::R => {}, 
        VirtualKeyCode::F => self.shoot_ms.trigger(state), 
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
        self.shoot_ms.update();
        self.sg_ch.update();
        self.auto_aim.update();
        self.auto_track.update();
    }
}