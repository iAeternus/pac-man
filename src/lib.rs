mod game;
mod fonts;
mod localization;

pub use game::{setup_menu_ui, StartButton, QuitButton};
pub use fonts::{FontAssets, load_font_assets};
pub use localization::LanguageSettings;