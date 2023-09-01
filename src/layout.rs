use jengine::jcolor::{self, JColor};
use jengine::jengine::JEngine;
use jengine::jrenderer::SdlRect;
use jengine::jtexture::{JTexture, SdlPixelFormat, SdlSurface};
use jengine::jtimer::FrameTime45;

use crate::font::{FontDraw, FontObj};

pub struct CharsLayout {
    tex_layout: Option<JTexture>,
    tex_char: Option<JTexture>,
    page_line: u32,   //一屏有几行
    line_chars: u32,  //每行有几个码点
    char_first: u32,  //视图的第一个码点
    char_last: u32,   //视图的最后一个码点
    char_idx: u32,    //当前绘制字符的起始码点
    char_wid: u32,    //字符的指定宽度
    char_hei: u32,    //字符的指定高度
    block_start: u32, //选中块的首码点
    block_end: u32,   //选中块的尾码点
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
            line_chars: 0,
            char_idx: 0,
            tex_char: None,
            char_wid: 8,
            char_hei: 10,
        }
    }
}

impl CharsLayout {
    pub fn init(&mut self, engine: &mut JEngine) {
        self.resize(engine);
        self.resize_char(engine, 1.0);
    }
    // 更新布局纹理的内容
    pub fn update(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.char_idx >= self.char_last {
            return;
        }
        self.draw_tex(engine, fontobj);
    }
    /// 调整字符纹理的尺寸
    pub fn resize_char(&mut self, engine: &mut JEngine, scale: f32) {
        self.char_wid = (scale * 8.0) as u32;
        self.char_hei = (scale * 10.0) as u32;
        let surf = SdlSurface::new(self.char_wid, self.char_hei, SdlPixelFormat::ARGB8888).unwrap();
        self.tex_char = Some(JTexture::from_surface(engine.renderer(), &surf));
    }
    //// 调整布局纹理的尺寸
    pub fn resize(&mut self, engine: &mut JEngine) {
        let (wwid, whei) = engine.window().size();
        self.line_chars = wwid / self.char_wid;
        self.page_line = whei / self.char_hei + 1;
        let last = self.line_chars * self.page_line + self.char_first;
        self.char_last = if last > 65536 { 65536 } else { last };

        self.tex_layout = Some(JTexture::from_window(engine.renderer(), wwid, whei));
        self.char_idx = self.char_first;

        println!(
            "first {}, last {}, line_char {}, page_line {}",
            self.char_first, self.char_last, self.line_chars, self.page_line
        );
        // self.init_tex(engine);
    }
    /// 在布局纹理上绘制字符集
    fn draw_tex(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        let timer = engine.timer();
        let canvas = engine.canvas();
        canvas
            .with_texture_canvas(
                &mut (self.tex_layout.as_mut().unwrap().sdl_texture),
                |tex_canvas| {
                    let mut fontdraw = FontDraw::new(tex_canvas);
                    let color = JColor::new(0.3, 0.3, 0.0, 1.0);
                    let mut rect = SdlRect::new(0, 0, self.char_wid - 1, self.char_hei - 1);
                    // let time_start = timer.count();
                    loop {
                        let cx = (self.char_idx - self.char_first) % self.line_chars;
                        let cy = (self.char_idx - self.char_first) / self.line_chars;
                        rect.x = (cx * self.char_wid) as i32;
                        rect.y = (cy * self.char_hei) as i32;

                        let gid = fontobj.get_glyphs_id(self.char_idx);
                        if gid.is_none() {
                            // tex_canvas.set_draw_color(color.to_sdlcolor());
							fontdraw.color(&color);
						} else {
                            // tex_canvas.set_draw_color(jcolor::WHITE.to_sdlcolor());
							fontdraw.color(&jcolor::WHITE);
							fontdraw.rect(rect);
                            fontobj.draw_glyph(gid.unwrap(), &mut fontdraw);
							break;
                        }
                        // tex_canvas.fill_rect(rect).unwrap();

                        // if let Some(glyid) = fontobj.get_glyphs_id(self.char_idx) {
                        //     println!("{} => {}", self.char_idx, glyid.0);
                        // }

                        self.char_idx += 1;
                        if self.char_idx >= self.char_last {
                            break;
                        }
                        // if timer.count() - time_start > FrameTime45 {
                        // break;
                        // }
                    }
                },
            )
            .unwrap();
    }
    /// 把布局纹理绘制到屏幕上
    pub fn draw(&mut self, engine: &mut JEngine) {
        engine
            .renderer()
            .draw_pic(&self.tex_layout.as_ref().unwrap(), 0, 0);
    }
}
