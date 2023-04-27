use super::*;

pub struct FerrisInstance {
    ferris: EntityHolder<Ferris>, 
    aim: EntityHolder<aim::Aim>, 
}
impl FerrisInstance {
    pub fn new(
    ) -> Self {
        let ferris = Ferris {
            position: [0., 0.].into(),
            rotation: 0.,
            velocity: [0., 0.].into(),
            control: Default::default(),
            gear: gear::gear_type::gun::GearGun::new(), 
            gear_ms: gear::gear_type::missile::GearMissile::new(), 
        };
        let aim = aim::Aim {
            prev_position: [0., 0.].into(), 
            position: [0., 0.].into(),
            velocity: [0., 0.].into(),
            push_autoaim: Default::default(), 
            aiming_target: None, 
        };
        Self {
            ferris: EntityHolder::new(ferris), 
            aim: EntityHolder::new(aim), 
        }
    }

    pub fn input_mouse_motion(
        &mut self, 
        varea: &VisibleField, 
        motion: nalgebra::Vector2<f32>, 
    ) {
        if self.ferris.get().map_or(
            false, 
            |f| f.control.mouse_track.is_latch_on()
        ) {
            self.aim.manip_mut(|a| a.moving(varea, motion));
        }
    }

    pub fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) {
        self.ferris.manip_mut(|f| {
            f.mouse_input(button, state)
        });
        match button {
            MouseButton::Right => self.aim.manip_mut(|a| {
                a.push_autoaim.trigger(state);
            }).unwrap_or(()), 
            _ => {}, 
        }
    }

    pub fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) {
        if let Some(pos) = self.ferris.manip_mut(|f| {
            f.key_input(keycode, state);
            if f.control.mouse_track.latch_on_count() == 0 {
                Some(f.position)
            } else {
                None
            }
        }).flatten() {
            self.aim = EntityHolder::new(aim::Aim {
                prev_position: [0., 0.].into(),
                position: pos,
                velocity: [0., 0.].into(),
                push_autoaim: Default::default(), 
                aiming_target: None, 
            })
        }
    }

    pub fn update(
        &mut self, 
        _window: &winit::window::Window, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        gear: &mut gear::instance::GearInstance, 
        enemies: &enemy::instance::EnemyInstance, 
    ) {
        self.ferris.manip(|f| {
            self.aim.retain(|a| {
                a.update(f, cycle, &enemies.enemies);
                self.ferris.manip(|f| {
                    f.control.mouse_track.is_latch_on()
                }).unwrap_or(false)
            });
        });
        self.ferris.retain(|f| {
            f.update(cycle, varea, gear, &self.aim);
            true
        });
    }

    pub fn render_update(
        &self, 
        ferris: &mut simple2d::img_obj::ImgObjRender, 
        aim: &mut simple2d::img_obj::ImgObjRender, 
    ) {
        self.ferris.render_update(ferris);
        if let None = self.aim.manip(|a| {
            if let Some((_, _, sa)) = a.aiming_target.as_ref() {
                aim.update_instances([
                    aim::AimRO::Aim(a), 
                    aim::AimRO::SubAim(sa), 
                ].iter())
            } else {
                aim.update_instances([
                    aim::AimRO::Aim(a)
                ].iter())
            }
        }) {
            aim.update_instances(
                [None::<aim::AimRO>].iter().filter_map(|f| f.as_ref())
            )
        }
    }
}