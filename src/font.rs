use jengine::jcolor::JColor;
use jengine::jcolor::GRAY;
use jengine::jcolor::GRAY_DARK;
use jengine::jcolor::WHITE;
use jengine::jcolor::YELLOW;
use jengine::jengine::SdlCanvas;
// use jengine::jtexture::JPic;
use jengine::math::jgeom::JRect;
use jengine::math::jgeom::SdlRect;
use jengine::math::jvec2::Vec2;
// use jengine::jtexture::SdlSurface;
use ttf_parser::{Face, GlyphId, OutlineBuilder, Rect};

pub struct FontDraw<'a> {
    canvas: &'a mut SdlCanvas,
    bound: Rect, //字体包围盒, 字体自带
    rect: JRect, //绘制区域
    origin: Vec2,
    first: Vec2,
    start: Vec2,
    scale: f32,
}
impl<'a> FontDraw<'a> {
    pub fn new(canvas: &'a mut SdlCanvas, scale: f32) -> Self {
        Self {
            canvas,
            scale,
            first: Vec2::new(0.0, 0.0),
            start: Vec2::new(0.0, 0.0),
            rect: JRect::new(0.0, 0.0, 0.0, 0.0),
            origin: Vec2::ONE,
            bound: Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0 },
        }
    }
    pub fn color(&mut self, color: &JColor) {
        self.canvas.set_draw_color(color.to_sdlcolor());
    }
    pub fn set_rect(&mut self, rect: JRect, nothing: bool) {
        self.rect = rect;
        self.origin = Vec2::new(self.rect.x, self.rect.y + self.rect.hei);

        if nothing {
            self.canvas.set_draw_color(GRAY_DARK.to_sdlcolor());
        } else {
            self.canvas.set_draw_color(GRAY.to_sdlcolor());
        }
        self.canvas.draw_rect(self.rect.to_sdlrect()).unwrap();
        self.canvas.set_draw_color(WHITE.to_sdlcolor());
    }
    pub fn set_bound(&mut self, rect: Rect) {
        self.bound = rect;
    }
    pub fn draw(&mut self) {
        println!("draw");
    }
}
impl<'a> OutlineBuilder for FontDraw<'a> {
    fn close(&mut self) {
        self.canvas.draw_line(self.start.to_sdlpoint(), self.first.to_sdlpoint()).unwrap();
        // println!("close");
    }
    fn move_to(&mut self, x: f32, y: f32) {
        self.first = Vec2::new(x, -y) * self.scale + self.origin;
        self.start = self.first;
        // (self.start.x, self.start.y) = (x as i32+self.x, self.rect.hself.bound.h as f32-y as i32+self.x);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let mut end = Vec2::new(x, -y) * self.scale + self.origin;
        self.canvas.draw_line(self.start.to_sdlpoint(), end.to_sdlpoint()).unwrap();
        self.start = end;
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let mut start = self.start;
        let mut mid = Vec2::new(x1, -y1) * self.scale + self.origin;
        let mut end = Vec2::new(x, -y) * self.scale + self.origin;

        let mut t = 0.0f32;
        while t < 1.0 {
            t += 0.1;
            let it = 1.0 - t;
            let next = Vec2::new(it * it * self.start.x + 2.0 * t * it * mid.x + t * t * end.x, it * it * self.start.y + 2.0 * t * it * mid.y + t * t * end.y);

            self.canvas.draw_line(start.to_sdlpoint(), next.to_sdlpoint()).unwrap();
            start = next;
        }
        self.start = end;
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        panic!();
        // let mid0 = SdlPoint::new(
        //     x1 as i32 - self.bound.x,
        //     self.bound.h - y1 as i32 + self.bound.y,
        // );
        // self.canvas.draw_line(self.start, mid0).unwrap();
        // let mid = SdlPoint::new(
        //     x2 as i32 - self.bound.x,
        //     self.bound.h - y2 as i32 + self.bound.y,
        // );
        // self.canvas.draw_line(mid0, mid).unwrap();
        // let end = SdlPoint::new(x as i32 - self.bound.x, self.bound.h - y as i32 + self.bound.y);
        // self.canvas.draw_line(mid, end).unwrap();
        // self.start = end;
    }
}

#[derive(Debug)]
pub struct GlyphInfo {
    name: String,
    pub bound: Rect,      //包围盒的x,y最大最小值
    advance_hor: u16,     //水平步进
    advance_ver: u16,     //垂直步进
    bearing_hor: i16,     //原点到包围盒左侧的距离
    pub bearing_ver: i16, //原点到包围盒上侧的距离
    origin_y: Option<i16>,
}

static mut FONT_DATA: Vec<u8> = Vec::new();
pub struct FontObj<'a> {
    // font: SdlFont,  //A方案, 用于渲染布局中的字符
    face: Face<'a>, //B方案, 用于渲染预览中的字符
}
impl<'a> FontObj<'a> {
    pub fn init(filename: &str) -> FontObj<'_> {
        let font_data = std::fs::read(filename).unwrap();
        let num = ttf_parser::fonts_in_collection(&font_data);
        if num.is_none() {
            println!("{} 非TTF字体集合", filename);
        } else {
            println!("{} 是TTF字体集合, 共有字体: {}", filename, num.unwrap());
        }

