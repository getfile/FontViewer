use jengine::jcolor::{self, JColor, WHITE};
use jengine::jengine::JEngine;
use jengine::jrenderer::{SdlCanvas, SdlRect};
use jengine::jtexture::JTexture;
use ttf_parser::GlyphId;

use crate::font::{FontDraw, FontObj};

pub struct CharPreview {
    tex: Option<JTexture>,
    gid: Option<GlyphId>,
}
impl Default for CharPreview {
    fn default() -> Self {
        Self {
            tex: None,
            gid: None,
        }
    }
}

impl CharPreview {
    pub fn init(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        self.tex = Some(JTexture::from_window(_engine.renderer(), 300, 300, false));
        self.draw_tex(_engine, fontobj);
    }
    /// 更新字符
    pub fn update(&mut self, engine: &mut JEngine, fontobj: &mut FontObj, uid: u32) {
        // println!("update {}", uid);
        self.gid = fontobj.get_glyphs_id(uid);
        self.draw_tex(engine, fontobj)
    }
    /// 绘制字符
    fn draw_tex(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.gid.is_none() {
            return;
        }
        _engine
            .canvas()
            .with_texture_canvas(
                &mut (self.tex.as_mut().unwrap().sdl_texture),
                |tex_canvas| {
                    tex_canvas.clear();
                    let rect = SdlRect::new(0, 0, 3, 3);
                    let rect = fontobj.get_bound();

                    let mut col = jcolor::WHITE;
                    col.0[3] = 0.2;
                    tex_canvas.set_draw_color(col.to_sdlcolor());
                    let r = SdlRect::new(0, 0, rect.width(), rect.height());
                    tex_canvas.draw_rect(r).unwrap();

                    let mut fontdraw = FontDraw::new(tex_canvas);
                    let color = JColor::new(0.4, 0.4, 0.7, 1.0);
                    fontdraw.color(&color);
                    fontdraw.rect(rect);
                    fontobj.draw_glyph(self.gid.unwrap(), &mut fontdraw);
                },
            )
            .unwrap();
    }
    /// 显示字符
    pub fn draw(&mut self, _engine: &mut JEngine) {
        if self.gid.is_none() {
            return;
        }
        _engine
            .renderer()
            .draw_pic(&self.tex.as_ref().unwrap(), 12, 12);
    }
}
