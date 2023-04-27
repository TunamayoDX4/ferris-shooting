use super::*;
use winit::event::{
    VirtualKeyCode, 
    ElementState, 
};

mod body;
pub mod instance;
pub mod control;
pub mod aim;

pub struct Ferris {
    pub position: nalgebra::Point2<f32>, 
    pub rotation: f32, 
    pub velocity: nalgebra::Vector2<f32>, 
    pub control: control::FerrisControl, 
    pub gear: gear::gear_type::gun::GearGun, 
    pub gear_ms: gear::gear_type::missile::GearMissile, 
}
impl Ferris {
    fn moving(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
    ) {
        const SPEED: f32 = 240.;
        let prev_position = self.position;
        self.position += nalgebra::Vector2::from({
            let fw = match self.control.forward.get_mode() {
                crate::util::RevMode::Forward => [
                    ((std::f32::consts::PI * 0.5) + self.rotation).cos(), 
                    ((std::f32::consts::PI * 0.5) + self.rotation).sin(), 
                ], 
                crate::util::RevMode::Backward => [
                    ((std::f32::consts::PI * 1.5) + self.rotation).cos(), 
                    ((std::f32::consts::PI * 1.5) + self.rotation).sin(), 
                ], 
                _ => [0., 0.], 
            };
            let rg = match self.control.right.get_mode() {
                crate::util::RevMode::Forward => [
                    (0.0f32 + self.rotation).cos(), 
                    (0.0f32 + self.rotation).sin(), 
                ], 
                crate::util::RevMode::Backward => [
                    (std::f32::consts::PI + self.rotation).cos(), 
                    (std::f32::consts::PI + self.rotation).sin(), 
                ], 
                _ => [0., 0.], 
            };
            std::array::from_fn::<_, 2, _>(|i| {
                fw[i] + rg[i]
            })
        }) * SPEED * cycle.dur;

        // 画面外処理
        let varea = varea.visible_area();
        if self.position.x < varea[0].x {
            self.position.x = varea[0].x 
        } else if varea[1].x < self.position.x {
            self.position.x = varea[1].x
        }
        if self.position.y < varea[0].y {
            self.position.y = varea[0].y 
        } else if varea[1].y < self.position.y {
            self.position.y = varea[1].y
        }
        self.velocity = (self.position - prev_position) * cycle.cps;
    }

    /// 回転処理
    fn rotating(
        &mut self, 
        cycle: &CycleMeasure, 
        aim: &EntityHolder<aim::Aim>, 
    ) {
        const ROTATE_SPEED: f32 = 360. * (std::f32::consts::PI / 180.);
        if let Some(angle) = aim.manip(|a| {
            let tgv = if let Some((
                _, _, sa
            )) = a.aiming_target.as_ref() {
                sa.position
            } else {
                a.position
            } - self.position;
            f32::atan2(tgv.y, tgv.x) - std::f32::consts::PI * 0.5
        }) {
            let diff = (
                (angle - self.rotation) + std::f32::consts::PI
            ).rem_euclid(std::f32::consts::PI * 2.).abs() - std::f32::consts::PI;
            let rotate_speed = ROTATE_SPEED * cycle.dur;
            if diff <= -rotate_speed {
                self.rotation -= rotate_speed;
            } else if rotate_speed * cycle.dur <= diff {
                self.rotation += rotate_speed;
            } else {
                self.rotation = angle;
            }
        } else {
            self.rotation += match self.control.rot_left.get_mode() {
                crate::util::RevMode::Forward => 1.,
                crate::util::RevMode::Backward => -1.,
                _ => 0., 
            } * ROTATE_SPEED * cycle.dur;
        }
        self.rotation = (self.rotation + std::f32::consts::PI).rem_euclid(
            std::f32::consts::PI * 2.
        ).abs() - std::f32::consts::PI;
    }

    /// 銃の更新処理
    fn gun(
        &mut self, 
        cycle: &CycleMeasure, 
        gear: &mut gear::instance::GearInstance, 
    ) {
        if self.control.change_gunmode.get_trig_count() == 1 {
            self.gear.shift_mode(&self.control.change_gunmode.get_mode())
        }
        self.gear.update(
            cycle, 
            self.position, 
            self.rotation + std::f32::consts::PI * 0.5, 
            self.velocity, 
            self.control.shoot.is_triggered() || self.control.shoot_mb.is_triggered(), 
            gear
        );
    }

    /// ミサイルの更新処理
    fn missile(
        &mut self, 
        cycle: &CycleMeasure, 
        gear: &mut gear::instance::GearInstance, 
    ) {
        self.gear_ms.update(
            cycle, 
            self.position, 
            self.rotation + std::f32::consts::PI * 0.5, 
            self.velocity, 
            self.control.shoot_ms.is_triggered(), 
            gear
        );
    }

    /// 更新処理
    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        gear: &mut gear::instance::GearInstance, 
        aim: &EntityHolder<aim::Aim>, 
    ) {
        self.control.update();
        self.moving(cycle, varea);
        self.rotating(cycle, aim);
        self.gun(cycle, gear);
        self.missile(cycle, gear);
    }
}