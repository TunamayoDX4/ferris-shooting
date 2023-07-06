use std::io::Read;

use tm_wg_wrapper::{
    prelude::*, 
    util::simple2d::Simple2DRender, 
};
use simple2d::{
    S2DCamera, 
    SquareShared, 
    ImagedShared, 
    img_obj, 
    font_typing, 
};

pub struct FSRenderer {
    pub camera: S2DCamera, 
    square: SquareShared, 
    imaged: ImagedShared, 

    img_obj: img_obj::ImgObjRenderShared, 

    pub ferris: img_obj::ImgObjRender, 
    pub aim: img_obj::ImgObjRender, 
    pub gear: img_obj::ImgObjRender, 
    pub enemy: img_obj::ImgObjRender, 
    pub font: font_typing::FontTypeRender, 
    pub indicator: img_obj::ImgObjRender, 
}
impl FSRenderer {
    pub fn new(
        gfx: &GfxCtx
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let camera = S2DCamera::new(
            simple2d::types::Camera {
                position: [0., 0.].into(),
                size: [
                    gfx.config.width as f32, 
                    gfx.config.height as f32, 
                ].into(),
                zoom: 1.,
                rotation: 0.,
            }, 
            gfx, 
        );
        let square = SquareShared::new(gfx);
        let imaged = ImagedShared::new(gfx);

        let img_obj = img_obj::ImgObjRenderShared::new(
            gfx, 
            &camera, 
            &imaged, 
        );

        let ferris = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/ferris.png"
        )?;
        let aim = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/aim.png"
        )?;
        let gear = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/gear.png"
        )?;
        let enemy = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/enemy_sprite.png"
        )?;
        let font = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/font.png", 
        )?;
        let font_set = font_typing::FontSet {
            fonts: [
                ' ', '!', '"', '#', 
                '$', '%', '&', '\'', 
                '(', ')', '*', '+', 
                ',', '-', '.', '/', 
                '0', '1', '2', '3', 
                '4', '5', '6', '7', 
                '8', '9', ':', ';', 
                '<', '=', '>', '?', 
                '@', 'A', 'B', 'C', 
                'D', 'E', 'F', 'G', 
                'H', 'I', 'J', 'K', 
                'L', 'M', 'N', 'O', 
                'P', 'Q', 'R', 'S', 
                'T', 'U', 'V', 'W', 
                'X', 'Y', 'Z', '[', 
                '\\', ']', '^', '_', 
                '`', 'a', 'b', 'c', 
                'd', 'e', 'f', 'g', 
                'h', 'i', 'j', 'k', 
                'l', 'm', 'n', 'o', 
                'p', 'q', 'r', 's', 
                't', 'u', 'v', 'w', 
                'x', 'y', 'z', '{', 
                '|', '}', '~', '\n', 
                'ｱ', 'ｲ', 'ｳ', 'ｴ', 
                'ｵ', 'ｶ', 'ｷ', 'ｸ', 
                'ｹ', 'ｺ', 'ｻ', 'ｼ', 
                'ｽ', 'ｾ', 'ｿ', 'ﾀ', 
                'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 
                'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 
                'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 
                'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 
                'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 
                'ヰ', 'ﾕ', 'ヱ', 'ﾖ', 
                'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 
                'ﾛ', 'ﾜ', 'ｦ', 'ﾝ', 
                'ｧ', 'ｨ', 'ｩ', 'ｪ', 
                'ｫ', 'ｬ', 'ｭ', 'ｮ', 
                'ﾞ', 'ﾟ', '､', '｡', 
                '･', //'', '', '', 
            ].into_iter()
                .enumerate()
                .map(|(i, c)| (
                    c, [16. * (i % 16) as f32, 32. * (i / 16) as f32]
                ))
                .map(|(c, s)| (c, font_typing::CharModel {
                    tex_coord: s,
                    tex_size: [16., 32.],
                    base_line: [0., 0.],
                }))
                .collect(),
            default: font_typing::CharModel { 
                tex_coord: [16. * 15., 32. * 5.], 
                tex_size: [16., 32.], 
                base_line: [0., 0.] 
            },
        };
        let font = font_typing::FontTypeRender::new(
            font, 
            font_set, 
        );
        let indicator = img_obj::ImgObjRender::new(
            gfx, 
            &imaged, 
            "./assets/images/indicator.png", 
        )?;

        Ok(Self {
            camera,
            square,
            imaged,
            img_obj,
            ferris,
            aim, 
            gear,
            enemy, 
            font, 
            indicator, 
        })
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.camera.camera.size = [
            size.width as f32, 
            size.height as f32, 
        ].into();
    }
}
impl Renderer for FSRenderer {
    fn rendering(
        &mut self, 
        _output: &wgpu::SurfaceTexture, 
        view: &wgpu::TextureView, 
        gfx: &GfxCtx, 
    ) {
        let mut encoder = gfx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("fs renderer encoder") }
        );
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("bg_clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.25,
                        g: 0.25,
                        b: 0.25,
                        a: 1.,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        self.gear.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        self.enemy.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        self.ferris.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        self.aim.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        self.font.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        self.indicator.rendering(gfx, &mut encoder, view, &self.camera, (
            &self.square, 
            &self.imaged, 
            &self.img_obj, 
        ));
        gfx.queue.submit(Some(encoder.finish()));
    }
}