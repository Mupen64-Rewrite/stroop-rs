use crate::tab::Message;
use crate::tab::Tab;
use iced::{widget::Text, Element};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{grid, grid_row};
use stroop_rs::containers::sm64_types::types::Mario;
use stroop_rs::map_file::MapFile;

#[derive(Debug, Clone)]
pub enum MarioMessage {
    ReadData,
    WriteData,
    ReceiveData,
}
#[derive(Default)]
pub struct MarioTab {
    map_file: MapFile,
    mario_data: Mario,
}

impl MarioTab {
    pub fn update(&mut self, message: MarioMessage) {
        match message {
            MarioMessage::ReadData => {
                /* self.mario_data.update_read(
                    &self.map_file,
                   &self.emulator? not sure here
                ).unwrap() */
            }
            MarioMessage::WriteData => {
                //self.mario_data.update_write();
            }
            _ => {}
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
        let data = self.mario_data;
        let mut grid = grid!(
            grid_row!(Text::new(format!("Mario X: {}", (*data.get_pos()).x))),
            grid_row!(Text::new(format!("Mario Y: {}", (*data.get_pos()).y))),
            grid_row!(Text::new(format!("Mario Z: {}", (*data.get_pos()).z))),
        );

        let mut content: Element<'_, MarioMessage> = Element::from(grid);
        content.map(Message::Mario)
    }
}
