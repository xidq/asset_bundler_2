use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptRozszerzeniaPlikówZdjęciowych};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, slider, space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_png_wybor(
    dane: &DaneDoBathKonwersjaZdjec,
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    let space_val = if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {15} else {0};
    Column::new()
        .push(
            if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                Row::new()
                    .push(
                        button(
                            text("png")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .center(),
                        )
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaWybranyPng)
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany,
                            KOLOR_SPANISH_ORANGE,
                        ))
                        .width(Length::FillPortion(5)),
                    )
                    .push(space().width(Length::FillPortion(1)))
                    .push(
                        Column::new()
                            .push(
                                slider(
                                    0..=9,
                                    match dane.rozszerzenia_plików_zdjęciowych[1] {
                                        OptRozszerzeniaPlikówZdjęciowych::Png {
                                            kompresja,
                                            ..
                                        } => kompresja,
                                        _ => 0,
                                    },
                                    Message::ZdjeciaEdycjaZmianaKompresjiPng,
                                )
                                .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
                            )
                            .push(
                                text(format!(
                                    "Kompresja: {}",
                                    match dane.rozszerzenia_plików_zdjęciowych[1] {
                                        OptRozszerzeniaPlikówZdjęciowych::Png {
                                            kompresja,
                                            ..
                                        } => kompresja,
                                        _ => 0,
                                    }
                                ))
                                .font(jezyk.get_font())
                                .color(KOLOR_CZCIONKI_SREDNI)
                                .width(Length::Fill)
                                .center(),
                            )
                            .width(Length::FillPortion(12)),
                    )
            } else {
                Row::new()
                    .push(
                        button(
                            text("png")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .center(),
                        )
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaWybranyPng)
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany,
                            KOLOR_SPANISH_ORANGE,
                        ))
                        .width(Length::FillPortion(5)),
                    )
                    .push(space().width(Length::FillPortion(13)))
            }
            .width(Length::FillPortion(5)),
        )
        .push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
            Row::new()
                .push(
                    button(
                        text("8-bit")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::B8,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bit,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("Luma8")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::L8,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bit,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("16-bit")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::B16,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bit,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("Luma16")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::L16,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bit,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
        } else {
            Row::new()
        })
        .push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
            Row::new()
                .push(
                    button(
                        text("8-bit\nAlpha")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::B8a,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bita,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("Luma8\nAlpha")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::L8a,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bita,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("16-bit\nAlpha")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::B16a,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bita,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
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
                .push(
                    button(
                        text("luma16\nAlpha")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(
                        OptFormatyKoloruObrazOgólny::L16a,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bita,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
        } else {
            Row::new()
        })
        .spacing(space_val) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_png_misc(
    dane: &DaneDoBathKonwersjaZdjec,
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    Row::new()
        .push(
            text("Png")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany { 0.5 } else { 0.2 },
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
            text("8")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bit && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("8a")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bita && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("l8")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bit && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("l8a")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bita && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("16")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bit && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("16a")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bita && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("l16")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bit && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("l16a")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bita && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany {
                        0.5
                    } else {
                        0.2
                    },
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
            text(format!(
                "{}",
                match dane.rozszerzenia_plików_zdjęciowych[1] {
                    OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, .. } => kompresja,
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
                if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany { 0.5 } else { 0.2 },
            ))
            .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
}
