use enumy::inne_ui::UstawieniaThemeWsio;
use iced_core::Color;

pub fn styl_progress_bar<'a>(
    kolor_akcentu: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> impl Fn(&iced::Theme) -> iced::widget::progress_bar::Style + 'a {
    move |_theme: &iced::Theme| {
        use iced::widget::progress_bar;
        use iced::{Border, Color};

        let wypelnienie = Color{ a: temat.obecny_theme.mid, ..*kolor_akcentu};
        let kolor_border = Color{ a: temat.obecny_theme.low, ..*kolor_akcentu};;

        progress_bar::Style {
            // Tło całego paska (to pod spodem)
            background: temat.obecny_theme.bground.into(),
            // To, co się przesuwa (postęp)
            bar: wypelnienie.into(),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: kolor_border,
            },
        }
    }
}
