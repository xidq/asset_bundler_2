use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::scrollable::{AutoScroll, Rail, Scroller};
use iced_core::Color;

pub fn styl_scrollable<'a>(
    kolor_akcenty: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme, iced::widget::scrollable::Status) -> iced::widget::scrollable::Style + 'a {
    move |_theme: &iced::Theme, status: iced::widget::scrollable::Status| {
        use iced::widget::scrollable::{self, AutoScroll, Rail, Scroller};
        use iced::{Background, Border, Color};



        // Dynamiczny dobór przezroczystości uchwytu w zależności od interakcji
        let scroller_color = match status {
            scrollable::Status::Active { .. } => Color { a: temat.obecny_theme.low, ..*kolor_akcenty },
            scrollable::Status::Hovered { .. } => Color { a: temat.obecny_theme.mid, ..*kolor_akcenty },
            scrollable::Status::Dragged { .. } => Color { a: temat.obecny_theme.hi, ..*kolor_akcenty },
        };

        // Wspólny border dla uchwytów (zaokrąglenie)
        let scroller_border = Border {
            radius: 5.0.into(), // Zaokrąglamy tak jak progress bar
            width: 0.0,
            color: Color{a:temat.obecny_theme.min, ..*kolor_akcenty},
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