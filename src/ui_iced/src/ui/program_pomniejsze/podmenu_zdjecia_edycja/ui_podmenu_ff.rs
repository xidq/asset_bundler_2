use crate::ui::program_pomniejsze::kolory::{
    KOLOR_CZCIONKI_SREDNI, KOLOR_SPANISH_ORANGE, KOLOR_TŁA,
};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, pick_list, slider, space, text, Column, Row};
use iced::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_ff_wybor(
    dane: &DaneDoBathKonwersjaZdjec,
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {


    let opcje = vec![
        OptMetodaKompresjiZdjecia::Brak,
        OptMetodaKompresjiZdjecia::Zstd(3), // Domyślny poziom
        OptMetodaKompresjiZdjecia::Bzip2(1),
        OptMetodaKompresjiZdjecia::Xz(6),
    ];

    // 2. Pobieramy aktualnie wybraną metodę z Twoich danych
    let wybrana = if let OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji } =
        &dane.rozszerzenia_plików_zdjęciowych[4]
    {
        Some(*metoda_kompresji)
    } else {
        None
    };
    // let aktualna_wartosc_zstd = match &dane.rozszerzenia_plików_zdjęciowych[4] {
    //     rozszerzenia_plików_zdjęciowych::Ff { metoda_kompresji } => {
    //         metoda_kompresji.iter().find_map(|m| {
    //             if let OptMetodaKompresjiZdjecia::Zstd(v) = m { Some(*v) } else { None }
    //         }).unwrap_or(3) // Domyślnie 3, jeśli nie ma na liście
    //     },
    //     _ => 0,
    // };

    // .push(
    //     slider(0..=198,
    //            match dane.rozszerzenia_plików_zdjęciowych[4] {
    //         rozszerzenia_plików_zdjęciowych::Ff { metoda_kompresji:OptMetodaKompresjiZdjecia::Zstd(v)} =>  v,
    //         _ => 0
    //     },
    //        Message::ZdjeciaEdycjaZmianaKompresjaWartoscFF)
    //         .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
    //     )
    // )

    Column::new().push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany {
        Row::new()
            .push(
                button(
                    text("ff")
                        .font(jezyk.get_font())
                        .width(Length::Fill)
                        .center(),
                )
                .padding(10)
                .on_press(Message::ZdjeciaEdycjaZmianaWybranyFF)
                .style(styl_przycisków(
                    false,
                    stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany,
                    KOLOR_SPANISH_ORANGE,
                ))
                .width(Length::FillPortion(5)),
            )
            .push(space().width(Length::FillPortion(1)))
            .push(
                pick_list(opcje, wybrana, Message::ZdjeciaEdycjaZmianaKompresjaFF)
                    .width(Length::FillPortion(5))
                    .padding(2)
                    .style(styl_pick_list(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                    .menu_style(styl_menu_pick(KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
            )
            .spacing(5)
            .push(space().width(Length::FillPortion(1)))
            .push(
                if matches!(
                    dane.rozszerzenia_plików_zdjęciowych[4],
                    OptRozszerzeniaPlikówZdjęciowych::Ff {
                        metoda_kompresji: OptMetodaKompresjiZdjecia::Brak
                    }
                ) {
                    Column::new().width(Length::FillPortion(5))
                } else {
                    Column::new()
                        .push(
                            slider(
                                1..=198,
                                match dane.rozszerzenia_plików_zdjęciowych[4] {
                                    OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji } => {
                                        match metoda_kompresji {
                                            OptMetodaKompresjiZdjecia::Zstd(v)
                                            | OptMetodaKompresjiZdjecia::Bzip2(v)
                                            | OptMetodaKompresjiZdjecia::Xz(v) => v,
                                            OptMetodaKompresjiZdjecia::Brak => 0,
                                        }
                                    }
                                    _ => 0,
                                },
                                Message::ZdjeciaEdycjaZmianaKompresjaWartoscFF,
                            )
                            .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
                        )
                        .push(
                            text(format!(
                                "{}: {}",
                                jezyk.t("foto_edit_quality"),
                                match dane.rozszerzenia_plików_zdjęciowych[4] {
                                    OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji } =>
                                        match metoda_kompresji {
                                            OptMetodaKompresjiZdjecia::Zstd(v) => {
                                                (v as f32 / 9.).round().clamp(1., 22.) as u8
                                            }
                                            OptMetodaKompresjiZdjecia::Bzip2(v) => {
                                                (v as f32 / 22.).round().clamp(1., 9.) as u8
                                            }
                                            OptMetodaKompresjiZdjecia::Xz(v) => {
                                                (v as f32 / 22.).round().clamp(1., 9.) as u8
                                            }
                                            OptMetodaKompresjiZdjecia::Brak => 0,
                                        },
                                    _ => 0,
                                }
                            ))
                            .color(KOLOR_CZCIONKI_SREDNI)
                            .font(jezyk.get_font()),
                        )
                        .width(Length::FillPortion(5))
                },
            )
            .padding(15)
    } else {
        Row::new()
            .push(
                button(
                    text("ff")
                        .font(jezyk.get_font())
                        .width(Length::Fill)
                        .center(),
                )
                .padding(10)
                .on_press(Message::ZdjeciaEdycjaZmianaWybranyFF)
                .style(styl_przycisków(
                    false,
                    stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany,
                    KOLOR_SPANISH_ORANGE,
                ))
                .width(Length::FillPortion(5)),
            )
            .push(space().width(Length::FillPortion(13)))
            .padding(15)
    })

    // MENU Z WYBORAMI
    // DRUGI ROW
}

pub fn podmenu_ff_misc(
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    Row::new()
        .push(
            text("FF")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany { 0.5 } else { 0.2 },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("|")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(1., 1., 1., 0.3))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("Brak")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_brak && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("Zstd")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_zstd && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("Bzip2")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_bzip2 && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("Xz")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_xz && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
}
