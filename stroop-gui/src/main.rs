use crate::tab::StroopGui;
use iced_aw::iced_fonts;
pub mod tab;
mod tabs;

fn main() -> iced::Result {
    iced::application("Stroop GUI", StroopGui::update, StroopGui::view)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .run()
}
