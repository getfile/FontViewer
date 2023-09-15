pub mod font;
pub mod layout;
pub mod preview;
pub mod unicode_layout;

use std::default;

use font::FontObj;
use jengine::jcolor::JColor;
use jengine::jengine::{IApp, JEngine, JEngineArgs, SdlEvent, SdlResult, SdlWindowEvent};
use jengine::jfont::{FontMgr, SdlFontStyle};
use jengine::jinput::{SdlKeycode, SdlKeymod};
use layout::CharsLayout;
use preview::CharPreview;

struct App<'a> {
    layout: CharsLayout,
    prev: CharPreview,
    font_obj: Option<FontObj<'a>>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            layout: CharsLayout::default(),
            prev: CharPreview::default(),
            font_obj: None,
        }
    }
}

impl<'a> IApp for App<'a> {
    fn on_init(&mut self, _engine: &mut JEngine) -> SdlResult {
        // let mut fontmgr = FontMgr::init();
        // fontmgr.setfont("d:/web/font/Code_8X8.ttf", 12, SdlFontStyle::BOLD);

        self.layout.init(_engine);
        // self.font_obj = Some(FontObj::init("d:/web/font/VonwaonBitmap-12px.ttf"));
        // self.font_obj = Some(FontObj::init("d:/web/font/mplus_hzk_13.ttf"));
        self.font_obj = Some(FontObj::init("d:/web/font/Small SimSun.ttf"));
        if self.font_obj.is_some() {
            self.font_obj.as_ref().unwrap().debug();
        }
        self.prev.init(_engine, self.font_obj.as_mut().unwrap());
        Ok(())
    }

    fn on_draw(&mut self, _engine: &mut JEngine) -> SdlResult {
        self.layout.draw(_engine);
        self.prev.draw(_engine);
        Ok(())
    }
    fn on_update(&mut self, _engine: &mut JEngine) -> SdlResult {
        self.layout.update(_engine, self.font_obj.as_mut().unwrap());
        Ok(())
    }
    fn on_event(&mut self, _engine: &mut JEngine, _event: SdlEvent) -> SdlResult {
        // println!("{:?}", _event);
        match _event {
            SdlEvent::Window { win_event: SdlWindowEvent::SizeChanged(wid, hei), .. } => {
                self.layout.set_layout_size(_engine);
                self.layout.init_cursor(_engine);
                println!("resize {}, {}", wid, hei);
            }
            SdlEvent::MouseMotion { x, y, .. } => {
                self.prev.set_side(x);

                let ch = self.layout.charid(x, y);
                self.prev.update(_engine, self.font_obj.as_mut().unwrap(), ch);
            }
            SdlEvent::MouseButtonDown { x, y, .. } => {
                self.layout.set_charid(_engine, x, y);
            }
            SdlEvent::KeyDown { keycode, keymod, .. } => {
                match keycode.unwrap() {
                    SdlKeycode::J => {
                        self.layout.line_move(_engine, if keymod == SdlKeymod::LSHIFTMOD { 10 } else { 1 });
                    }
                    SdlKeycode::K => {
                        self.layout.line_move(_engine, if keymod == SdlKeymod::LSHIFTMOD { -10 } else { -1 });
                    }
                    SdlKeycode::H => {
                        self.layout.char_move(_engine, if keymod == SdlKeymod::LSHIFTMOD { -10 } else { -1 });
                    }
                    SdlKeycode::L => {
                        self.layout.char_move(_engine, if keymod == SdlKeymod::LSHIFTMOD { 10 } else { 1 });
                    }
                    SdlKeycode::Home => {
                        self.layout.page_home(_engine);
                    }
                    SdlKeycode::End => {
                        self.layout.page_end(_engine);
                    }
                    SdlKeycode::PageUp => {
                        self.layout.page_up(_engine);
                    }
                    SdlKeycode::PageDown => {
                        self.layout.page_down(_engine);
                    }
                    SdlKeycode::U => {
                        self.layout.set_char_size(_engine, false);
                    }
                    SdlKeycode::D => {
                        self.layout.set_char_size(_engine, true);
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        Ok(())
    }
}

fn main() {
    JEngineArgs {
        width: 768,
        height: 800,
        resize: true,
        title: "字体查看器".to_string(),
        bg: JColor::new(0.3, 0.0, 0.0, 1.0),
        ..JEngineArgs::default()
    }
    .build()
    .unwrap()
    .run(&mut App::default())
    .unwrap();
}
