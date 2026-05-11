use iced::Element;
use iced::widget::{container, Container};
use iced_core::Background::{ Gradient};

use iced_core::{Background, Color, Length, Radians};
use iced_core::gradient::{ColorStop, Linear};
use crate::ui::wiadomosci::message_ui::Message;

pub fn oddzielacz_pionowy<'a>() -> Element<'a, Message> {
    Container::new("")
        .style( |_theme|{

            let mut stopsy = [None; 8];
            stopsy[0] = Some(ColorStop {
                offset: 0.0,
                color: Color {
                    a: 0.0,
                    ..Color::WHITE
                },
            });
            stopsy[1] = Some(ColorStop {
                offset: 0.5,
                color: Color {
                    a: 0.1,
                    ..Color::WHITE
                },
            });
            stopsy[2] = Some(ColorStop {
                offset: 1.0,
                color: Color {
                    a: 0.0,
                    ..Color::WHITE
                },
            });

            container::Style{
                background: Some(Background::Gradient(
                    iced_core::gradient::Gradient::Linear(Linear {
                        angle: Radians(0.0),
                        stops: stopsy,
                    }),
                )),
                ..Default::default()
            }
        }).width(2.)
        .height(Length::Fill)
        .into()
}
pub fn oddzielacz_poziomy<'a>() -> Container<'a, Message> {
    container("")
        .width(Length::Fill)
        .height(Length::Fixed(2.))
        .style(|_theme| container::Style {
            background: Some(Color::from_rgba(1., 1., 1., 0.2).into()),
            ..container::Style::default()
        })
}
