//! ゲーム中のポーズ画面の実装
//! ゲーム画面からPキーもしくはEscキーを押下することで遷移、
//! そこから再度PキーもしくはEscキーを押下すると離脱。
//! 
//! ポーズ画面中ではマウスのグラブが解除される。

use crate::renderer::FSRenderer;

pub struct GamePause {}
impl GamePause {

    pub fn rendering(
        &self, 
        renderer: &mut FSRenderer, 
    ) {
    }
    
}