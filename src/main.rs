#![allow(warnings)]

use ui::program::Program;
use ui::program::czcionki::FONT_DEFAULT;
use crate::ui::program::czcionki::{FONT_JAPANESE, FONT_KOREAN, FONT_MONOSPACE_LATIN, FONT_THAI};

mod io;
mod kompresja;
mod no_sync;
mod szyfr;
mod test_compression;
mod ui;
mod foty;
mod laczenie_fot;

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
