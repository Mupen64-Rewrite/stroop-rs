use iced::{
    alignment::{Horizontal, Vertical},
    widget::Text,
    Element,
};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{grid, grid_row};
use stroop_rs::containers::sm64_types::Mario;

use crate::Message;

use super::Tab;

#[derive(Debug, Clone)]
pub enum MarioMessage {
    ReadData,
    WriteData,
}
pub struct MarioStruct {
    mario_data: Mario,
}
pub struct MarioTab {
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    column_spacing: f32,
    row_spacing: f32,
    fill_width: bool,
    fill_height: bool,
    padding: f32,
}

impl MarioTab {
    pub fn new() -> Self {
        MarioTab {
            horizontal_alignment: (Horizontal::Center),
            vertical_alignment: (Vertical::Center),
            column_spacing: 0.0,
            row_spacing: 0.0,
            fill_width: false,
            fill_height: false,
            padding: 0.0,
        }
    }
    pub fn update(&mut self, message: MarioMessage) {
        match message {
            MarioMessage::ReadData => { /*self.mario_data.read()*/ }
            MarioMessage::WriteData => { /*self.mario_data.write()*/ }
        }
    }
}

impl Tab for MarioTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Mario")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        //MarioTab::view(&mut Self);

        let mut grid = grid!(
            grid_row!(Text::new("Mario Data1")),
            grid_row!(Text::new("Mario Data2")),
            grid_row!(Text::new("Mario Data3")),
        );

        let mut content: Element<'_, MarioMessage> = Element::from(grid);
        content.map(Message::Mario)
    }
}
