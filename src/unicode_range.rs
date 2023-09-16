use jengine::jcolor::JColor;

/// 编码范围(索引, 范围名称, 范围起点值(unicode编码), 范围终点值(unicode编码), 说明)
#[derive(Debug)]
pub struct UnicodeRange(pub u8, pub &'static str, pub u32, pub u32, &'static str);
impl UnicodeRange {
    /// 计算本range应该对应的色彩(条纹格式)
    pub fn color(&self) -> JColor {
        if self.0 % 2 == 0 {
            JColor::new(0.0, 0.75, 0.0, 1.0)
        } else {
            JColor::new(0.75, 0.0, 0.750, 1.0)
        }
    }
	/// 彩虹格式
    pub fn color_rainbow(&self) -> JColor {
        JColor::from_hsl(self.0 as f32 / 168., 1.0, 0.35, 1.0)
    }
	// pub fn info(&self, )->String{
	// 	let out = format!("")
	// }
}
const UNICODE_LAYOUT: [UnicodeRange; 169] = [
    UnicodeRange(0, "Basic Latin", 0x0000, 0x007F, ""),
    UnicodeRange(1, "Latin-1 Supplement", 0x0080, 0x00FF, ""),
    UnicodeRange(2, "Latin Extended-A", 0x0100, 0x017F, ""),
    UnicodeRange(3, "Latin Extended-B", 0x0180, 0x024F, ""),
    UnicodeRange(4, "IPA Extensions", 0x0250, 0x02AF, ""),
    UnicodeRange(4, "Phonetic Extensions", 0x1D00, 0x1D7F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(4, "Phonetic Extensions Supplement", 0x1D80, 0x1DBF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(5, "Spacing Modifier Letters", 0x02B0, 0x02FF, ""),
    UnicodeRange(5, "Modifier Tone Letters", 0xA700, 0xA71F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(6, "Combining Diacritical Marks", 0x0300, 0x036F, ""),
    UnicodeRange(6, "Combining Diacritical Marks Supplement", 0x1DC0, 0x1DFF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(7, "Greek and Coptic", 0x0370, 0x03FF, ""),
    UnicodeRange(8, "Coptic", 0x2C80, 0x2CFF, "Added in OpenType 1.5 for OS/2 version 4. See below for other version differences."),
    UnicodeRange(9, "Cyrillic", 0x0400, 0x04FF, ""),
    UnicodeRange(9, "Cyrillic Supplement", 0x0500, 0x052F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(9, "Cyrillic Extended-A", 0x2DE0, 0x2DFF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(9, "Cyrillic Extended-B", 0xA640, 0xA69F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(10, "Armenian", 0x0530, 0x058F, ""),
    UnicodeRange(11, "Hebrew", 0x0590, 0x05FF, ""),
    UnicodeRange(12, "Vai", 0xA500, 0xA63F, "Added in OpenType 1.5 for OS/2 version 4. See below for other version differences."),
    UnicodeRange(13, "Arabic", 0x0600, 0x06FF, ""),
    UnicodeRange(13, "Arabic Supplement", 0x0750, 0x077F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(14, "NKo", 0x07C0, 0x07FF, "Added in OpenType 1.5 for OS/2 version 4. See below for other version differences."),
    UnicodeRange(15, "Devanagari", 0x0900, 0x097F, ""),
    UnicodeRange(16, "Bengali", 0x0980, 0x09FF, ""),
    UnicodeRange(17, "Gurmukhi", 0x0A00, 0x0A7F, ""),
    UnicodeRange(18, "Gujarati", 0x0A80, 0x0AFF, ""),
    UnicodeRange(19, "Oriya", 0x0B00, 0x0B7F, ""),
    UnicodeRange(20, "Tamil", 0x0B80, 0x0BFF, ""),
    UnicodeRange(21, "Telugu", 0x0C00, 0x0C7F, ""),
    UnicodeRange(22, "Kannada", 0x0C80, 0x0CFF, ""),
    UnicodeRange(23, "Malayalam", 0x0D00, 0x0D7F, ""),
    UnicodeRange(24, "Thai", 0x0E00, 0x0E7F, ""),
    UnicodeRange(25, "Lao", 0x0E80, 0x0EFF, ""),
    UnicodeRange(26, "Georgian", 0x10A0, 0x10FF, ""),
    UnicodeRange(26, "Georgian Supplement", 0x2D00, 0x2D2F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(27, "Balinese", 0x1B00, 0x1B7F, "Added in OpenType 1.5 for OS/2 version 4. See below for other version differences."),
    UnicodeRange(28, "Hangul Jamo", 0x1100, 0x11FF, ""),
    UnicodeRange(29, "Latin Extended Additional", 0x1E00, 0x1EFF, ""),
    UnicodeRange(29, "Latin Extended-C", 0x2C60, 0x2C7F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(29, "Latin Extended-D", 0xA720, 0xA7FF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(30, "Greek Extended", 0x1F00, 0x1FFF, ""),
    UnicodeRange(31, "General Punctuation", 0x2000, 0x206F, ""),
    UnicodeRange(31, "Supplemental Punctuation", 0x2E00, 0x2E7F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(32, "Superscripts And Subscripts", 0x2070, 0x209F, ""),
    UnicodeRange(33, "Currency Symbols", 0x20A0, 0x20CF, ""),
    UnicodeRange(34, "Combining Diacritical Marks For Symbols", 0x20D0, 0x20FF, ""),
    UnicodeRange(35, "Letterlike Symbols", 0x2100, 0x214F, ""),
    UnicodeRange(36, "Number Forms", 0x2150, 0x218F, ""),
    UnicodeRange(37, "Arrows", 0x2190, 0x21FF, ""),
    UnicodeRange(37, "Supplemental Arrows-A", 0x27F0, 0x27FF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(37, "Supplemental Arrows-B", 0x2900, 0x297F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(37, "Miscellaneous Symbols and Arrows", 0x2B00, 0x2BFF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(38, "Mathematical Operators", 0x2200, 0x22FF, ""),
    UnicodeRange(38, "Supplemental Mathematical Operators", 0x2A00, 0x2AFF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(38, "Miscellaneous Mathematical Symbols-A", 0x27C0, 0x27EF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(38, "Miscellaneous Mathematical Symbols-B", 0x2980, 0x29FF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(39, "Miscellaneous Technical", 0x2300, 0x23FF, ""),
    UnicodeRange(40, "Control Pictures", 0x2400, 0x243F, ""),
    UnicodeRange(41, "Optical Character Recognition", 0x2440, 0x245F, ""),
    UnicodeRange(42, "Enclosed Alphanumerics", 0x2460, 0x24FF, ""),
    UnicodeRange(43, "Box Drawing", 0x2500, 0x257F, ""),
    UnicodeRange(44, "Block Elements", 0x2580, 0x259F, ""),
    UnicodeRange(45, "Geometric Shapes", 0x25A0, 0x25FF, ""),
    UnicodeRange(46, "Miscellaneous Symbols", 0x2600, 0x26FF, ""),
    UnicodeRange(47, "Dingbats", 0x2700, 0x27BF, ""),
    UnicodeRange(48, "CJK Symbols And Punctuation", 0x3000, 0x303F, ""),
    UnicodeRange(49, "Hiragana", 0x3040, 0x309F, ""),
    UnicodeRange(50, "Katakana", 0x30A0, 0x30FF, ""),
    UnicodeRange(50, "Katakana Phonetic Extensions", 0x31F0, 0x31FF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(51, "Bopomofo", 0x3100, 0x312F, ""),
    UnicodeRange(51, "Bopomofo Extended", 0x31A0, 0x31BF, "Added in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(52, "Hangul Compatibility Jamo", 0x3130, 0x318F, ""),
    UnicodeRange(53, "Phags-pa", 0xA840, 0xA87F, "Added in OpenType 1.5 for OS/2 version 4. See below for other version differences."),
    UnicodeRange(54, "Enclosed CJK Letters And Months", 0x3200, 0x32FF, ""),
    UnicodeRange(55, "CJK Compatibility", 0x3300, 0x33FF, ""),
    UnicodeRange(56, "Hangul Syllables", 0xAC00, 0xD7AF, ""),
    UnicodeRange(57, "Non-Plane 0", 0x10000, 0x10FFFF, "Implies at least one character beyond the Basic Multilingual Plane. First assigned in OpenType 1.3 for OS/2 version 2."),
    UnicodeRange(58, "Phoenician", 0x10900, 0x1091F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(59, "CJK Unified Ideographs", 0x4E00, 0x9FFF, ""),
    UnicodeRange(59, "CJK Radicals Supplement", 0x2E80, 0x2EFF, "Added in OpenType 1.3 for OS/2 version 2."),
    UnicodeRange(59, "Kangxi Radicals", 0x2F00, 0x2FDF, "Added in OpenType 1.3 for OS/2 version 2."),
    UnicodeRange(59, "Ideographic Description Characters", 0x2FF0, 0x2FFF, "Added in OpenType 1.3 for OS/2 version 2."),
    UnicodeRange(59, "CJK Unified Ideographs Extension A", 0x3400, 0x4DBF, "Added in OpenType 1.3 for OS/2 version 2."),
    UnicodeRange(59, "CJK Unified Ideographs Extension B", 0x20000, 0x2A6DF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(59, "Kanbun", 0x3190, 0x319F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(60, "Private Use Area (plane 0)", 0xE000, 0xF8FF, ""),
    UnicodeRange(61, "CJK Strokes", 0x31C0, 0x31EF, "Range added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(61, "CJK Compatibility Ideographs", 0xF900, 0xFAFF, ""),
    UnicodeRange(61, "CJK Compatibility Ideographs Supplement", 0x2F800, 0x2FA1F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(62, "Alphabetic Presentation Forms", 0xFB00, 0xFB4F, ""),
    UnicodeRange(63, "Arabic Presentation Forms-A", 0xFB50, 0xFDFF, ""),
    UnicodeRange(64, "Combining Half Marks", 0xFE20, 0xFE2F, ""),
    UnicodeRange(65, "Vertical Forms", 0xFE10, 0xFE1F, "Range added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(65, "CJK Compatibility Forms", 0xFE30, 0xFE4F, ""),
    UnicodeRange(66, "Small Form Variants", 0xFE50, 0xFE6F, ""),
    UnicodeRange(67, "Arabic Presentation Forms-B", 0xFE70, 0xFEFF, ""),
    UnicodeRange(68, "Halfwidth And Fullwidth Forms", 0xFF00, 0xFFEF, ""),
    UnicodeRange(69, "Specials", 0xFFF0, 0xFFFF, ""),
    UnicodeRange(70, "Tibetan", 0x0F00, 0x0FFF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(71, "Syriac", 0x0700, 0x074F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(72, "Thaana", 0x0780, 0x07BF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(73, "Sinhala", 0x0D80, 0x0DFF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(74, "Myanmar", 0x1000, 0x109F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(75, "Ethiopic", 0x1200, 0x137F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(75, "Ethiopic Supplement", 0x1380, 0x139F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(75, "Ethiopic Extended", 0x2D80, 0x2DDF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(76, "Cherokee", 0x13A0, 0x13FF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(77, "Unified Canadian Aboriginal Syllabics", 0x1400, 0x167F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(78, "Ogham", 0x1680, 0x169F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(79, "Runic", 0x16A0, 0x16FF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(80, "Khmer", 0x1780, 0x17FF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(80, "Khmer Symbols", 0x19E0, 0x19FF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(81, "Mongolian", 0x1800, 0x18AF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(82, "Braille Patterns", 0x2800, 0x28FF, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(83, "Yi Syllables", 0xA000, 0xA48F, "First assigned in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(83, "Yi Radicals", 0xA490, 0xA4CF, "Added in OpenType 1.3, extending OS/2 version 2."),
    UnicodeRange(84, "Tagalog", 0x1700, 0x171F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(84, "Hanunoo", 0x1720, 0x173F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(84, "Buhid", 0x1740, 0x175F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(84, "Tagbanwa", 0x1760, 0x177F, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(85, "Old Italic", 0x10300, 0x1032F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(86, "Gothic", 0x10330, 0x1034F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(87, "Deseret", 0x10400, 0x1044F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(88, "Byzantine Musical Symbols", 0x1D000, 0x1D0FF, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(88, "Musical Symbols", 0x1D100, 0x1D1FF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(88, "Ancient Greek Musical Notation", 0x1D200, 0x1D24F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(89, "Mathematical Alphanumeric Symbols", 0x1D400, 0x1D7FF, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(90, "Private Use (plane 15)", 0xF0000, 0xFFFFD, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(90, "Private Use (plane 16)", 0x100000, 0x10FFFD, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(91, "Variation Selectors", 0xFE00, 0xFE0F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(91, "Variation Selectors Supplement", 0xE0100, 0xE01EF, "Added in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(92, "Tags", 0xE0000, 0xE007F, "First assigned in OpenType 1.4 for OS/2 version 3."),
    UnicodeRange(93, "Limbu", 0x1900, 0x194F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(94, "Tai Le", 0x1950, 0x197F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(95, "New Tai Lue", 0x1980, 0x19DF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(96, "Buginese", 0x1A00, 0x1A1F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(97, "Glagolitic", 0x2C00, 0x2C5F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(98, "Tifinagh", 0x2D30, 0x2D7F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(99, "Yijing Hexagram Symbols", 0x4DC0, 0x4DFF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(100, "Syloti Nagri", 0xA800, 0xA82F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(101, "Linear B Syllabary", 0x10000, 0x1007F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(101, "Linear B Ideograms", 0x10080, 0x100FF, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(101, "Aegean Numbers", 0x10100, 0x1013F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(102, "Ancient Greek Numbers", 0x10140, 0x1018F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(103, "Ugaritic", 0x10380, 0x1039F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(104, "Old Persian", 0x103A0, 0x103DF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(105, "Shavian", 0x10450, 0x1047F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(106, "Osmanya", 0x10480, 0x104AF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(107, "Cypriot Syllabary", 0x10800, 0x1083F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(108, "Kharoshthi", 0x10A00, 0x10A5F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(109, "Tai Xuan Jing Symbols", 0x1D300, 0x1D35F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(110, "Cuneiform", 0x12000, 0x123FF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(110, "Cuneiform Numbers and Punctuation", 0x12400, 0x1247F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(111, "Counting Rod Numerals", 0x1D360, 0x1D37F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(112, "Sundanese", 0x1B80, 0x1BBF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(113, "Lepcha", 0x1C00, 0x1C4F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(114, "Ol Chiki", 0x1C50, 0x1C7F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(115, "Saurashtra", 0xA880, 0xA8DF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(116, "Kayah Li", 0xA900, 0xA92F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(117, "Rejang", 0xA930, 0xA95F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(118, "Cham", 0xAA00, 0xAA5F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(119, "Ancient Symbols", 0x10190, 0x101CF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(120, "Phaistos Disc", 0x101D0, 0x101FF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(121, "Carian", 0x102A0, 0x102DF, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(121, "Lycian", 0x10280, 0x1029F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(121, "Lydian", 0x10920, 0x1093F, "Added in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(122, "Domino Tiles", 0x1F030, 0x1F09F, "First assigned in OpenType 1.5 for OS/2 version 4."),
    UnicodeRange(122, "Mahjong Tiles", 0x1F000, 0x1F02F, "First assigned in OpenType 1.5 for OS/2 version 4."),
];
//123-127	Reserved for process-internal usage

pub struct UnicodeRangeMgr {
    rangeid: usize,                     //上一次查询所在范围在列表中的索引
    ranges: Vec<&'static UnicodeRange>, //范围集合(以每个范围的起点值排序))
}

impl UnicodeRangeMgr {
    pub fn init() -> Self {
        let mut out = Vec::with_capacity(UNICODE_LAYOUT.len());
        for i in 0..UNICODE_LAYOUT.len() {
            if UNICODE_LAYOUT[i].0 == 57 {
                continue;
            }
            out.push(&UNICODE_LAYOUT[i]);
        }

        out.sort_by(|a, b| a.2.cmp(&b.2));
        for i in 0..out.len() {
            println!("{:?}", out[i]);
        }

        Self { rangeid: 0, ranges: out }
    }

	pub fn reset(&mut self){
		self.rangeid = 0;
	}

    /// 计算某字符id(unicode编码)所在的范围, 根据上次的结果搜索
    pub fn range(&mut self, charid: u32) -> Option<&'static UnicodeRange> {
        let range = self.ranges[self.rangeid];
        if charid < range.2 {
            return None;
        }
        if charid <= range.3 {
            return Some(range);
        }
        self.rangeid += 1;
        if self.rangeid >= self.ranges.len() {
            return None;
        }
        self.range(charid)
    }

    /// 计算某字符id(unicode编码)所在的范围, 全局搜索
    pub fn range_any(&mut self, charid: u32) -> Option<&'static UnicodeRange> {
        for i in 0..self.ranges.len() {
            let range = self.ranges[i];
            if range.2 <= charid && charid <= range.3 {
				// println!("rangeid {}", self.rangeid);
                return Some(range);
            }
        }
        None
    }

    pub fn debug(&self) {
        for i in 0..self.ranges.len() {
            let range = self.ranges[i];
            println!("{:3} - id {:3}, name: {:45}, start {:7}, end {}", i, range.0, range.1, range.2, range.3);
        }
    }
}
