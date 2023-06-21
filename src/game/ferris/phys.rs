use tm_wg_wrapper::prelude::*;

/// Ferrisの物理的な値のパック
pub struct FerrisPhysVal {
    pub position: nalgebra::Point2<f32>, 
    pub velocity: nalgebra::Vector2<f32>, 
    pub rotation: f32, 
}