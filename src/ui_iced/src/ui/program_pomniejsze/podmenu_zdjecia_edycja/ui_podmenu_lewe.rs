use crate::ui::program_pomniejsze::kolory::KOLOR_SPANISH_ORANGE;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN, WybraneOknoEdycjiZdjęć,
};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row, button, space, text};
use iced_core::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_lewe_wybor(
    wybrane_okno: &WybraneOknoEdycjiZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    Column::new()
        .push(
            button(
                text(jezyk.t("foto_edit_menu_paths"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::Ścieżki,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::Ścieżki),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk rozszerzenia_plików_zdjęciowych
        .push(
            button(
                text(jezyk.t("foto_edit_menu_extensions"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(
                    wybrane_okno,
                    WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych
                ),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk OptRozdzielczościObrazów
        .push(
            button(
                text(jezyk.t("foto_edit_menu_resolution"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::MenuOptRozdzielczościObrazów,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(
                    wybrane_okno,
                    WybraneOknoEdycjiZdjęć::MenuOptRozdzielczościObrazów
                ),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk inne
        .push(
            button(
                text(jezyk.t("foto_edit_menu_rest"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::MenuReszta,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::MenuReszta),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        .spacing(15)
        .width(Length::FillPortion(1))
}

pub fn podmenu_lewe_rozdzielczosci(
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    Row::new()
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
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("32")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_32 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("64")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_64 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("128")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_128 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("256")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_256 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("512")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_512 {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("1k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_1k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("2k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_2k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("4k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_4k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("6k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_6k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("8k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_8k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("16k")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16k {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("org")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_org {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
}
