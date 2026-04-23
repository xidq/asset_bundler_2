use iced::widget::slider;
use iced_core::{Border, Color};


pub fn styl_sliderów<'a>(
    kolor: (f32, f32, f32),
) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style + 'a {
    move |_theme, _status| slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Color::from_rgba(kolor.0, kolor.1, kolor.2, 0.4).into(), // Aktywne (lewo) - Czerwony
                Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
            ),
            width: 4.,
            border: Border {
                color: Color::from_rgba(kolor.0, kolor.1, kolor.2, 0.2),
                width: 1.0,
                radius: 5.0.into(),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Color::from_rgb(kolor.0, kolor.1, kolor.2).into(), // Czerwona kropka
            border_width: 1.0,
            border_color: Color::BLACK,
        },
    }
}
