type ULazy<T> = once_cell::unsync::Lazy<T>;
use std::cell::RefCell;
use rand::SeedableRng;
use tm_wg_wrapper::prelude::*;

thread_local! {
    pub static RNG: ULazy<RefCell<rand_pcg::Pcg64>> = ULazy::new(||
        RefCell::new(
            rand_pcg::Pcg64::from_entropy()
        )
    );
}

pub mod log;
pub mod renderer;

pub mod old;

pub mod elem;

pub mod util;

/// Ferris Shootingのフレーム
pub struct FerrisShooting {
    renderer: renderer::FSRenderer, 
    element: elem::FSElement, 
    cycle: cycle_measure::CycleMeasure, 
    visible_field: simple2d::types::VisibleField, 
    mouse_unlock: util::Latch, 
}
impl Frame for FerrisShooting {
    type Initializer = ();

    fn window_builder() -> winit::window::WindowBuilder {
        winit::window::WindowBuilder::new()
            .with_title("Ferris Shooting")
            .with_inner_size(winit::dpi::PhysicalSize::new(640, 960))
            .with_active(true)
            .with_resizable(false)
    }

    fn new(
        _initializer: Self::Initializer, 
        gfx: &tm_wg_wrapper::ctx::gfx::GfxCtx, 
        _sfx: &tm_wg_wrapper::ctx::sfx::SfxCtx, 
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let renderer = renderer::FSRenderer::new(gfx)?;
        let element = elem::FSElement::new();
        let cycle = cycle_measure::CycleMeasure::new();
        let visible_field = simple2d::types::VisibleField::new(
            &renderer.camera.camera
        );
        let mouse_unlock = util::Latch::default();

        Ok(Self {
            renderer,
            element,
            cycle, 
            visible_field, 
            mouse_unlock, 
        })
    }

    fn input_key(
        &mut self, 
        keycode: VirtualKeyCode, 
        state: ElementState, 
    ) {
        self.element.input_key(keycode, state);
        match keycode {
            VirtualKeyCode::Escape => self.mouse_unlock.trigger(state), 
            _ => {}, 
        }
    }

    fn input_mouse_button(
        &mut self, 
        button: MouseButton, 
        state: ElementState, 
    ) {
        self.element.input_mouse_button(button, state);
    }

    fn input_mouse_motion(
        &mut self, 
        delta: (f64, f64), 
    ) {
        let motion = nalgebra::Vector2::new(
            delta.0 as f32, 
            -delta.1 as f32
        );
        self.element.input_mouse_motion(motion, &self.visible_field);
    }

    fn input_mouse_scroll(
        &mut self, 
        _delta: MouseScrollDelta, 
    ) {
    }

    fn window_resizing(
        &mut self, 
        size: winit::dpi::PhysicalSize<u32>, 
    ) {
        self.renderer.resize(size);
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
        self.mouse_unlock.update();
        if self.mouse_unlock.latch_off_count() == 1 {
            window.set_cursor_visible(false);
            window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
        } else if self.mouse_unlock.latch_on_count() == 1 {
            window.set_cursor_visible(true);
            window.set_cursor_grab(winit::window::CursorGrabMode::None)?;
        }
        self.visible_field = simple2d::types::VisibleField::new(&self.renderer.camera.camera);
        self.cycle.update();
        self.element.update(
            window, 
            &self.cycle,    
            &self.visible_field, 
            &mut self.renderer.ferris, 
            &mut self.renderer.aim, 
            &mut self.renderer.gear, 
            &mut self.renderer.enemy, 
        );
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::fern_init()?;
    pollster::block_on(Context::<FerrisShooting>::new(()))?.run().1?;

    Ok(())
}
