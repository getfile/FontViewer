use jengine::jcolor::{JColor, BLACK, YELLOW};
use jengine::jengine::JEngine;
use jengine::jtexture::JTexture;
use jengine::jtimer::{FrameTime45, JTime};
use jengine::math::jgeom::JRect;
use jengine::math::jgeom::SdlRect;
use jengine::ui::juilist::JScroll;

use crate::font::{FontDraw, FontObj};

#[derive(PartialEq)]
enum CharSize {
    Overview = 0isize, //(4, 4)
    Small,             //(8, 8)
    Standart,          //(16, 16)
    Big,               //(32, 32)
    Large,             //(64, 64)
}
impl CharSize {
    pub fn next(&mut self, up: bool) {
        match self {
            CharSize::Overview => *self = if up { CharSize::Large } else { CharSize::Small },
            CharSize::Small => *self = if up { CharSize::Overview } else { CharSize::Standart },
            CharSize::Standart => *self = if up { CharSize::Small } else { CharSize::Big },
            CharSize::Big => *self = if up { CharSize::Standart } else { CharSize::Large },
            CharSize::Large => *self = if up { CharSize::Big } else { CharSize::Overview },
        }
    }
    pub fn size(&self) -> (i32, i32) {
        match self {
            Self::Overview => (4, 4),
            Self::Small => (8, 8),
            Self::Standart => (16, 16),
            Self::Big => (32, 32),
            Self::Large => (64, 64),
        }
    }
}

pub struct CharsLayout {
    tex_layout: Option<JTexture>,
    tex_char: Option<JTexture>,
    tex_cursor: Option<JTexture>,

    pub char_first: u32, //视图的第一个码点
    char_last: u32,      //视图的最后一个码点
    char_idx: u32,       //当前绘制的字符码点
    char_size: CharSize, //字符尺寸的缩放

    vscroll: JScroll,
    hscroll: JScroll,
    // pub box_wid: u32, //字符的指定宽度
    // pub box_hei: u32, //字符的指定高度

    // page_line: u32,    //一屏有几行
    // pub line_box: u32, //每行有几个码点
    // block_start: u32, //选中块的首码点
    // block_end: u32,   //选中块的尾码点
}
impl Default for CharsLayout {
    fn default() -> Self {
        Self {
            tex_layout: None,
            tex_char: None,
            tex_cursor: None,
            // block_start: 0,
            // block_end: 0,
            char_first: 0,
            char_last: 0,
            char_idx: 0,
            char_size: CharSize::Overview,
            vscroll: JScroll::default(),
            hscroll: JScroll::default(),
        }
    }
}

impl CharsLayout {
    pub fn init(&mut self, engine: &mut JEngine) {
        let (wid, hei) = engine.window().size();
        self.hscroll.init(wid as i32, 24, false, wid as i32 / 24);
        self.vscroll.init(hei as i32, 24, true, 65536 / self.hscroll.unit_box);
        self.set_char_size(engine, true);
        self.set_layout_size(engine);
    }

    pub fn set_char_size(&mut self, engine: &mut JEngine, big: bool) {
        self.char_size.next(!big);
        let (wid, hei) = self.char_size.size();
        self.hscroll.set_unit_size(wid);
        self.vscroll.set_unit_size(hei);
        self.update_charlist();

        // 调整光标纹理的尺寸
        self.tex_cursor = Some(JTexture::from_window(engine.canvas(), wid as u32, hei as u32, true));
        engine
            .canvas()
            .with_texture_canvas(&mut self.tex_cursor.as_mut().unwrap().sdl_texture, |tex_canvas| {
                let mut col = YELLOW.clone();
                col.0[3] = 0.75;
                tex_canvas.set_draw_color(col.to_sdlcolor());
                tex_canvas.fill_rect(SdlRect::new(0, 0, wid as u32, hei as u32)).unwrap();
            })
            .unwrap();
        if self.tex_layout.is_some() {
            self.clear_layout(engine);
        }
    }

    pub fn set_layout_size(&mut self, engine: &mut JEngine) {
        let (wwid, whei) = engine.window().size();
        self.hscroll.set_box_size(wwid as i32);
        self.vscroll.set_box_size(whei as i32);
        self.update_charlist();

        // 调整布局纹理的尺寸
        self.tex_layout = Some(JTexture::from_window(engine.canvas(), wwid, whei, false));
        self.clear_layout(engine);
    }

