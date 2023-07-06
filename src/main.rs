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
pub mod game_pause;
pub mod game_over;

pub struct FSFrameParam {
    cycle_measure: cycle_measure::CycleMeasure, 
    visible_area: Option<simple2d::types::VisibleField>, 
}
impl scene_frame::FrameParam for FSFrameParam {
    type Rdr = crate::renderer::FSRenderer;

    fn update(
        &mut self, 
        renderer: &Self::Rdr, 
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cycle_measure.update();
        self.visible_area = Some(simple2d::types::VisibleField::new(
            &renderer.camera.camera
        ));
        Ok(())
    }
}

pub struct FSPopV {
}

pub enum FSFrame {
    Game(game::Game), 
    GamePause(game_pause::GamePause), 
    GameOver(game_over::GameOver), 
}
impl scene_frame::Scene for FSFrame {
    type Rdr = renderer::FSRenderer;
    type Fpr = FSFrameParam;
    type PopV = FSPopV;

    fn window_builder() -> winit::window::WindowBuilder {
        winit::window::WindowBuilder::new()
            .with_active(true)
            .with_resizable(false)
            .with_inner_size(winit::dpi::PhysicalSize::new(640, 960))
            .with_title("Ferris shooting")
    }

    fn init_proc(
        window: &Window, 
        gfx: &GfxCtx, 
        sfx: &SfxCtx, 
    ) -> Result<Self::Fpr, Box<dyn std::error::Error>> {
        Ok(FSFrameParam {
            cycle_measure: cycle_measure::CycleMeasure::new(),
            visible_area: None,
        })
    }

    fn render_init(
        gfx: &GfxCtx, 
    ) -> Result<Self::Rdr, Box<dyn std::error::Error>> {
        renderer::FSRenderer::new(gfx)
    }

    fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) { match self {
        Self::Game(g) => g.input_key(keycode, state), 
        Self::GamePause(gp) => match keycode {
            VirtualKeyCode::Escape 
            | VirtualKeyCode::P 
            if state == ElementState::Pressed => {
                gp.do_exit = true;
            }, 
            _ => {}, 
        }, 
        Self::GameOver(gp) => match keycode {
            VirtualKeyCode::Escape
            | VirtualKeyCode::P
            if state == ElementState::Pressed => {
                gp.do_exit = true;
            }, 
            _ => {}, 
        }, 
    }}

    fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) { match self {
        FSFrame::Game(g) => g.input_mouse_button(button, state),
        FSFrame::GamePause(_) => {}, 
        FSFrame::GameOver(_) => {}, 
    }}

    fn input_mouse_motion(
        &mut self, 
        delta: (f64, f64), 
    ) { match self {
        FSFrame::Game(g) => g.input_mouse_motion([delta.0 as f32, -delta.1 as f32]),
        FSFrame::GamePause(_) => {},
        FSFrame::GameOver(_) => {}, 
    }}

    fn input_mouse_scroll(
        &mut self, 
        delta: MouseScrollDelta, 
    ) {
    }

    fn window_resizing(
        &mut self, 
        size: winit::dpi::PhysicalSize<u32>, 
    ) {
    }

    fn process(
        &mut self, 
        depth: usize, 
        is_top: bool, 
        renderer: &Self::Rdr, 
        frame_param: &mut Self::Fpr, 
        window: &Window, 
        gfx: &GfxCtx, 
        sfx: &SfxCtx, 
    ) -> Result<
        scene_frame::SceneProcOp<Self>, 
        Box<dyn std::error::Error>
    > { match self {
        FSFrame::Game(g) => {
            frame_param.visible_area = Some(frame_param.visible_area.take().unwrap_or(
                simple2d::types::VisibleField::new(&renderer.camera.camera)
            ));
            g.update(
                is_top, 
                window, 
                &frame_param.cycle_measure, 
                frame_param.visible_area.as_ref().unwrap(), 
            )
        },
        FSFrame::GamePause(gp) => if gp.do_exit {
            gp.pop(window)?;
            Ok(scene_frame::SceneProcOp::StkCtl(
                scene_frame::SceneStackCtrlOp::Pop
            ))
        } else {
            Ok(scene_frame::SceneProcOp::Nop)
        }, 
        FSFrame::GameOver(gp) => if gp.do_exit {
            gp.pop(window)?;
            Ok(scene_frame::SceneProcOp::StkCtl(
                scene_frame::SceneStackCtrlOp::PopAll(
                    FSFrame::Game(game::Game::new())
                )
            ))
        } else {
            Ok(scene_frame::SceneProcOp::Nop)
        }, 
    }}

    fn require_rendering(
        &self, 
        depth: usize, 
        is_top: bool, 
    ) -> bool { match self {
        FSFrame::Game(_) => true,
        FSFrame::GamePause(_) => is_top, 
        FSFrame::GameOver(_) => is_top, 
    }}

    fn rendering(
        &self, 
        depth: usize, 
        is_top: bool, 
        renderer: &mut Self::Rdr, 
        frame_param: &Self::Fpr, 
    ) { match self {
        FSFrame::Game(g) => g.rendering(renderer),
        FSFrame::GamePause(gp) => gp.rendering(renderer), 
        FSFrame::GameOver(gp) => gp.rendering(renderer), 
    }}

    fn pop(self) -> Self::PopV {
        FSPopV {}
    }

    fn return_foreground(&mut self, popv: Self::PopV) {
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(Context::<_, scene_frame::SceneFrame<FSFrame>>::new(
        |fpr, rdr| {
            [FSFrame::Game(game::Game::new())].into_iter()
        }
    ))?.run().1?;
    Ok(())
}
