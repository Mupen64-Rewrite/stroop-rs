use iced_aw::TabLabel;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    Application, Element, Length,
};
use crate::tabs::mario_tab::MarioTab;
use crate::tabs::misc_tab::MiscTab;

pub(crate) mod mario_tab;
pub(crate) mod misc_tab;

const HEADER_SIZE: u16 = 16;
const TAB_PADDING: u16 = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TabId {
    MarioTab,
    MiscTab,
}

pub struct State {
    pub active_tab: TabId,
    pub mario_tab: MarioTab,
    pub misc_tab: MiscTab,
}

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}