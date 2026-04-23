use iced::widget::text_input;
use crate::ui::program_pomniejsze::kolory::KOLOR_ERROR;

pub fn styl_text_input<'a>(
    checker:bool,
    kolor_akcentu: (f32, f32, f32),
    kolor_tla: (f32, f32, f32),
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
        use iced::widget::text_input;
        use iced::{Border, Color};

        let kolor_error = Color::from_rgba(KOLOR_ERROR.0, KOLOR_ERROR.1, KOLOR_ERROR.2, 0.2);
        let tlo_bazowe = Color::from_rgb(kolor_tla.0, kolor_tla.1, kolor_tla.2);
        let kolor_akcentu = Color::from_rgb(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2);

        // Sprawdzamy stany z uwzględnieniem struktury wariantu Focused
        let (czy_hover, czy_focused, czy_disabled) = match _status {
            text_input::Status::Hovered => (true, false, false),
            text_input::Status::Focused { is_hovered } => (is_hovered, true, false),
            text_input::Status::Disabled => (false, false, true),
            _ => (false, false, false),
        };

        text_input::Style {
            // Jeśli wyłączony, lekko przyciemniamy tło
            background: if czy_disabled {
                Color {
                    a: 0.5,
                    ..tlo_bazowe
                }
                .into()
            } else {
                tlo_bazowe.into()
            },

            value: if czy_disabled {
                Color {
                    a: 0.3,
                    ..Color::WHITE
                }
            } else {
                Color {
                    a: 0.7,
                    ..Color::WHITE
                }
            },

            placeholder: Color {
                a: 0.3,
                ..Color::WHITE
            },

            border: Border {
                radius: 5.0.into(),
                // Świecimy ramką jeśli hover LUB focused
                width: if (czy_hover || czy_focused) && !czy_disabled {
                    1.0
                } else {
                    0.5
                },
                color: if !checker{
                    kolor_error
                }else if (czy_hover || czy_focused) && !czy_disabled {
                    kolor_akcentu
                } else {
                    Color {
                        a: 0.1,
                        ..Color::WHITE
                    }
                },
            },

            icon: Color::WHITE,
            selection: Color {
                a: 0.3,
                ..kolor_akcentu
            },
        }
    }
}
