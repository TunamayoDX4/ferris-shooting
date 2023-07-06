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

pub struct GamePause {
    pub do_exit: bool, 
}
impl GamePause {
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
                s: "\
                [Pause]\n\n\
                Escape pause mode: `Escape` or\n\
                `P` key press moment\n\
                Exit: `Escape` Key press over 1sec\
                ",
                position: [0., 0.],
                rotation: 0., 
                size_ratio: [1., 1.],
                align_v: font_typing::TypeAlignV::Middle,
                align_h: font_typing::TypeAlignH::Center,
                direction: font_typing::TypeDirection::Horizontal,
            }, 
        );
    }
}

