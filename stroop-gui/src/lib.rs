use iced::{
    alignment::{Horizontal},
    font,
    widget::{container, text},
    Application, Command, Element, Font, Length, Settings, Theme,
};
use iced_aw::{TabBarStyles, Tabs};
use iced_aw::{grid, grid_row};
use tabs::misc_tab::{MiscMessage, MiscTab};
use tabs::{
    mario_tab::{MarioMessage, MarioTab},
    Tab,
};
use tabs::{State, TabId};

// TODO  organise this file
mod tabs;

#[derive(Clone, Debug)]
pub enum Message {
    TabSelected(TabId),
    Mario(MarioMessage),
    Misc(MiscMessage),
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>)
}

pub enum StroopRS {
    Loading,
    Loaded(tabs::State),
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for StroopRS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (StroopRS, Command<Message>) {
        (
            StroopRS::Loading,
            Command::batch(vec![Command::perform(load(), Message::Loaded)]),
        )
    }

    fn title(&self) -> String {
        format!("stroop-rs") /*add in version*/
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            StroopRS::Loading => {
                if let Message::Loaded(_) = message {
                    *self = StroopRS::Loaded(State {
                        active_tab: TabId::MarioTab,
                        mario_tab: MarioTab::new(),
                        misc_tab: MiscTab::new(),
                    })
                }
            }
            StroopRS::Loaded(state) => match message {
                Message::TabSelected(selected) => state.active_tab = selected,
                Message::Mario(message) => state.mario_tab.update(message),
                Message::Misc(message) => state.misc_tab.update(message),
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            StroopRS::Loading => container(
                text("Loading...")
                    .horizontal_alignment(Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            StroopRS::Loaded(state) => Tabs::new(Message::TabSelected)
                .push(
                    TabId::MarioTab,
                    state.mario_tab.tab_label(),
                    state.mario_tab.view(),
                )
                .push(
                    TabId::MiscTab,
                    state.misc_tab.tab_label(),
                    state.misc_tab.view(),
                )
                .set_active_tab(&state.active_tab)
                .tab_bar_style(TabBarStyles::Default)
                .icon_font(Font::with_name("icons"))
                .tab_bar_position(iced_aw::TabBarPosition::Top)
                .into(),
        }
    }
}
