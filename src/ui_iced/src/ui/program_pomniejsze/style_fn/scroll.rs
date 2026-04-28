use iced::widget::scrollable::{AutoScroll, Rail, Scroller};

pub fn styl_scrollable<'a>(
    kolor_akcentu: (f32, f32, f32),
) -> impl Fn(&iced::Theme, iced::widget::scrollable::Status) -> iced::widget::scrollable::Style + 'a {
    move |_theme: &iced::Theme, status: iced::widget::scrollable::Status| {
        use iced::widget::scrollable::{self, Rail, Scroller, AutoScroll};
        use iced::{Border, Color, Background};

        let akcent = Color::from_rgb(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2);

        // Dynamiczny dobór przezroczystości uchwytu w zależności od interakcji
        let scroller_color = match status {
            scrollable::Status::Active { .. } => Color { a: 0.2, ..akcent },
            scrollable::Status::Hovered { .. } => Color { a: 0.5, ..akcent },
            scrollable::Status::Dragged { .. } => Color { a: 0.8, ..akcent },
        };

        // Wspólny border dla uchwytów (zaokrąglenie)
        let scroller_border = Border {
            radius: 5.0.into(), // Zaokrąglamy tak jak progress bar
            width: 0.0,
            color: Color::TRANSPARENT,
        };

        scrollable::Style {
            container: iced::widget::container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: Border::default(),
                ..Default::default()
            },

            vertical_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(scroller_color),
                    border: scroller_border,
                },
            },

            horizontal_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(scroller_color),
                    border: scroller_border,
                },
            },

            gap: None,

            auto_scroll: AutoScroll {
                background: Background::Color(scroller_color),
                border: scroller_border,
                shadow: Default::default(),
                icon: Default::default(),
            },
        }
    }
}