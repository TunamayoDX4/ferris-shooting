//! ゲーム中のポーズ画面の実装
//! ゲーム画面からPキーもしくはEscキーを押下することで遷移、
//! そこから再度PキーもしくはEscキーを押下すると離脱。
//! 
//! ポーズ画面中ではマウスのグラブが解除される。

use tm_wg_wrapper::{
    util::simple2d::{
        font_typing, 
    }, 
    prelude::Window
};

use crate::renderer::FSRenderer;

pub struct GameOver {
    pub do_exit: bool, 
}
impl GameOver {
    pub fn spawn(
        window: &Window, 
    ) -> Result<Self, Box<dyn std::error::Error>> {
        window.set_cursor_grab(
            tm_wg_wrapper::prelude::winit::window::CursorGrabMode::None
        )?;
        window.set_cursor_visible(true);
        Ok(Self { 
            do_exit: false, 
        })
    }

    pub fn pop(
        &self, 
        window: &Window, 
    ) -> Result<(), Box<dyn std::error::Error>> {
        window.set_cursor_grab(
            tm_wg_wrapper::prelude::winit::window::CursorGrabMode::None
        )?;
        window.set_cursor_visible(false);
        Ok(())
    }

    pub fn rendering(
        &self, 
        renderer: &mut FSRenderer, 
    ) {
        renderer.font.draw_type(
            &font_typing::TypeParam {
                s: "[!!Game Over!!]",
                position: [0., 0.],
                rotation: 0., 
                size_ratio: [3., 3.],
                align_v: font_typing::TypeAlignV::Middle,
                align_h: font_typing::TypeAlignH::Center,
                direction: font_typing::TypeDirection::Horizontal,
            }, 
        );
    }
}
