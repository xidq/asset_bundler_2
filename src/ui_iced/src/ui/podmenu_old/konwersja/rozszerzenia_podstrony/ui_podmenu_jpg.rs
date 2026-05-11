use std::sync::Arc;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, slider, space, text, tooltip, Column, Row};
use iced::Length;
use strum::{EnumMessage, IntoEnumIterator};
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg};
use enumy::rozszerzenia::kolor::{JpgQuant, JpgSamplingFac};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::fn_ogolne::{info_male};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::btn_bdepth_jpg_konwersja;
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage::JpgQua;

pub fn podmenu_jpg_wybor_top<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {





     Column::new()
         .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Jpg, kolor, jezyk.get_font(), temat))

        .push(
            if let Some(
                Rozszerzenia::Jpg {
                            jakosc,
                            progresywny: _,
                            bit_depth: _,
                            sampling,
                            quant,
                            scans,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Jpg { .. }))
                {
                container(
                    Column::new()
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=100,
                                    *jakosc,
                                    |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::JpgJakość(vv))
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

                        BdepthJpg::iter()
                            .into_iter()
                            .fold(
                                Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_jpg_konwersja(
                                            wariant,
                                            wariant.bath_konwersja_id(),
                                            jezyk.get_font(),
                                            kolor,
                                            temat
                                        ),
                                    )
                                }
                            )
                            
                        .push(
                            Row::new()
                                .push(
                                    hint_btn(
                                        button(
                                            text(jezyk.t("conversion_jpg_prog"))
                                                .font(jezyk.get_font())
                                                .width(Length::Fill)
                                                .height(Length::Fill)
                                                .center(),
                                        )
                                            .padding(10)
                                            .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::JpgProg("btn_id_batch_jpg_progress")))
                                            .style(
                                                styl_przycisków(
                                                "btn_id_batch_jpg_progress",
                                                None,
                                                kolor,
                                                temat,
                                            ))
                                            .width(Length::FillPortion(1)),

                                        jezyk.t("hint_conversion_jpg_prog"),
                                        temat
                                    )
                                )
                        )
                    )
                    .push(
                        Row::new()
                            .push(
                                pick_list(JpgSamplingFac::iter().collect::<Vec<_>>(), Some(*sampling), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::JpgSampling(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(kolor,temat))
                                    .menu_style(styl_menu_pick(kolor,temat))
                                    .width(Length::FillPortion(4))
                            )
                            .push(
                                pick_list(JpgQuant::iter().collect::<Vec<_>>(), Some(*quant), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::JpgQua(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(kolor,temat))
                                    .menu_style(styl_menu_pick(kolor,temat))
                                    .width(Length::FillPortion(4))
                            ).padding(15).spacing(10)
                    )
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    2..=64,
                                    *scans,
                                    |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::JpgScan(vv))
                                )
                                    .height(20.)
                                    .width(Length::FillPortion(6))
                                    .style(styl_sliderów(kolor,temat)),
                            )
                            .push(
                                text(format!(
                                    "Sc: {}",
                                    scans
                                ))
                                    .color(KOLOR_CZCIONKI_SREDNI)
                                    .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                            )
                            .padding(15)
                    )
                ).height(200.).style(styl_kontenera(true, RodzajeContainer::Góra,kolor,temat))
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

pub fn podmenu_jpg_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    
    let jpg_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, Rozszerzenia::Jpg { .. })
    });



    let jakosc_str = jpg_data
        .and_then(|f| if let Rozszerzenia::Jpg { jakosc, .. } = f { Some(jakosc.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());

    let qa_str = jpg_data
        .and_then(|f| if let Rozszerzenia::Jpg { quant, .. } = f { Some(quant.get_message().unwrap_or("err").to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());





    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_jpg_ext",temat));

    for wariant in BdepthJpg::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_jpg_ext",temat))
        .push(info_male(jakosc_str, "btn_id_batch_jpg_ext",temat))
        .push(info_male("|".to_string(), "btn_id_batch_jpg_ext",temat))
        .push(info_male(qa_str, "btn_id_batch_jpg_ext",temat))
        .push(info_male("|".to_string(), "btn_id_batch_jpg_ext",temat))
        .push(info_male("Prog".to_string(), "btn_id_batch_jpg_progress",temat));

    Column::new()
        /*tekst*/.push(info_male("Jpg".to_string(),"btn_id_batch_jpg_ext",temat))
        .push(row_rozszerzen)
}
