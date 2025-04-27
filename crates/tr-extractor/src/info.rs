/// Supported language
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lang {
    English,
    SpanishLa,
    German,
    French,
    Italian,
    Polish,
    PortugueseBr,
    Japanese,
    Russian,
    ChineseSimple,
    Korean,
    Turkish,
}

impl Lang {
    pub fn file_name(self) -> &'static str {
        match self {
            Lang::English => "english",
            Lang::SpanishLa => "spanish_la",
            Lang::German => "german",
            Lang::French => "french",
            Lang::Italian => "italian",
            Lang::Polish => "polish",
            Lang::PortugueseBr => "portuguese_br",
            Lang::Japanese => "japanese",
            Lang::Russian => "russian",
            Lang::ChineseSimple => "chinese_simple",
            Lang::Korean => "korean",
            Lang::Turkish => "turkish",
        }
    }
}