        unsafe {
            FONT_DATA = font_data;

            let face = match ttf_parser::Face::parse(&FONT_DATA, 0) {
                Ok(f) => f,
                Err(e) => {
                    eprint!("\t解析字体失败. Error: {}.", e);
                    std::process::exit(1);
                }
            };
            FontObj { face }
        }
    }

    #[inline(always)]
    pub fn get_bound(&self) -> SdlRect {
        let rect = self.face.global_bounding_box();
        to_sdlrect(rect)
    }

    #[inline(always)]
    pub fn get_glyphs_num(&self) -> u16 {
        self.face.number_of_glyphs()
    }

    #[inline(always)]
    pub fn get_glyph_id(&self, code: u32) -> Option<GlyphId> {
        let code_point = char::from_u32(code);
        // let code_point = Some('W');
        if code_point.is_none() {
            return None;
        }
        self.face.glyph_index(code_point.unwrap())
    }

    pub fn get_glyph_info(&self, gid: Option<GlyphId>) -> Option<GlyphInfo> {
        if gid.is_none() {
            return None;
        }
        let gid = gid.unwrap();
        Some(GlyphInfo {
            name: self.face.glyph_name(gid).or(Some("none")).unwrap().to_string(),
            bound: self.face.glyph_bounding_box(gid).or_else(|| Some(Rect { x_min: 0, x_max: 0, y_min: 0, y_max: 0 })).unwrap(),
            // ),
            advance_hor: self.face.glyph_hor_advance(gid).unwrap(),
            advance_ver: self.face.glyph_ver_advance(gid).or(Some(0)).unwrap(),
            bearing_hor: self.face.glyph_hor_side_bearing(gid).unwrap(),
            bearing_ver: self.face.glyph_ver_side_bearing(gid).or(Some(0)).unwrap(),
            origin_y: self.face.glyph_y_origin(gid),
            // self.face.glyph_raster_image(gid, 1),
            // self.face.glyph_svg_image(gid),
        })
    }

    pub fn draw_glyph(&self, id: GlyphId, fontdraw: &mut FontDraw) {
        // let res = self.face.glyph_raster_image(id, 256);
        // if res.is_some() {
        //     println!("has raster image in {:?} {:?}", id, res.unwrap());
        // }

        // JPic::from_data(res.unwrap().data, res.unwrap().format);
        // let res = self.face.glyph_svg_image(id);
        // if res.is_some() {
        //     println!("has svg image in {:?}", id);
        // }
        self.face.outline_glyph(id, fontdraw);
    }

    pub fn unit_em(&self) -> u16 {
        self.face.units_per_em()
    }

    // pub fn get_glyphs_img(&self) -> SdlSurface {}

    pub fn debug(&self) {
        let names = self.face.names();
        println!("font info:");
        println!("\tFamily names: {} {:?}", names.len(), names);
        for i in 0..names.len() {
            println!("\t\t{:?}", names.get(i));
        }
        println!("");
        println!("\tNumber of glyphs: {}", self.face.number_of_glyphs());
        println!("");
        println!("\tis bitmap allow(是否允许嵌入位图): {}", self.face.is_bitmap_embedding_allowed());
        println!("\tis monospaced(是否是等宽字体): {}", self.face.is_monospaced());
        println!("\tis subsetting allow: {}", self.face.is_subsetting_allowed());
        println!("\tis Variable(是否是可变字体): {:?}", self.face.is_variable());
        println!("");
        println!("\tis Regular: {}", self.face.is_regular());
        println!("\tis Italic: {}", self.face.is_italic());
        println!("\tis Bold: {}", self.face.is_bold());
        println!("\tis Oblique: {}", self.face.is_oblique());
        // println!("\tPostScript name: {:?}", post_script_name);
        println!("");
        println!("\tUnits per EM: {:?}", self.face.units_per_em());
        println!("\tBaseline: 0 (总是为零)");
        println!("\tAscender: {}", self.face.ascender());
        println!("\tDescender: {}", self.face.descender());
        println!("\tLine gap: {}", self.face.line_gap());
        println!("\tx height: {:?}", self.face.x_height());
        println!("\tCapital height: {:?}", self.face.capital_height());
        println!("\tItalic angle: {:?}", self.face.italic_angle());
        println!("\tVertical ascender: {:?}", self.face.vertical_ascender());
        println!("\tVertical descender: {:?}", self.face.vertical_descender());
        println!("\tVertical line gap: {:?}", self.face.vertical_line_gap());
        println!("");
        println!("\tUnderline: {:?}", self.face.underline_metrics());
        println!("\tWeight: {:?}", self.face.weight());
        println!("\tWidth: {:?}", self.face.width());
        println!("\tHeight: {:?}", self.face.height());
        println!("\tGlobal bbox: {:?}", self.face.global_bounding_box());
        println!("");
        println!("\tStrikeout: {:?}", self.face.strikeout_metrics());
        println!("\tSubscript: {:?}", self.face.subscript_metrics());
        println!("\tSuperscript: {:?}", self.face.superscript_metrics());
        println!("\tPermissions: {:?}", self.face.permissions());
        println!("");
        println!("\tunicode range: {:?}", self.face.unicode_ranges());
    }
}

// fn main() {
//     let mut fo = FontObj::init("d:/web/font/mplus_hzk_13.ttf");
// 	println!("\tglyphs num {}", fo.get_glyphs_num());
//     println!("\tbye");
// }

pub fn to_sdlrect(rect: Rect) -> SdlRect {
    SdlRect::new(rect.x_min as i32, rect.y_min as i32, (rect.x_max - rect.x_min) as u32, (rect.y_max - rect.y_min) as u32)
}
