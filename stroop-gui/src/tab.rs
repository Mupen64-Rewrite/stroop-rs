use crate::tabs::mario_tab::{MarioMessage, MarioTab};
use crate::tabs::misc_tab::{MiscMessage, MiscTab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Text};
use iced::{Element, Length, Task, Theme, widget};
use iced_aw::{TabBarPosition, TabLabel, Tabs};
use stroop_emu_mem::StaticMemoryEmulator;
use stroop_rs::map_file::MapFile;

const HEADER_SIZE: u16 = 30;

#[derive(Default)]
pub struct StroopGui {
    emulator: Option<StaticMemoryEmulator>,
    map_file: Option<MapFile>,
    theme: iced::Theme,
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
    ThemeChange(iced::Theme),
    TabSelected(TabId),
    Mario(MarioMessage),
    Misc(MiscMessage),
    TabClosed(TabId),
    ConnectEmulator(u32),
}

impl StroopGui {
    pub fn new() -> (Self, Task<Message>) {
        //Load the config file here
        //self.map_file = Some(MapFile::new("config.json").unwrap());

        //Write these settings to a file, possibly using serde, to save the user's preferences
        (
            StroopGui {
                emulator: None,
                map_file: None,
                theme: iced::Theme::Dark,
                active_tab: TabId::Mario,
                mario_tab: MarioTab::default(),
                misc_tab: MiscTab::default(),
            },
            Task::batch([widget::focus_next()]),
        )
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ConnectEmulator(process_id) => {
                self.emulator =
                    Some(StaticMemoryEmulator::new(0, false, &process_id.to_string()).unwrap());
            }
            Message::TabSelected(tab_id) => {
                self.active_tab = tab_id;
            }
            Message::Mario(message) => {
                self.mario_tab.update(message);
            }
            Message::Misc(message) => {
                self.misc_tab.update(message);
            }
            Message::ThemeChange(theme) => {
                self.theme = theme;
            }
            Message::TabClosed(tab_id) => if self.active_tab == tab_id {},
            _ => {}
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
    pub(crate) fn theme(&self) -> Theme {
        self.theme.clone()
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
