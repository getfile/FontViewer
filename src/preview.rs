use jengine::jcolor::{self, JColor, WHITE};
use jengine::jengine::JEngine;
use jengine::jrenderer::{SdlCanvas, SdlRect};
use jengine::jtexture::JTexture;

use crate::font::{FontDraw, FontObj};

pub struct CharPreview {
    tex: Option<JTexture>,
}
impl Default for CharPreview {
    fn default() -> Self {
        Self { tex: None }
    }
}

impl CharPreview {
    pub fn init(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        self.tex = Some(JTexture::from_window(_engine.renderer(), 300, 300));
        self.draw_tex(_engine, fontobj);
    }
    fn draw_tex(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        // let canvas = _engine.canvas();
        _engine
            .canvas()
            .with_texture_canvas(
                &mut (self.tex.as_mut().unwrap().sdl_texture),
                |tex_canvas| {
					// tex_canvas.clear();
					let rect = SdlRect::new(0, 0, 3, 3);
					let rect = fontobj.get_bound();

					let mut col = jcolor::WHITE;
					col.0[3]=0.2;
					tex_canvas.set_draw_color(col.to_sdlcolor());
					tex_canvas.draw_rect(rect).unwrap();

                    let gid = fontobj.get_glyphs_id(0);
                    if gid.is_none() {
                        return;
                    }
                    let mut fontdraw = FontDraw::new(tex_canvas);
                    let color = JColor::new(0.4, 0.4, 0.7, 1.0);
                    fontdraw.color(&color);
                    fontdraw.rect(rect);
                    fontobj.draw_glyph(gid.unwrap(), &mut fontdraw);
                },
            )
            .unwrap();
    }
    pub fn draw(&mut self, _engine: &mut JEngine) {
        _engine
            .renderer()
            .draw_pic(&self.tex.as_ref().unwrap(), 12, 12);
    }
}