    pub fn update_charlist(&mut self) {
        self.char_first = (self.vscroll.unit_start * self.hscroll.unit_box + self.hscroll.unit_start) as u32;
        let last = (self.vscroll.unit_box * self.hscroll.unit_box) as u32 + self.char_first;
        self.char_last = if last > 65536 { 65536 } else { last };
        self.char_idx = self.char_first;
        println!("first {}, last {}, line_char {}, page_line {}", self.char_first, self.char_last, self.hscroll.unit_box, self.vscroll.unit_box);
    }

    pub fn charid(&self, mx: i32, my: i32) -> u32 {
        (mx / self.hscroll.unit_size + my / self.vscroll.unit_size * self.hscroll.unit_box) as u32 + self.char_first
    }

    pub fn char_left(&mut self, engine: &mut JEngine) {
        self.hscroll.line_up();
    }
    pub fn char_right(&mut self, engine: &mut JEngine) {
        self.hscroll.line_down();
    }
    pub fn line_down(&mut self, engine: &mut JEngine) {
        if self.vscroll.line_down() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }
    pub fn line_up(&mut self, engine: &mut JEngine) {
        if self.vscroll.line_up() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }
    pub fn page_down(&mut self, engine: &mut JEngine) {
        if self.vscroll.page_down() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }
    pub fn page_up(&mut self, engine: &mut JEngine) {
        if self.vscroll.page_up() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }
    pub fn page_home(&mut self, engine: &mut JEngine) {
        if self.vscroll.home() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }
    pub fn page_end(&mut self, engine: &mut JEngine) {
        if self.vscroll.end() {
            self.update_charlist();
            self.clear_layout(engine);
        }
    }

    /// 清除布局纹理
    fn clear_layout(&mut self, engine: &mut JEngine) {
        let (wwid, whei) = engine.window().size();
        engine
            .canvas()
            .with_texture_canvas(&mut (self.tex_layout.as_mut().unwrap().sdl_texture), |tex_canvas| {
                tex_canvas.set_draw_color(BLACK.to_sdlcolor());
                tex_canvas.fill_rect(SdlRect::new(0, 0, wwid, whei)).unwrap();
            })
            .unwrap();
    }

    /// 在布局纹理上绘制字符集
    fn draw_layout(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        let timer = JTime::inst();
        engine
            .canvas()
            .with_texture_canvas(&mut (self.tex_layout.as_mut().unwrap().sdl_texture), |tex_canvas| {
                let mut fontdraw = FontDraw::new(tex_canvas, self.hscroll.unit_size as f32 / fontobj.unit_em() as f32);
                let color = JColor::new(0.3, 0.3, 0.0, 1.0);
                let mut rect = JRect::new(0., 0., self.hscroll.unit_size as f32, self.vscroll.unit_size as f32);
                let time_start = timer.count();

                loop {
                    let cx = (self.char_idx - self.char_first) as i32 % self.hscroll.unit_box;
                    let cy = (self.char_idx - self.char_first) as i32 / self.hscroll.unit_box;
                    rect.x = (cx * self.hscroll.unit_size) as f32;
                    rect.y = (cy * self.vscroll.unit_size + self.vscroll.unit_start_offs) as f32;

                    let gid = fontobj.get_glyph_id(self.char_idx);
                    if gid.is_none() {
                        fontdraw.set_rect(rect, true);
                    } else {
                        fontdraw.set_rect(rect, false);
                        // fontdraw.set_bound(rrect);
                        if self.char_size != CharSize::Overview {
                            fontobj.draw_glyph(gid.unwrap(), &mut fontdraw);
                        }
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

    // 更新布局纹理的内容
    pub fn update(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        if self.char_idx >= self.char_last {
            return;
        }
        self.draw_layout(engine, fontobj);
    }

    /// 把布局纹理和光标纹理绘制到屏幕上
    pub fn draw(&mut self, engine: &mut JEngine) {
        engine.renderer().draw_pic(&self.tex_layout.as_ref().unwrap(), 0, 0);
        let (cx, cy) = (
            (self.hscroll.unit_now - self.hscroll.unit_start) * self.hscroll.unit_size + self.hscroll.unit_start_offs,
            (self.vscroll.unit_now - self.vscroll.unit_start) * self.vscroll.unit_size + self.vscroll.unit_start_offs,
        );
        // println!("cursor pos {}-{}", cx, cy);
        engine.renderer().draw_pic(&self.tex_cursor.as_ref().unwrap(), cx, cy);
        // self.vscroll.debug();
    }
}
