use iced::{
    widget::{Column, Container, Text},
    Element,
};
use iced_aw::tab_bar::TabLabel;

use crate::{Message, Tab};

#[derive(Debug, Clone)]
pub enum MiscMessage {
    MiscData,
}
pub struct MiscData {
    // data from/for grid !!TODO
}
pub struct MiscTab {
    pub misc_data: i32,
}

impl MiscTab {
    pub fn new() -> Self { MiscTab { misc_data: 0 } }
    pub fn update(&mut self, message: MiscMessage) {}
}

impl Tab for MiscTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Misc")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, MiscMessage> = Container::new(
            Column::new()
                .push(Text::new("miscData")
                )
        ).into();

        content.map(Message::Misc)
    }
}