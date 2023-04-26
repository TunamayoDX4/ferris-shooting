use super::*;

impl Ferris {
    pub(super) fn motion(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        area: &simple2d::types::VisibleField, 
    ) {
        let position_from = self.position;
        self.position += (
            match self.control.left.get_mode() {
                input::RevMode::Forward => -1.,
                input::RevMode::Backward => 1.,
                _ => 0., 
            } * nalgebra::Vector2::new(
                self.speed * self.rotation.cos(), 
                self.speed * self.rotation.sin()
            ) + match self.control.forward.get_mode() {
                input::RevMode::Forward => 1., 
                input::RevMode::Backward => -1., 
                _ => 0., 
            } * nalgebra::Vector2::new(
                self.speed * (self.rotation + std::f32::consts::PI / 2.).cos(), 
                self.speed * (self.rotation + std::f32::consts::PI / 2.).sin()
            )
        ) * cycle.dur;
        
        let area = area.visible_area();
        if self.position.x < area[0].x {
            self.position.x = area[0].x
        } else if area[1].x < self.position.x {
            self.position.x = area[1].x
        }
        if self.position.y < area[0].y {
            self.position.y = area[0].y
        } else if area[1].y < self.position.y {
            self.position.y = area[1].y
        }

        // 旋回
        self.rotation += match self.control.turn_left.get_mode() {
            input::RevMode::Forward => self.rot_speed * (std::f32::consts::PI / 180.), 
            input::RevMode::Backward => -self.rot_speed * (std::f32::consts::PI / 180.), 
            _ => 0.,
        } * cycle.dur;

        // 速度の計算
        self.velocity = self.position - position_from;
    }
}