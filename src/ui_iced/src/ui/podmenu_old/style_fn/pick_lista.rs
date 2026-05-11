use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::pick_list;
use iced::{Border, Color};
use iced_core::{Shadow, Vector};

pub fn styl_pick_list<'a>(
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style + 'a {
    move |_theme: &iced::Theme, _status: iced::widget::pick_list::Status| {
        


        use iced::widget::pick_list;

        // Kolor bazowy tła (0.1, 0.1, 0.1)
        let tlo_bazowe = temat.obecny_theme.bground;
        let kolor_akcentuu =
            Color{a:temat.obecny_theme.mid, ..*kolor_akcentu };
        let kolor_border = Color{a:temat.obecny_theme.low, ..*kolor_akcentu };

        // 1. Styl samego głównego pola PickList
        pick_list::Style {
            text_color: Color {
                a: temat.tekst.hi,
                ..temat.tekst.kolor
            },
            placeholder_color: Color {
                a: temat.obecny_theme.mid,
                ..temat.tekst.kolor
            },
            handle_color: temat.obecny_theme.kolor,
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
    kolor_zaznaczenia: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> iced::overlay::menu::Style + 'a {
    move |_theme| {
        let tlo_menu = temat.obecny_theme.bground;
        let kolor_zaznaczenia = Color{a: temat.obecny_theme.mid, ..*kolor_zaznaczenia};

        iced::overlay::menu::Style {
            text_color: Color {
                a: temat.tekst.mid,
                ..temat.tekst.kolor
            },
            background: tlo_menu.into(),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: Color {
                    a: temat.obecny_theme.low,
                    ..temat.obecny_theme.kolor
                },
            },

            selected_text_color: temat.obecny_theme.kolor,
            selected_background: kolor_zaznaczenia.into(),
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
        }
    }
}
