use iced::widget::pick_list;
use iced::{Border, Color};
use iced_core::{Shadow, Vector};

pub fn styl_pick_list<'a>(
    kolor_akcentu: (f32, f32, f32),
    kolor_tla: (f32, f32, f32),
) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::pick_list::Status| {
        use iced::widget::pick_list;

        // Kolor bazowy tła (0.1, 0.1, 0.1)
        let tlo_bazowe = Color::from_rgb(kolor_tla.0, kolor_tla.1, kolor_tla.2);
        let kolor_akcentuu =
            Color::from_rgba(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2, 0.5);
        let kolor_border = Color::from_rgba(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2, 0.2);

        // 1. Styl samego głównego pola PickList
        pick_list::Style {
            text_color: Color {
                a: 0.7,
                ..Color::WHITE
            },
            placeholder_color: Color {
                a: 0.5,
                ..Color::WHITE
            },
            handle_color: Color::WHITE,
            background: tlo_bazowe.into(),
            border: Border {
                radius: 5.0.into(),
                width: if matches!(_status, pick_list::Status::Hovered) {
                    1.1
                } else {
                    1.0
                },
                color: if matches!(_status, pick_list::Status::Hovered) {
                    kolor_akcentuu
                } else {
                    kolor_border
                },
            },
        }
    }
}

pub fn styl_menu_pick<'a>(
    kolor_zaznaczenia: (f32, f32, f32),
    kolor_tla: (f32, f32, f32),
) -> impl Fn(&iced::Theme) -> iced::overlay::menu::Style + 'a {
    move |_theme| {
        let tlo_menu = Color::from_rgb(kolor_tla.0, kolor_tla.1, kolor_tla.2);
        let kolor_zaznaczenia = Color::from_rgba(
            kolor_zaznaczenia.0,
            kolor_zaznaczenia.1,
            kolor_zaznaczenia.2,
            0.5,
        ); // TWÓJ KOLOR

        iced::overlay::menu::Style {
            text_color: Color {
                a: 0.7,
                ..Color::WHITE
            },
            background: tlo_menu.into(),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: Color {
                    a: 0.1,
                    ..Color::WHITE
                },
            },

            selected_text_color: Color::WHITE,
            selected_background: kolor_zaznaczenia.into(),
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
        }
    }
}
