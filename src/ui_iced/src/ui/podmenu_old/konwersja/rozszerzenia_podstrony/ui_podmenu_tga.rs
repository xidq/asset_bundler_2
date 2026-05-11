use std::sync::Arc;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::fn_ogolne::{info_male};
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, text, tooltip, Column, Row};
use iced_core::Length;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthQoi, BdepthTga};
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::{btn_bdepth_qoi_konwersja, btn_bdepth_tga_konwersja};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;

pub fn podmenu_tga_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Tga, kolor, jezyk.get_font(), temat))

        .push(
            if let Some(Rozszerzenia::Tga {
                            bit_depth: _,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Tga { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    text("bez kompresji")
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font()).width(Length::Fill).height(Length::Fill).center(),
                                )
                                .height(Length::FillPortion(1)).padding(15)
                        )
                        .push(
                            BdepthTga::iter()
                                .skip(1)
                                .step_by(2)
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_tga_konwersja(
                                            wariant,
                                            wariant.bath_konwersja_id(),
                                            jezyk.get_font(),
                                            kolor,
                                            temat
                                        ),
                                    )
                                })
                        )
                        // .push(
                        //     Row::new()
                        //         .push(
                        //             tooltip(
                        //                 btn_zbiorowe_bdepth(RozszerzeniaZnacznik::Tga, Arc::new(BdepthTga::Luma8), Arc::new(dane), jezyk.get_font(), kolor, temat),
                        //                 text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                        //                 tooltip::Position::Top,
                        //             )
                        //         )
                        //         .push(
                        //             tooltip(
                        //                 btn_zbiorowe_bdepth(RozszerzeniaZnacznik::Tga, Arc::new(BdepthTga::HighColor16), Arc::new(dane), jezyk.get_font(), kolor, temat),
                        //                 text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                        //                 tooltip::Position::Top,
                        //             )
                        //         )
                        //         .push(
                        //             tooltip(
                        //                 btn_zbiorowe_bdepth(RozszerzeniaZnacznik::Tga, Arc::new(BdepthTga::TrueColor24), Arc::new(dane), jezyk.get_font(), kolor, temat),
                        //                 text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                        //                 tooltip::Position::Top,
                        //             )
                        //         )
                        //         .push(
                        //             tooltip(
                        //                 btn_zbiorowe_bdepth(RozszerzeniaZnacznik::Tga, Arc::new(BdepthTga::TrueColorA32), Arc::new(dane), jezyk.get_font(), kolor, temat),
                        //                 text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                        //                 tooltip::Position::Top,
                        //             )
                        //         )
                        //         .height(Length::FillPortion(1))
                        // )
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
            } else {
                container(Row::new())
            }
        )
        // MENU Z WYBORAMI
        // DRUGI ROW
        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_tga_misc(
    // dane: &'a DaneDoBathKonwersjaZdjec,
    temat: &UstawieniaThemeWsio,
    // stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Column<Message> {





    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_tga_ext",temat));

    for wariant in BdepthTga::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    // row_rozszerzen = row_rozszerzen
    //     .push(info_male("|".to_string(), "btn_id_batch_avif_ext",temat))
    //     .push(info_male(jakosc_str, "btn_id_batch_avif_ext",temat))
    //     .push(info_male("|".to_string(), "btn_id_batch_avif_ext",temat))
    //     .push(info_male(kompresja, "btn_id_batch_avif_ext",temat))
    //     .push(info_male("|".to_string(), "btn_id_batch_avif_ext",temat))
    //     .push(info_male(chrum, "btn_id_batch_avif_ext",temat));

    Column::new()
        /*tekst*/.push(info_male("Tga".to_string(),"btn_id_batch_tga_ext",temat))
        .push(row_rozszerzen)
}
