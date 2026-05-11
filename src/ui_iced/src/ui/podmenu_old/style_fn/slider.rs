use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::slider;
use iced_core::{Border, Color};

pub fn styl_sliderów<'a>(
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style + 'a {
    move |_theme, _status| slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Color{a:temat.obecny_theme.mid, ..*kolor}.into(), // Aktywne (lewo) - Czerwony
                Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
            ),
            width: 4.,
            border: Border {
                color: Color{a:temat.obecny_theme.low, ..*kolor},
                width: 1.0,
                radius: 5.0.into(),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Color{a: temat.obecny_theme.hi, ..*kolor}.into(), // Czerwona kropka
            border_width: 1.0,
            border_color: Color::BLACK,
        },
    }
}
