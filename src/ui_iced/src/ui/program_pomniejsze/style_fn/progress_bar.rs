pub fn styl_progress_bar<'a>(
    kolor_akcentu: (f32, f32, f32),
    kolor_tla: (f32, f32, f32),
) -> impl Fn(&iced::Theme) -> iced::widget::progress_bar::Style + 'a {
    move |_theme: &iced::Theme| {
        use iced::widget::progress_bar;
        use iced::{Border, Color};

        let tlo_paska = Color::from_rgb(kolor_tla.0, kolor_tla.1, kolor_tla.2);
        let wypelnienie = Color::from_rgba(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2, 0.6);
        let kolor_border = Color::from_rgba(kolor_akcentu.0, kolor_akcentu.1, kolor_akcentu.2, 0.2);

        progress_bar::Style {
            // Tło całego paska (to pod spodem)
            background: tlo_paska.into(),
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
