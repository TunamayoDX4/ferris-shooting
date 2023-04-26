use super::winit;
pub struct RevCtrl {
    triggering: bool, 
    count: u32, 
    mode: RevMode, 
}
impl Default for RevCtrl {
    fn default() -> Self {
        Self {
            triggering: false,  
            count: 0, 
            mode: RevMode::Brake, 
        }
    }
}
impl RevCtrl {
    pub fn input(
        &mut self, 
        mode: RevMode, 
        state: winit::event::ElementState, 
    ) { match (mode, state) {
        (
            RevMode::Forward, winit::event::ElementState::Pressed
        ) => {
            self.triggering = true;
            self.mode = RevMode::Forward;
        }, 
        (
            RevMode::Forward, winit::event::ElementState::Released
        ) if self.mode == RevMode::Forward => {
            self.triggering = false;
            self.mode = RevMode::Brake;
        },
        (
            RevMode::Backward, winit::event::ElementState::Pressed
        ) => {
            self.triggering = true;
            self.mode = RevMode::Backward;
        }, 
        (
            RevMode::Backward, winit::event::ElementState::Released
        ) if self.mode == RevMode::Backward => {
            self.triggering = false;
            self.mode = RevMode::Brake;
        },
        _ => {}, 
    }}
    pub fn update(&mut self) { if self.triggering {
        self.count += 1
    } else {
        self.count = 0
    }}
    pub fn get_mode(&self) -> RevMode { self.mode }
    pub fn is_triggered(&self) -> bool { self.triggering }
    pub fn get_trig_count(&self) -> u32 { self.count }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevMode {
    Forward, 
    Brake, 
    Backward, 
}

pub struct Trigger {
    triggering: bool, 
    count: u32, 
}
impl Default for Trigger {
    fn default() -> Self {
        Self { 
            triggering: false, 
            count: 0, 
        }
    }
}
impl Trigger {
    pub fn trigger(&mut self, state: winit::event::ElementState) { match state {
        winit::event::ElementState::Pressed => self.triggering = true, 
        winit::event::ElementState::Released => self.triggering = false, 
    }}
    pub fn update(&mut self) {if self.triggering { 
        self.count += 1 
    } else { 
        self.count = 0 
    }}
    pub fn is_triggered(&self) -> bool { self.triggering }
    pub fn get_trig_count(&self) -> u32 { self.count }
}