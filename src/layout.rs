use jengine::jcolor::{self, JColor, BLACK, GRAY};
use jengine::jengine::JEngine;
use jengine::jtexture::{JTexture, SdlPixelFormat, SdlSurface};
use jengine::jtimer::{FrameTime45, JTime};
use jengine::math::jgeom::JRect;
use jengine::math::jgeom::SdlRect;
use ttf_parser::Rect;

use crate::font::{FontDraw, FontObj};

pub struct CharsLayout {
    tex_layout: Option<JTexture>,
    tex_char: Option<JTexture>,
    page_line: u32,      //一屏有几行
    pub line_box: u32,   //每行有几个码点
    pub char_first: u32, //视图的第一个码点
    char_last: u32,      //视图的最后一个码点
    char_idx: u32,       //当前绘制字符的起始码点
    pub box_wid: u32,    //字符的指定宽度
    pub box_hei: u32,    //字符的指定高度
    block_start: u32,    //选中块的首码点
    block_end: u32,      //选中块的尾码点
}
impl Default for CharsLayout {
    fn default() -> Self {
        Self {
            tex_layout: None,
            block_start: 0,
            block_end: 0,
            char_first: 0,
            char_last: 0,
            page_line: 0,
            line_box: 0,
            char_idx: 0,
            tex_char: None,
            box_wid: 8,
            box_hei: 10,
        }
    }
}

impl CharsLayout {
    pub fn init(&mut self, engine: &mut JEngine) {
        self.resize_char(engine, 2.0);
        self.resize(engine);
    }

    // 更新布局纹理的内容
    pub fn update(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.char_idx >= self.char_last {
            return;
        }
        self.draw_tex(engine, fontobj);
    }

    pub fn line_down(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.char_last + self.line_box < 65536 {
            self.char_first += self.line_box;
            self.char_last += self.line_box;
            self.char_idx = self.char_first;
            self.clear_tex(engine);
        }
    }

    pub fn line_up(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.char_first >= self.line_box {
            self.char_first -= self.line_box;
            self.char_last -= self.line_box;
            self.char_idx = self.char_first;
            self.clear_tex(engine);
        }
    }

    pub fn page_down(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        let page = self.line_box * self.page_line;
        if self.char_last + page < 65536 {
            self.char_first += page;
            self.char_last += page;
            self.char_idx = self.char_first;
            self.clear_tex(engine);
        }
    }

    pub fn page_up(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        let page = self.line_box * self.page_line;
        if self.char_first >= self.line_box {
            self.char_first -= page;
            self.char_last -= page;
            self.char_idx = self.char_first;
            self.clear_tex(engine);
        }
    }
    pub fn page_home(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {}
    pub fn page_end(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {}

    /// 调整字符纹理的尺寸
    pub fn resize_char(&mut self, engine: &mut JEngine, scale: f32) {
        self.box_wid = (scale * 8.0) as u32;
        self.box_hei = (scale * 10.0) as u32;
        // let surf = SdlSurface::new(self.char_wid, self.char_hei, SdlPixelFormat::ARGB8888).unwrap();
        // self.tex_char = Some(JTexture::from_surface(engine.renderer(), &surf));
    }
    //// 调整布局纹理的尺寸
    pub fn resize(&mut self, engine: &mut JEngine) {
        let (wwid, whei) = engine.window().size();
        self.line_box = wwid / self.box_wid;
        self.page_line = whei / self.box_hei + 1;
        let last = self.line_box * self.page_line + self.char_first;
        self.char_last = if last > 65536 { 65536 } else { last };

        self.tex_layout = Some(JTexture::from_window(engine.canvas(), wwid, whei, false));
        self.char_idx = self.char_first;

        println!("first {}, last {}, line_char {}, page_line {}", self.char_first, self.char_last, self.line_box, self.page_line);
    }

    /// 清除布局纹理
    fn clear_tex(&mut self, engine: &mut JEngine) {
        let (wwid, whei) = engine.window().size();
        // println!("clear {}-{}", wwid, whei);
        engine
            .canvas()
            .with_texture_canvas(&mut (self.tex_layout.as_mut().unwrap().sdl_texture), |tex_canvas| {
                tex_canvas.set_draw_color(BLACK.to_sdlcolor());
                tex_canvas.fill_rect(SdlRect::new(0, 0, wwid, whei)).unwrap();
            })
            .unwrap();
    }

    /// 在布局纹理上绘制字符集
    fn draw_tex(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        // println!("draw_tex");
        let timer = JTime::inst();
        engine
            .canvas()
            .with_texture_canvas(&mut (self.tex_layout.as_mut().unwrap().sdl_texture), |tex_canvas| {
                let mut fontdraw = FontDraw::new(tex_canvas, self.box_wid as f32 / fontobj.unit_em() as f32);
                let color = JColor::new(0.3, 0.3, 0.0, 1.0);
                let mut rect = JRect::new(0., 0., self.box_wid as f32, self.box_hei as f32);
                let time_start = timer.count();
                loop {
                    let cx = (self.char_idx - self.char_first) % self.line_box;
                    let cy = (self.char_idx - self.char_first) / self.line_box;
                    rect.x = (cx * self.box_wid) as f32;
                    rect.y = (cy * self.box_hei) as f32;

                    let gid = fontobj.get_glyph_id(self.char_idx);
                    if gid.is_none() {
                        fontdraw.set_rect(rect, true);
                    } else {
                        fontdraw.set_rect(rect, false);
                        // fontdraw.set_bound(rrect);
                        fontobj.draw_glyph(gid.unwrap(), &mut fontdraw);
                    }
                    self.char_idx += 1;
                    if self.char_idx >= self.char_last {
                        break;
                    }
                    if timer.count() - time_start > FrameTime45 {
                        break;
                    }
                }
            })
            .unwrap();
    }
    /// 把布局纹理绘制到屏幕上
    pub fn draw(&mut self, engine: &mut JEngine) {
        engine.renderer().draw_pic(&self.tex_layout.as_ref().unwrap(), 0, 0);
    }
}
