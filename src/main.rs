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

pub struct FerrisShooting {
    renderer: renderer::FSRenderer, 
    unlock_mouse: control::Latch, 
    vfield: simple2d::types::VisibleField, 
    cycle: cycle_measure::CycleMeasure, 
}
impl Frame for FerrisShooting {
    type Initializer = ();

    fn window_builder() -> winit::window::WindowBuilder {
        winit::window::WindowBuilder::new()
            .with_title("Ferris shooting")
            .with_resizable(false)
            .with_active(true)
            .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
    }

    fn new(
        initializer: Self::Initializer, 
        window: &winit::window::Window, 
        gfx: &tm_wg_wrapper::ctx::gfx::GfxCtx, 
        sfx: &tm_wg_wrapper::ctx::sfx::SfxCtx, 
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(mon) = window.primary_monitor() {
            window.set_outer_position({
                let size = mon.size();
                winit::dpi::PhysicalPosition::new(size.width / 2, size.height / 2)
            });
        }

        let renderer = renderer::FSRenderer::new(gfx)?;
        let vfield = simple2d::types::VisibleField::new(
            &renderer.camera.camera, 
        );
        let cycle = cycle_measure::CycleMeasure::new();

        Ok(Self {
            renderer,
            unlock_mouse: control::Latch::default(), 
            vfield, 
            cycle, 
        })
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
        self.renderer.resize(size)
    }

    fn rendering<'r>(
        &mut self, 
        render_chain: tm_wg_wrapper::ctx::gfx::RenderingChain<'r>, 
    ) -> tm_wg_wrapper::ctx::gfx::RenderingChain<'r> {
        render_chain.rendering(&mut self.renderer)
    }

    fn update(
        &mut self, 
        window: &winit::window::Window, 
        _ctrl: &mut winit::event_loop::ControlFlow, 
        _gfx: &tm_wg_wrapper::ctx::gfx::GfxCtx, 
        _sfx: &tm_wg_wrapper::ctx::sfx::SfxCtx, 
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.unlock_mouse.update();
        if self.unlock_mouse.latch_off_count() == 1 {
            window.set_cursor_visible(false);
            window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
        } else if self.unlock_mouse.latch_on_count() == 1 {
            window.set_cursor_visible(false);
            window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
        };
        self.vfield = simple2d::types::VisibleField::new(&self.renderer.camera.camera);
        self.cycle.update();

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
