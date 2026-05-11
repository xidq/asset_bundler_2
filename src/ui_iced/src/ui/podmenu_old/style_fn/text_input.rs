use crate::ui::podmenu::kolory::KOLOR_ERROR;
use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::text_input;
use iced_core::Color;

pub fn styl_text_input<'a>(
    checker:bool,
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
        use iced::widget::text_input;
        use iced::{Border, Color};

        let kolor_error = Color{a: 0.2, ..KOLOR_ERROR};
        let tlo_bazowe = temat.obecny_theme.bground;
        // let kolor_akcentu = Color::from_rgb(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2);

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
                    a: temat.obecny_theme.mid,
                    ..tlo_bazowe
                }
                .into()
            } else {
                tlo_bazowe.into()
            },

            value: if czy_disabled {
                Color {
                    a: temat.obecny_theme.low,
                    ..temat.obecny_theme.kolor
                }
            } else {
                Color {
                    a: temat.obecny_theme.hi,
                    ..temat.obecny_theme.kolor
                }
            },

            placeholder: Color {
                a: temat.obecny_theme.low,
                ..temat.obecny_theme.kolor
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
                    *kolor_akcentu
                } else {
                    Color {
                        a: temat.obecny_theme.low,
                        ..temat.obecny_theme.kolor
                    }
                },
            },

            icon: Color{a:temat.tekst.hi,..temat.tekst.kolor},
            selection: Color {
                a: temat.obecny_theme.low,
                ..*kolor_akcentu
            },
        }
    }
}
