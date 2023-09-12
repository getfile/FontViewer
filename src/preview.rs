use jengine::jcolor::{self, JColor, BLACK, WHITE};
use jengine::jengine::JEngine;
use jengine::jtexture::JTexture;
use jengine::math::jgeom::JRect;
use ttf_parser::GlyphId;

use crate::font::{FontDraw, FontObj, GlyphInfo};

pub struct CharPreview {
    uid: u32, //字符unicode
    tex: Option<JTexture>,
    gid: Option<GlyphId>, //字形id
    ginfo: Option<GlyphInfo>,
}
impl Default for CharPreview {
    fn default() -> Self {
        Self { uid: 0, tex: None, gid: None, ginfo: None }
    }
}

impl CharPreview {
    pub fn init(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        self.tex = Some(JTexture::from_window(_engine.canvas(), 300, 300, false));
        self.draw_tex(_engine, fontobj);
    }
    /// 更新字符
    pub fn update(&mut self, engine: &mut JEngine, fontobj: &mut FontObj, uid: u32) {
        if uid == self.uid {
            return;
        }
        self.uid = uid;

        // println!("preview update char: {}", uid);
        self.gid = fontobj.get_glyph_id(uid);
        self.ginfo = fontobj.get_glyph_info(self.gid);
        if self.ginfo.is_some() {
            // println!("字形数据 {:?}", self.ginfo.as_ref().unwrap());
        }
        self.draw_tex(engine, fontobj)
    }
    /// 绘制字符
    fn draw_tex(&mut self, _engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.gid.is_none() {
            return;
        }
        _engine
            .canvas()
            .with_texture_canvas(&mut (self.tex.as_mut().unwrap().sdl_texture), |tex_canvas| {
                tex_canvas.set_draw_color(BLACK.to_sdlcolor());
                tex_canvas.clear();

                // if self.ginfo.is_some()
                let rect = self.ginfo.as_ref().unwrap().bound;

                // let mut col = jcolor::WHITE;
                // tex_canvas.set_draw_color(col.to_sdlcolor());
                // tex_canvas.draw_rect(SdlRect::new()).unwrap();

                let mut fontdraw = FontDraw::new(tex_canvas, 300. / fontobj.unit_em() as f32);
                fontdraw.set_rect(JRect::new(0., 0., 300., 300.), false);
                fontdraw.set_bound(rect);
                let color = JColor::new(0.4, 0.4, 0.7, 1.0);
                fontdraw.color(&color);
                fontobj.draw_glyph(self.gid.unwrap(), &mut fontdraw);
            })
            .unwrap();
    }
    /// 显示字符
    pub fn draw(&mut self, _engine: &mut JEngine) {
        if self.gid.is_none() {
            return;
        }
        _engine.renderer().draw_pic(&self.tex.as_ref().unwrap(), 12, 12);
    }
}
