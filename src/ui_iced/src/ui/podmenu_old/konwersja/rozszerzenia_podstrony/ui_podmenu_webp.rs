use std::sync::Arc;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::fn_ogolne::{info_male};
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, slider, space, text, tooltip, Column, Row};
use iced_core::Length;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::{btn_bdepth_avif_konwersja, btn_bdepth_webp_konwersja};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;

pub fn podmenu_webp_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Webp, kolor, jezyk.get_font(), temat))

        .push(
            if let Some(Rozszerzenia::Webp {
                            jakosc,
                            lossless,
                            bit_depth: _,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Webp { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=100,
                                        *jakosc,
                                        |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WebpJakość(vv))
                                    )
                                        .height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów(kolor,temat)),
                                )
                                .push(
                                    text(format!(
                                        "Q: {}%",
                                        jakosc
                                    ))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                                )
                                .padding(15)
                        )
                        .push(
                            Row::new()

                                .push(
                                    BdepthWebp::iter()
                                        .into_iter()
                                        .fold(Row::new(), |row, wariant| {
                                            row.push(
                                                btn_bdepth_webp_konwersja(
                                                    wariant,
                                                    wariant.bath_konwersja_id(),
                                                    jezyk.get_font(),
                                                    kolor,
                                                    temat
                                                ),
                                            )
                                        }
                                        )
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            tooltip(
                                                button(
                                                    text(jezyk.t("conversion_webp_losless"))
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fill)
                                                        .center(),
                                                )
                                                    .padding(10)
                                                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WebpLossless("btn_id_batch_webp_losless",
                                                    )))
                                                    .style(styl_przycisków(
                                                        "btn_id_batch_webp_losless",
                                                        None,
                                                        kolor,
                                                        temat,
                                                    ))
                                                    .width(Length::FillPortion(1)),

                                                jezyk.t("hint_conversion_webp_losless"),
                                                tooltip::Position::Top,
                                            )
                                        )
                                )
                        )
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra, kolor,temat))
            } else {
                container(Row::new())
            }
        )
        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_webp_misc<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    let webp_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, Rozszerzenia::Webp{ .. })
    });

    let lossless_bool = || {
        if let Some(Rozszerzenia::Webp { lossless, .. }) = webp_data {
            *lossless // zwracamy wartość bool
        } else {
            false
        }
    };
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: BdepthWebp| {
        if let Some(Rozszerzenia::Webp { bit_depth, .. }) = webp_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    // 3. Przygotowanie jakości
    let jakosc_str = webp_data
        .and_then(|f| if let Rozszerzenia::Webp { jakosc, .. } = f { Some(jakosc.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());


    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_webp_ext",temat));

    for wariant in BdepthWebp::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_webp_ext",temat))
        .push(info_male(jakosc_str, "btn_id_batch_webp_ext",temat));

    Column::new()
        /*tekst*/.push(info_male("Tga".to_string(),"btn_id_batch_webp_ext",temat))
        .push(row_rozszerzen)
}
