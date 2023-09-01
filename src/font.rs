use jengine::jcolor::JColor;
use jengine::jfont::SdlFont;
use jengine::jrenderer::{SdlCanvas, SdlPoint, SdlRect};
use jengine::jtexture::SdlSurface;
use ttf_parser::{Face, GlyphId, OutlineBuilder};

pub struct FontDraw<'a> {
    canvas: &'a mut SdlCanvas,
    rect: SdlRect,
    first: SdlPoint,
    start: SdlPoint,
}
impl<'a> FontDraw<'a> {
    pub fn new(canvas: &'a mut SdlCanvas) -> Self {
        Self {
            canvas,
            first: SdlPoint::new(0, 0),
            start: SdlPoint::new(0, 0),
            rect: SdlRect::new(0, 0, 0, 0),
        }
    }
    pub fn color(&mut self, color: &JColor) {
        self.canvas.set_draw_color(color.to_sdlcolor());
    }
    pub fn rect(&mut self, rect: SdlRect) {
        self.rect = rect;
    }
    pub fn draw(&mut self) {
        println!("draw");
    }
}
impl<'a> OutlineBuilder for FontDraw<'a> {
    fn close(&mut self) {
        // self.canvas.draw_line(self.start, self.first).unwrap();
        // println!("close");
    }
    fn move_to(&mut self, x: f32, y: f32) {
        (self.first.x, self.first.y) = (x as i32 + self.rect.x, 256 - y as i32 + self.rect.y);
        self.start = self.first;
        // (self.start.x, self.start.y) = (x as i32+self.x, 256-y as i32+self.x);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let end = SdlPoint::new(x as i32 + self.rect.x, 256 - y as i32 + self.rect.y);
        self.canvas.draw_line(self.start, end).unwrap();
        self.start = end;
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let mid = SdlPoint::new(x1 as i32 + self.rect.x, 256 - y1 as i32 + self.rect.y);
        let end = SdlPoint::new(x as i32 + self.rect.x, 256 - y as i32 + self.rect.y);
        self.canvas.draw_line(self.start, mid).unwrap();
        self.canvas.draw_line(mid, end).unwrap();
        self.start = end;
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let mid0 = SdlPoint::new(x1 as i32 + self.rect.x, 256 - y1 as i32 + self.rect.y);
        self.canvas.draw_line(self.start, mid0).unwrap();
        let mid = SdlPoint::new(x2 as i32 + self.rect.x, 256 - y2 as i32 + self.rect.y);
        self.canvas.draw_line(mid0, mid).unwrap();
        let end = SdlPoint::new(x as i32 + self.rect.x, 256 - y as i32 + self.rect.y);
        self.canvas.draw_line(mid, end).unwrap();
        self.start = end;
    }
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
        SdlRect::new(
            rect.x_min as i32,
            rect.y_min as i32,
            (rect.x_max - rect.x_min) as u32,
            (rect.y_max - rect.y_min) as u32,
        )
    }

    #[inline(always)]
    pub fn get_glyphs_num(&self) -> u16 {
        self.face.number_of_glyphs()
    }

    #[inline(always)]
    pub fn get_glyphs_id(&self, code: u32) -> Option<GlyphId> {
        // let code_point = char::from_u32(code);
        let code_point = Some('W');
        if code_point.is_none() {
            return None;
        }
        self.face.glyph_index(code_point.unwrap())
    }

    pub fn draw_glyph(&self, id: GlyphId, fontdraw: &mut dyn OutlineBuilder) {
        self.face.outline_glyph(id, fontdraw);
    }

    // pub fn get_glyphs_img(&self) -> SdlSurface {}

    pub fn debug(&self) {
        let names = self.face.names();
        println!("font info:");
        println!("\tFamily names: {} {:?}", names.len(), names);
        for i in 0..names.len() {
            println!("\t\t{:?}", names.get(i));
        }
        println!("\tNumber of glyphs: {}", self.face.number_of_glyphs());
        println!("");
        println!("\tRegular: {}", self.face.is_regular());
        println!("\tVariable: {:?}", self.face.is_variable());
        println!("\tItalic: {}", self.face.is_italic());
        println!("\tBold: {}", self.face.is_bold());
        println!("\tOblique: {}", self.face.is_oblique());
        println!("");
        // println!("\tPostScript name: {:?}", post_script_name);
        println!("\tUnits per EM: {:?}", self.face.units_per_em());
        println!("\tAscender: {}", self.face.ascender());
        println!("\tDescender: {}", self.face.descender());
        println!("\tLine gap: {}", self.face.line_gap());
        println!("\tGlobal bbox: {:?}", self.face.global_bounding_box());
        println!("\tUnderline: {:?}", self.face.underline_metrics());
        println!("\tX height: {:?}", self.face.x_height());
        println!("\tWidth: {:?}", self.face.width());
        println!("\tWeight: {:?}", self.face.weight());
        println!("");
        println!("\tStrikeout: {:?}", self.face.strikeout_metrics());
        println!("\tSubscript: {:?}", self.face.subscript_metrics());
        println!("\tSuperscript: {:?}", self.face.superscript_metrics());
        println!("\tPermissions: {:?}", self.face.permissions());
        println!("");
    }
}

// fn main() {
//     let mut fo = FontObj::init("d:/web/font/mplus_hzk_13.ttf");
// 	println!("\tglyphs num {}", fo.get_glyphs_num());
//     println!("\tbye");
// }
