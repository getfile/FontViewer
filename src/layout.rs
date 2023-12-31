use jengine::jcolor::{BLACK, GRAY_DARK, YELLOW};
use jengine::jengine::JEngine;
use jengine::jtexture::JTexture;
use jengine::jtimer::{FrameTime45, JTime};
use jengine::math::jgeom::JRect;
use jengine::math::jgeom::SdlRect;
use jengine::ui::juilist::JScroll;

use crate::font::{FontDraw, FontObj};
use crate::unicode_range::UnicodeRangeMgr;

/// 字符尺寸类型
#[derive(Debug, PartialEq)]
enum CharSize {
    Overview = 0isize, //(4, 4)
    Tiny,              //(6,6)
    Small,             //(8, 8)
    Standart,          //(16, 16)
    Big,               //(32, 32)
    Large,             //(64, 64)
}
impl CharSize {
    pub fn next(&mut self, up: bool) {
        match self {
            CharSize::Overview => *self = if up { CharSize::Large } else { CharSize::Tiny },
            CharSize::Tiny => *self = if up { CharSize::Overview } else { CharSize::Small },
            CharSize::Small => *self = if up { CharSize::Tiny } else { CharSize::Standart },
            CharSize::Standart => *self = if up { CharSize::Small } else { CharSize::Big },
            CharSize::Big => *self = if up { CharSize::Standart } else { CharSize::Large },
            CharSize::Large => *self = if up { CharSize::Big } else { CharSize::Overview },
        }
    }
    pub fn size(&self) -> (i32, i32) {
        match self {
            Self::Overview => (4, 4),
            Self::Tiny => (6, 6),
            Self::Small => (8, 8),
            Self::Standart => (16, 16),
            Self::Big => (32, 32),
            Self::Large => (64, 64),
        }
    }
}

const CHARALL: u32 = 65536;

pub struct CharsLayout {
    wid: i32, //布局纹理尺寸
    hei: i32,
    tex_layout: Option<JTexture>,
    tex_char: Option<JTexture>,
    char_size: CharSize,          //字符尺寸类型, 决定字符块的尺寸
    tex_cursor: Option<JTexture>, //光标纹理尺寸, 和字符块尺寸一致

    pub char_first: u32, //视图的第一个码点
    char_last: u32,      //视图的最后一个码点
    char_idx: u32,       //当前绘制的字符码点
    char_now: i32,       //当前光标的字符id
    title_old: bool,

    unicodemgr: UnicodeRangeMgr,

    vscroll: JScroll,
    hscroll: JScroll,
}
impl Default for CharsLayout {
    fn default() -> Self {
        Self {
            wid: 1,
            hei: 1,
            tex_layout: None,
            tex_char: None,
            char_size: CharSize::Overview,
            tex_cursor: None,

            char_first: 0,
            char_last: 0,
            char_idx: 0, //用于分帧绘制
            char_now: 0,
            title_old: true,

            unicodemgr: UnicodeRangeMgr::init(),

            vscroll: JScroll::default(),
            hscroll: JScroll::default(),
        }
    }
}

impl CharsLayout {
    pub fn vlen(hlen: i32) -> i32 {
        CHARALL as i32 / hlen + if CHARALL as i32 % hlen == 0 { 0 } else { 1 }
    }

    pub fn init(&mut self, engine: &mut JEngine) {
        self.unicodemgr.debug();

        let (wid, hei) = engine.window().size();
        self.wid = wid as i32;
        self.hei = hei as i32;

        self.hscroll.init(wid as i32, 24, false, wid as i32 / 24);
        self.vscroll.init(hei as i32, 24, true, Self::vlen(self.hscroll.unit_box));
        self.set_char_size(engine, true);
        self.set_layout_size(engine);
    }

    pub fn set_char_size(&mut self, engine: &mut JEngine, big: bool) {
        self.char_size.next(!big);
        let (wid, hei) = self.char_size.size();
        self.hscroll.set_unit_size_unitnum(wid, self.wid / wid);
        self.vscroll.set_unit_size_unitnum(hei, Self::vlen(self.hscroll.unit_box));
        self.update_unit_now();
        self.update_charlist();

        self.init_cursor(engine);
        if self.tex_layout.is_some() {
            self.clear_layout(engine);
        }
    }

    pub fn set_layout_size(&mut self, engine: &mut JEngine) {
        let (wid, hei) = engine.window().size();
        self.wid = wid as i32;
        self.hei = hei as i32;

        self.hscroll.set_box_size_unitnum(self.wid, self.wid / self.hscroll.unit_size);
        self.vscroll.set_box_size_unitnum(self.hei, Self::vlen(self.hscroll.unit_box));
        self.update_unit_now();
        self.update_charlist();

        // 调整布局纹理的尺寸
        self.tex_layout = Some(JTexture::from_window(engine.canvas(), wid, hei, false));
        self.clear_layout(engine);
    }

    // 初始化或调整光标纹理
    pub fn init_cursor(&mut self, engine: &mut JEngine) {
        let (wid, hei) = self.char_size.size();
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
    }

    /// 更新各滚动条边长(面积不变,边长调整), 及恢复原来字符的坐标
    fn update_unit_now(&mut self) {
        // self.hscroll.set_units(self.wid / self.hscroll.unit_size);
        // self.vscroll.set_units(Self::vlen(self.hscroll.unit_box));
        self.hscroll.set_unit_now(self.char_now % self.hscroll.unit_box);
        self.vscroll.set_unit_now(self.char_now / self.hscroll.unit_box);
        // println!("must be {}, now is {}", self.char_now, self.hscroll.unit_now + self.vscroll.unit_now * self.hscroll.unit_box);
    }

