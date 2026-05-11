use iced::widget::{Container, container};
use iced_core::{Color, Length};
use crate::ui::wiadomosci::message_ui::Message;

pub fn ui_standard_oddzielacz<'a>() -> Container<'a, Message> {
    container("")
        .width(Length::Fill)
        .height(Length::Fixed(2.))
        .style(|_theme| container::Style {
            background: Some(Color::from_rgba(1., 1., 1., 0.2).into()),
            ..container::Style::default()
        })
}
