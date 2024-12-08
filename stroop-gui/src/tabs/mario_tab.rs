use iced::{Element, widget::Text};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{grid, grid_row};
use stroop_rs::containers::sm64_types::types::Mario;

use crate::tab::Message;
use crate::tab::Tab;

#[derive(Debug, Clone)]
pub enum MarioMessage {
    ReadData,
    WriteData,
}
#[derive(Default)]
pub struct MarioTab {
    mario_data: Mario,
}

impl MarioTab {
    pub fn new() -> Self {
        MarioTab {
            mario_data: Mario::default(),
        }
    }
    pub fn update(&mut self, message: MarioMessage) {
        match message {
            MarioMessage::ReadData => {
                //self.mario_data.update_read();
            }
            MarioMessage::WriteData => {
                //self.mario_data.update_write();
            }
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
