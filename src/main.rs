pub mod font;
pub mod layout;
pub mod preview;

use font::FontObj;
use jengine::jengine::{IApp, JEngine, JEngineArgs, SdlEvent, SdlResult, SdlWindowEvent};
use jengine::jfont::{FontMgr, SdlFontStyle};
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
        let mut fontmgr = FontMgr::init();
        // fontmgr.setfont("d:/web/font/Code_8X8.ttf", 12, SdlFontStyle::BOLD);

        self.layout.init(_engine);
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
        if let SdlEvent::Window {
            win_event: SdlWindowEvent::SizeChanged(wid, hei),
            ..
        } = _event
        {
            self.layout.resize(_engine);
            // self.prev.init_tex(_engine);
            println!("resize {}, {}", wid, hei);
        } else if let SdlEvent::MouseMotion { x, y, .. } = _event {
            let ch = x / 3 + y / 3 * 256;
            self.prev
                .update(_engine, self.font_obj.as_mut().unwrap(), ch as u32);
        }

        Ok(())
    }
}

fn main() {
    let mut args = JEngineArgs::default();
    args.width = 768;
    args.height = 800;
    // args.resize = true;
    args.title = "字体查看器".to_string();
    args.bg.0[0] = 0.3;

    args.build().unwrap().run(&mut App::default()).unwrap();
}