    fn update_charlist(&mut self) {
        self.char_first = (self.vscroll.unit_start * self.hscroll.unit_box + self.hscroll.unit_start) as u32;
        let last = (self.vscroll.unit_box * self.hscroll.unit_box) as u32 + self.char_first;
        self.char_last = if last > CHARALL { CHARALL } else { last };
        self.char_idx = self.char_first;
        self.unicodemgr.reset();

        // println!("::{:?}", self.char_size);
        // println!("first {}, last {}, line_char {}, page_line {}, now {}", self.char_first, self.char_last, self.hscroll.unit_box, self.vscroll.unit_box, self.char_now);
        // print!("hscroll: ");
        // self.hscroll.debug();
        // print!("vscroll: ");
        // self.vscroll.debug();
    }

    /// 光标定位到鼠标所在位置
    pub fn set_charid(&mut self, engine: &mut JEngine, mx: i32, my: i32) {
        let (sx, sy) = (mx / self.hscroll.unit_size + self.hscroll.unit_start, (my - self.vscroll.unit_start_offs) / self.vscroll.unit_size + self.vscroll.unit_start);
        self.hscroll.set_unit_now(sx);
        self.line_move(engine, sy - self.vscroll.unit_now);
    }

    /// 计算鼠标坐标所在字符的位置(unicode码)
    pub fn charid(&self, mx: i32, my: i32) -> u32 {
        (mx / self.hscroll.unit_size + (my - self.vscroll.unit_start_offs) / self.vscroll.unit_size * self.hscroll.unit_box) as u32 + self.char_first
    }

    // 计算光标所在字符的位置(unicode码)
    fn update_char_now(&mut self) {
        self.char_now = self.hscroll.unit_now + self.vscroll.unit_now * self.hscroll.unit_box;
        if self.char_now >= CHARALL as i32 {
            self.char_now = CHARALL as i32 - 1;
            self.update_unit_now();
        }
        // println!("{}", self.char_now);
        self.title_old = true;
    }

    pub fn char_move(&mut self, engine: &mut JEngine, dist: i32) {
        self.hscroll.line_move(dist);
        self.update_char_now();
    }
    pub fn line_move(&mut self, engine: &mut JEngine, dist: i32) {
        if self.vscroll.line_move(dist) {
            self.update_charlist();
            self.clear_layout(engine);
        }
        self.update_char_now();
    }
    pub fn page_down(&mut self, engine: &mut JEngine) {
        if self.vscroll.page_down() {
            self.update_charlist();
            self.clear_layout(engine);
        }
        self.update_char_now();
    }
    pub fn page_up(&mut self, engine: &mut JEngine) {
        if self.vscroll.page_up() {
            self.update_charlist();
            self.clear_layout(engine);
        }
        self.update_char_now();
    }
    pub fn page_home(&mut self, engine: &mut JEngine) {
        self.hscroll.home();
        if self.vscroll.home() {
            self.update_charlist();
            self.clear_layout(engine);
        }
        self.update_char_now();
    }
    pub fn page_end(&mut self, engine: &mut JEngine) {
        self.hscroll.end();
        if self.vscroll.end() {
            self.update_charlist();
            self.clear_layout(engine);
        }
        self.update_char_now();
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

    /// 生成布局纹理(分帧绘制所有字符)
    fn draw_layout(&mut self, engine: &mut JEngine, fontobj: &mut FontObj) {
        let timer = JTime::inst();
        engine
            .canvas()
            .with_texture_canvas(&mut (self.tex_layout.as_mut().unwrap().sdl_texture), |tex_canvas| {
                let mut fontdraw = FontDraw::new(tex_canvas, self.hscroll.unit_size as f32 / fontobj.unit_em() as f32);
                // let mut color = JColor::new(1.0, 1.0, 1.0, 1.0);
                let mut rect = JRect::new(0., 0., self.hscroll.unit_size as f32, self.vscroll.unit_size as f32);
                let time_start = timer.count();

                loop {
                    let cx = (self.char_idx - self.char_first) as i32 % self.hscroll.unit_box;
                    let cy = (self.char_idx - self.char_first) as i32 / self.hscroll.unit_box;
                    rect.x = (cx * self.hscroll.unit_size) as f32;
                    rect.y = (cy * self.vscroll.unit_size + self.vscroll.unit_start_offs) as f32;

                    let range = self.unicodemgr.range(self.char_idx);
                    let color = if range.is_none() { GRAY_DARK } else { range.unwrap().color() };
                    fontdraw.set_rect(rect, color);

                    let gid = fontobj.get_glyph_id(self.char_idx);
                    if gid.is_some() {
                        // fontdraw.set_bound(rrect);
                        if self.char_size == CharSize::Overview {
                        } else if self.char_size == CharSize::Tiny {
                            fontdraw.draw_glyph_box(rect);
                        } else {
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
        if self.title_old {
            self.title_old = false;
            let gidobj = fontobj.get_glyph_id(self.char_now as u32);
            let gid = if gidobj.is_none() { -1 } else { gidobj.unwrap().0 as i32 };
            let rangeobj = self.unicodemgr.range_any(self.char_now as u32);
            if rangeobj.is_none() {
                let out = format!("unicode {:04X}({}), glyphid {}, Set: None", self.char_now, self.char_now, gid);
                engine.window().set_title(&out).unwrap();
            } else {
                let range = rangeobj.unwrap();
                let out = format!("unicode {:04X}({}), glyphid {}, Set: {} [{}-{}]", self.char_now, self.char_now, gid, range.1, range.2, range.3);
                engine.window().set_title(&out).unwrap();
            };
        }
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
