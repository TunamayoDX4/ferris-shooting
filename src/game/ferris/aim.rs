use super::*;

pub struct Aim {
    position: nalgebra::Point2<f32>, 
    visible: bool, 
    state: AimState, 
}
impl Aim {
    pub fn new() -> Self { Self {
        position: [0., 0.].into(), 
        visible: true, 
        state: AimState::Normal, 
    }}

    pub fn input_mouse_motion(
        &mut self, 
        motion: nalgebra::Vector2<f32>, 
    ) {
        self.position += motion
    }

    pub fn update(
        &mut self, 
        varea: &VisibleField, 
    ) {
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
impl InstanceGen<ImgObjInstance> for Aim {
    fn generate(
        &self, 
        instances: &mut simple2d::instance::buffer::InstanceArray<ImgObjInstance>
    ) {
        instances.push(ImgObjInstance {
            position: self.position.into(),
            size: self.state.size().into(),
            rotation: 0.,
            tex_coord: self.state.tex_coord(),
            tex_size: self.state.tex_size(),
            tex_rev: [false, false],
        });

        match self.state {
            AimState::Tracking { 
                vec, 
                .. 
            } => {
                instances.push(ImgObjInstance {
                    position: (self.position + vec).into(),
                    size: [32., 32.],
                    rotation: 0.,
                    tex_coord: [128., 128.],
                    tex_size: [32., 32.],
                    tex_rev: [false, false],
                })
            }, 
            _ => {}, 
        }
    }
}

#[derive(Clone)]
pub enum AimState {
    /// 通常状態
    Normal, 

    /// 追尾中
    Tracking {
        enemy: enemy::enemy::EnemyRef, 
        vec: nalgebra::Vector2<f32>, 
    }, 

    /// 追尾OK
    TrackReady, 

    /// 機能不全
    Failure, 
}
impl AimState {
    pub fn size(&self) -> nalgebra::Vector2<f32> { match self {
        _ => [64., 64.].into(), 
    }}

    pub fn tex_coord(&self) -> [f32; 2] { match self {
        AimState::Normal => [0., 0.],
        AimState::Tracking{..} => [64., 0.],
        AimState::TrackReady => [0., 64.],
        AimState::Failure => [64., 64.],
    }}

    pub fn tex_size(&self) -> [f32; 2] { match self {
        _ => [64., 64.], 
    }}
}