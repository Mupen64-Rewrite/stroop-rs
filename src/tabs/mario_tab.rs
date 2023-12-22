use iced::{
  widget::{Column, Container, Text},
  Element,
};
use iced_aw::tab_bar::TabLabel;

use crate::{Message, Tab};
use crate::containers::sm64_types;
#[derive(Debug, Clone)]
pub enum MarioMessage {
  UpdateData,
}

pub struct MarioData {
  data: sm64_types::BaseType
  // data from/for grid !!wTODO
}
pub struct MarioTab {
  pub mario_data: MarioData,
}

impl MarioTab {
  pub fn new() -> Self {
    MarioTab { mario_data: MarioData{ data: sm64_types::BaseType::gMarioStates } }
  }
  pub fn update(&mut self, message: MarioMessage) {
    match message {
      MarioMessage::UpdateData => { /*self.mario_data.map()*/ }
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
    let content: Element<'_, MarioMessage> = Container::new(
      Column::new()
          .push(Text::new("marioData")
          )
        )
        .into();

    content.map(Message::Mario)
  }
}