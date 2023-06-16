//! ゲーム中のポーズ画面の実装
//! ゲーム画面からPキーもしくはEscキーを押下することで遷移、
//! そこから再度PキーもしくはEscキーを押下すると離脱。
//! 
//! ポーズ画面中ではマウスのグラブが解除される。

use tm_wg_wrapper::{util::simple2d::img_obj::ImgObjInstance, prelude::Window};

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
        Ok(Self { do_exit: false })
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
        renderer.indicator.push_instance(
            &ImgObjInstance {
                position: [0., 0.],
                size: [128., 32.],
                rotation: 0.,
                tex_coord: [0., 0.],
                tex_size: [128., 32.],
                tex_rev: [false, false],
            }
        )
    }
}