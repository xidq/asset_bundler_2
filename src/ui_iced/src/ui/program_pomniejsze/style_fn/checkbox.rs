use crate::ui::program_pomniejsze::kolory::{
    KOLOR_CZCIONKI_SREDNI, KOLOR_OBRAMOWANIA_NIE_AKTYWNY,
};
use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::checkbox;
use iced_core::Color;

pub fn styl_checkbox<'a>(
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, checkbox::Status) -> checkbox::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::checkbox::Status| {
        use iced::widget::checkbox;
        use iced::{Border, Color};


        let kolor_akcentuu =
            Color{ a:temat.obecny_theme.mid, ..*kolor_akcentu };

        // Sprawdzamy stan Hover
        let czy_hover = matches!(_status, checkbox::Status::Hovered { .. });
        // Sprawdzamy czy zaznaczony (Status ma w sobie info o zaznaczeniu)
        let czy_zaznaczony = match _status {
            checkbox::Status::Active { is_checked } => is_checked,
            checkbox::Status::Hovered { is_checked } => is_checked,
            checkbox::Status::Disabled { is_checked } => is_checked,
        };

        checkbox::Style {
            // Tło kwadracika: jeśli zaznaczony, dajemy akcent, jeśli nie - tło bazowe
            background: if czy_zaznaczony {
                kolor_akcentuu.into()
            } else {
                temat.obecny_theme.bground.into()
            },

            // Kolor "ptaszka" (widoczny tylko gdy zaznaczony)
            icon_color: KOLOR_CZCIONKI_SREDNI,

            // Kolor tekstu obok checkboxa
            text_color: Some(KOLOR_CZCIONKI_SREDNI),

            border: Border {
                radius: 3.0.into(), // Lekko mniejszy promień dla małego kwadratu
                width: if czy_hover { 1.0 } else { 0.5 },
                color: if czy_hover {
                    kolor_akcentuu
                } else {
                    KOLOR_OBRAMOWANIA_NIE_AKTYWNY
                },
            },
        }
    }
}
