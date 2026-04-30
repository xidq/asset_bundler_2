use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::container;
use iced_core::Color;

pub fn styl_hint<'a>(
temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> container::Style + 'a {
    move |_theme| {
        container::Style {

            text_color: Some(Color{a: temat.tekst.hi, ..Color::BLACK}),


            // Czarne tło, 80% widoczności (0.8)
            background: Some(iced::Color{a: temat.obecny_theme.hi, ..temat.kolory.hint}.into()),

            // Ramka w kolorze akcentu
            border: iced::Border {
                color: iced::Color{a: temat.obecny_theme.hi, ..Color::BLACK},
                width: 1.0,
                radius: 6.0.into(), // Zaokrąglone rogi
            },

            shadow: iced::Shadow::default(),
            snap: false,
        }
    }

}