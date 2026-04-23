
use iced::widget::container;
use iced::{Color, Background, Border};
use iced_core::gradient::{Linear, ColorStop};
use iced::Radians;

pub fn styl_kontenera<'a>(
    warunek: bool,
    kolor: (f32, f32, f32),
) -> impl Fn(&iced::Theme) -> container::Style + 'a {
    move |_theme| {
        if warunek {
            let mut stopsy = [None; 8];
            stopsy[0] = Some(ColorStop {
                offset: 0.5,
                color: Color {
                    a: 0.0,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });
            stopsy[1] = Some(ColorStop {
                offset: 1.0,
                color: Color {
                    a: 0.5,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });

            container::Style {
                background: Some(Background::Gradient(
                    iced_core::gradient::Gradient::Linear(Linear {
                        angle: Radians(0.0),
                        stops: stopsy,
                    }),
                )),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                ..container::Style::default()
            }
        } else {
            // Stan gdy warunek jest fałszywy (np. przezroczysty lub biały)
            container::Style {
                background: Some(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 }.into()),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color { a: 0.1, ..Color::WHITE },
                },
                ..container::Style::default()
            }
        }
    }
}