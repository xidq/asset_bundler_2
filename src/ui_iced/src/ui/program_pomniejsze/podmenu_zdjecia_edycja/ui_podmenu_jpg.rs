use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptRozszerzeniaPlikówZdjęciowych};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row, button, container, slider, space, text, tooltip};
use iced::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_jpg_wybor_top(
    dane: &DaneDoBathKonwersjaZdjec,
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    let space_val = if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {15} else {0};

    Column::new()
        .push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {
            Row::new()
                .push(
                    button(
                        text("jpg")
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                    .padding(10)
                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyJpg)
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    Column::new()
                        .push(
                            slider(
                                0..=100,
                                match dane.rozszerzenia_plików_zdjęciowych[0] {
                                    OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. } => {
                                        jakosc
                                    }
                                    _ => 0,
                                },
                                Message::ZdjeciaEdycjaZmianaJakosciJpg,
                            )
                            .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
                        )
                        .push(
                            text(format!(
                                "{} {}%",
                                jezyk.t("foto_edit_quality"),
                                match dane.rozszerzenia_plików_zdjęciowych[0] {
                                    OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. } =>
                                        jakosc,
                                    _ => 0,
                                }
                            ))
                            .color(KOLOR_CZCIONKI_SREDNI)
                            .font(jezyk.get_font()),
                        )
                        .width(Length::FillPortion(5)),
                ) //push column qniec
                .push(space().width(Length::FillPortion(1)))
                .push(tooltip(
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {
                        button(
                            text("Prog.")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .center(),
                        )
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaProgresJpg)
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_progres,
                            KOLOR_SPANISH_ORANGE,
                        ))
                        .width(Length::FillPortion(5))
                    } else {
                        button(
                            text("Prog.")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .center(),
                        )
                        .padding(10)
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_progres,
                            KOLOR_SPANISH_ORANGE,
                        ))
                        .width(Length::FillPortion(5))
                    },
                    "Zapis progresywny on/off",
                    tooltip::Position::Top,
                ))
                .push(space().width(Length::FillPortion(1)))
        } else {
            Row::new()
                .push(
                    button(
                        text("jpg")
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                    .padding(10)
                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyJpg)
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5)),
                )
                .push(space().width(Length::FillPortion(13)))
        })
        // MENU Z WYBORAMI
        // DRUGI ROW
        .push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {
            Row::new()
                .push(tooltip(
                    button(
                        text(jezyk.t("foto_edit_color").to_string())
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                    .padding(10)
                    .on_press(Message::ZdjeciaEdycjaZmianaKolorJpg(
                        OptFormatyKoloruObrazOgólny::B8,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_rgb,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                    text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                    tooltip::Position::Top,
                ))
                .push(space().width(Length::Fixed(5.)))
                .push(
                    container("")
                        .width(Length::Fixed(2.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style {
                            text_color: None,
                            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.2).into()),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: false,
                        }),
                )
                .push(space().width(Length::Fixed(5.)))
                .push(tooltip(
                    button(
                        text("bw")
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                    .padding(10)
                    .on_press(Message::ZdjeciaEdycjaZmianaKolorJpg(
                        OptFormatyKoloruObrazOgólny::L8,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_bw,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                    text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                    tooltip::Position::Top,
                ))
        } else {
            Row::new()
        })
        .spacing(space_val) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_jpg_misc(
    dane: &DaneDoBathKonwersjaZdjec,
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    Row::new()
        .push(
            text("Jpg")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany { 0.5 } else { 0.2 },
                ))
                .size(crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(
            crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN,
        )))
        .push(
            text("C")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_rgb && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(
            crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN,
        )))
        .push(
            text("BW")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_bw && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(
            crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN,
        )))
        .push(
            text("|")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(1., 1., 1., 0.3))
                .size(crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(
            crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN,
        )))
        .push(
            text(format!(
                "{}%",
                match dane.rozszerzenia_plików_zdjęciowych[0] {
                    OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. } => jakosc,
                    _ => 0,
                }
            ))
            .font(iced::Font {
                family: iced::font::Family::Name("VT323"),
                ..Default::default()
            })
            .color(Color::from_rgba(
                1.,
                1.,
                1.,
                if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany { 0.5 } else { 0.2 },
            ))
            .size(crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN),
        )
}
