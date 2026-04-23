use crate::ui::program_pomniejsze::kolory::{
    KOLOR_CZCIONKI_JASNY, KOLOR_CZCIONKI_SREDNI, KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
};
use iced::widget::button;
use iced::{Border, Color};
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians};

pub fn styl_przycisków<'a>(
    warunek: bool,
    warunek2: bool,
    kolor: (f32, f32, f32),
) -> impl Fn(&iced::Theme, button::Status) -> button::Style + 'a {
    move |_theme, _status| {
        if let button::Status::Hovered = _status {
            return button::Style {
                background: Some(
                    Color {
                        r: kolor.0,
                        g: kolor.1,
                        b: kolor.2,
                        a: 0.2, // Twoja przezroczystość dla hover
                    }
                    .into(),
                ),
                text_color: KOLOR_CZCIONKI_JASNY,
                border: Border {
                    radius: 5.0.into(),
                    ..Border::default()
                },
                ..button::Style::default()
            };
        }
        // if warunek_konieczny{
        if warunek {
            let mut stopsy = [None; 8];
            stopsy[0] = Some(ColorStop {
                offset: 0.5, // Koniec koloru w połowie wysokości
                color: Color {
                    a: 0.0,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });
            stopsy[1] = Some(ColorStop {
                offset: 1.0, // Start na górze
                color: Color {
                    a: 0.5,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });

            button::Style {
                background: Some(Background::Gradient(
                    iced_core::gradient::Gradient::Linear(Linear {
                                angle: Radians(0.0),
                                stops: stopsy,
                            }),
                )),
                text_color: KOLOR_CZCIONKI_JASNY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                ..button::Style::default()
            }
        } else if warunek2 {
            let mut stopsy = [None; 8];
            stopsy[0] = Some(ColorStop {
                offset: 0.0, // Start na dole
                color: Color {
                    a: 0.5,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });
            stopsy[1] = Some(ColorStop {
                offset: 0.5, // Koniec koloru w połowie wysokości
                color: Color {
                    a: 0.0,
                    ..Color::from_rgb(kolor.0, kolor.1, kolor.2)
                },
            });
            button::Style {
                background: Some(Background::Gradient(
                    iced_core::gradient::Gradient::Linear(Linear {
                        angle: Radians(0.0),
                        stops: stopsy,
                    }),
                )),
                text_color: KOLOR_CZCIONKI_JASNY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 5.0.into(),
                },
                ..button::Style::default()
            }
        } else {
            // --- STAN WYŁĄCZONY (Biały z Alpha 0.2) ---
            button::Style {
                background: Some(
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 0.0, // Twoje 0.2 alpha
                    }
                    .into(),
                ),
                text_color: KOLOR_CZCIONKI_SREDNI, // Lekko przygaszony tekst
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
                    // ..Border::default()
                },
                ..button::Style::default()
            }
        }
    }
}
