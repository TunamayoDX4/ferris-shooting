use std::cell::RefCell;
use rand::SeedableRng;
use rand_pcg::Pcg64;

type ULazy<T> = once_cell::unsync::Lazy<T>;
thread_local! {
    pub static RNG: ULazy<RefCell<Pcg64>> = ULazy::new(||
        RefCell::new(Pcg64::from_entropy())
    );
}

use tm_wg_wrapper::prelude::*;

pub mod log;
pub mod renderer;
pub mod game;

pub struct FSFrameParam {
    cycle_measure: cycle_measure::CycleMeasure, 
}

pub enum FSPopV {
}

pub enum FSFrame {
    Game(game::Game), 
}
impl scene_frame::Scene for FSFrame {
    type Rdr = renderer::FSRenderer;
    type Fpr = FSFrameParam;
    type PopV = FSPopV;

    fn window_builder() -> winit::window::WindowBuilder {
        todo!()
    }

    fn init_proc(
        window: &Window, 
        gfx: &GfxCtx, 
        sfx: &SfxCtx, 
    ) -> Result<Self::Fpr, Box<dyn std::error::Error>> {
        todo!()
    }

    fn render_init(
        gfx: &GfxCtx, 
    ) -> Result<Self::Rdr, Box<dyn std::error::Error>> {
        todo!()
    }

    fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) {
        todo!()
    }

    fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) {
        todo!()
    }

    fn input_mouse_motion(
        &mut self, 
        delta: (f64, f64), 
    ) {
        todo!()
    }

    fn input_mouse_scroll(
        &mut self, 
        delta: MouseScrollDelta, 
    ) {
        todo!()
    }

    fn window_resizing(
        &mut self, 
        size: winit::dpi::PhysicalSize<u32>, 
    ) {
        todo!()
    }

    fn require_process(
        &self, 
        depth: usize, 
        is_top: bool, 
    ) -> bool {
        todo!()
    }

    fn process(
        &mut self, 
        depth: usize, 
        is_top: bool, 
        frame_param: &mut Self::Fpr, 
        window: &Window, 
        gfx: &GfxCtx, 
        sfx: &SfxCtx, 
    ) -> Result<
        scene_frame::SceneProcOp<Self>, 
        Box<dyn std::error::Error>
    > {
        todo!()
    }

    fn require_rendering(
        &self, 
        depth: usize, 
        is_top: bool, 
    ) -> bool {
        todo!()
    }

    fn rendering(
        &self, 
        depth: usize, 
        is_top: bool, 
        render: &mut Self::Rdr, 
    ) {
        todo!()
    }

    fn pop(self) -> Self::PopV {
        todo!()
    }

    fn return_foreground(&mut self, popv: Self::PopV) {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*pollster::block_on(Context::<_, scene::SceneFrame<FSScene>>::new(
        |fpr, rdr| {
            [FSScene::Title(false)].into_iter()
        }
    ))?.run().1?;*/
    Ok(())
}
