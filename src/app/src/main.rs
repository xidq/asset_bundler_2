#![allow(warnings)]
// #![windows_subsystem = "windows"]
use ui_iced::ui::program::Program;

use enumy::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_MONOSPACE_LATIN, FONT_THAI};

#[tokio::main]
async fn main() -> iced::Result {
    iced::application(Program::new, Program::update, Program::view)
        .title("Bundler")
        .subscription(Program::subscription)
        .font(FONT_DEFAULT)
        .font(FONT_KOREAN)
        .font(FONT_JAPANESE)
        .font(FONT_THAI)
        .font(FONT_MONOSPACE_LATIN)
        .run()
}
