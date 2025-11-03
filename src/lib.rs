mod game;
mod fonts;
mod localization;

pub use game::GameState;
pub use game::setup_menu_ui;
pub use fonts::{FontAssets, load_font_assets};
pub use localization::LanguageSettings;