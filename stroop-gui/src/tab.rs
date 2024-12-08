use crate::tabs::mario_tab::{MarioMessage, MarioTab};
use crate::tabs::misc_tab::{MiscMessage, MiscTab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Text};
use iced::{Element, Length};
use iced_aw::{TabBarPosition, TabLabel, Tabs};

const HEADER_SIZE: u16 = 30;
#[derive(Default)]
pub struct StroopGui {
    active_tab: TabId,
    mario_tab: MarioTab,
    misc_tab: MiscTab,
}
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum TabId {
    #[default]
    Mario,
    Misc,
    Map,
}
#[derive(Debug)]
pub enum Message {
    TabSelected(TabId),
    Mario(MarioMessage),
    Misc(MiscMessage),
    TabClosed(TabId),
}
impl StroopGui {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab_id) => {
                self.active_tab = tab_id;
            }
            Message::Mario(message) => {
                self.mario_tab.update(message);
            }
            Message::Misc(message) => {
                self.misc_tab.update(message);
            }
            Message::TabClosed(tab_id) => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Tabs::new(Message::TabSelected)
            .on_close(Message::TabClosed)
            .push(
                TabId::Mario,
                self.mario_tab.tab_label(),
                self.mario_tab.view(),
            )
            .push(TabId::Misc, self.misc_tab.tab_label(), self.misc_tab.view())
            .set_active_tab(&self.active_tab)
            .tab_bar_position(TabBarPosition::Top)
            .into()
    }
}

pub trait Tab {
    type Message;
    fn title(&self) -> String;
    fn tab_label(&self) -> TabLabel;
    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content())
            .spacing(20)
            .align_x(iced::Alignment::Center);
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(10)
            .into()
    }
    fn content(&self) -> Element<'_, Self::Message>;
}
