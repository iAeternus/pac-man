/// 本地化文本结构
#[derive(Debug)]
#[allow(dead_code)]
pub struct LocalizedTexts {
    // UI 文本
    pub game_title: &'static str,
    pub start_button: &'static str,
    pub quit_button: &'static str,
}

/// 英文文本
pub const ENGLISH_TEXTS: LocalizedTexts = LocalizedTexts {
    // UI 文本
    game_title: "Pac-Man",
    start_button: "Start",
    quit_button: "Quit",
};
