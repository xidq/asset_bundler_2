use std::path::{Path, PathBuf};
use std::sync::Arc;
use iced::Element;
use iced::widget::{button, container, pick_list, progress_bar, space, text, text_input, Column, Row};
use iced_core::{Color, Length};
use strum::IntoEnumIterator;
use enumy::dane_do_przetwarzania::DaneDoPakowaniaDds;
use enumy::enums_structs_io::LogPakowaniaDds;
use enumy::inne_ui::{ActProces, UstawieniaThemeWsio};
use enumy::rozszerzenia::kompresje::{OptFormatDds, OptKompresjaDds};
use enumy::statusy::LogTxDoPakowanieDds;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::progress_bar::styl_progress_bar;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
fn tekst_sciezek<'a>(
    sciezki: &'a Option<Vec<PathBuf>>,
    yy: usize,
    jezyk: &'a WybórJęzyka,
) -> Element<'a, Message> {
    text(match sciezki {
        Some(xx) if yy < xx.len() => jezyk.format_sciezek(&xx[yy]),
        _ => "".to_string(),
    })
        .color(KOLOR_CZCIONKI_SREDNI)
        .font(jezyk.get_font())
        .height(20.)
        .into() // Ważne: rzutowanie na Element<'a>
}

pub fn dds_pakowanie_main<'a>(dane: Arc<&'a DaneDoPakowaniaDds>, logi: &'a LogPakowaniaDds, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    let czy_sie_nada_na_wyslanie_pakowanie=dane.ścieżka_wejściowa.is_some() && dane.ścieżka_wyjściowa.exists() && !dane.nazwa.is_empty();

        Column::new()
            .push(
                text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
            )
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(
                                button(text("📄").width(Length::Fill).center())
                                    .padding(10)
                                    .on_press(Message::Dds(DdsMessage::PakowaniePathInFiles))
                                    .style(styl_przycisków(false, false, kolor,temat))
                                    .width(Length::Fixed(40.))
                                    .height(Length::Fixed(40.))
                            )
                            .push(
                                button(text("folder").width(Length::Fill).center())
                                    .padding(10)
                                    .on_press(Message::Dds(DdsMessage::PakowaniePathInFolders))
                                    .style(styl_przycisków(false, false, kolor,temat))
                                    .width(Length::Fixed(40.))
                                    .height(Length::Fixed(40.))
                            )
                    )
                    .push(container("").style(|_theme| container::Style {
                        background: Some(Color::from_rgba(1., 1., 1., 0.2).into()),
                        ..container::Style::default()
                    }).width(2.).height(80.)
                    )
                    .push(
                        match dane.ścieżka_wejściowa.clone().unwrap_or(Vec::from([PathBuf::from("")])).len(){
                            0 => {
                                Column::new()
                                    .push(
                                        text("Nie wybrano plików lub folderów").height(Length::Fixed(20.))
                                    )
                                    .push(space().height(Length::Fixed(60.)))
                            },
                            1 => {
                                Column::new()
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 0, jezyk))
                                    .push(space().height(Length::Fixed(60.)))
                            },
                            2 => {
                                Column::new()
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 0, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 1, jezyk))
                                    .push(space().height(Length::Fixed(40.)))
                            },
                            3 => {
                                Column::new()
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 0, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 1, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 2, jezyk))
                                    .push(space().height(Length::Fixed(20.)))
                            },
                            4 => {
                                Column::new()
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 0, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 1, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 2, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 3, jezyk))
                            }
                            _ => {
                                Column::new()
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 0, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 1, jezyk))
                                    .push(tekst_sciezek(&dane.ścieżka_wejściowa, 2, jezyk))
                                    .push(
                                        text(format!("i {} więcej",dane.ścieżka_wejściowa.clone().unwrap().len() - 3)).height(Length::Fixed(20.))
                                    )
                            },
                        }
                    )
                    // .push(
                    // container(
                    //     text_input(
                    //         jezyk.t("input_folder_or_file"),
                    //         &dane.ścieżka_wejściowa.to_string_lossy()
                    //     ).font(jezyk.get_font())
                    //         .padding(10)
                    //         .on_input(|xxx|Message::Dds(DdsMessage::DdsPakowanieZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane.ścieżka_wejściowa.exists(),  kolor,KOLOR_TŁA))
                    //         .width(Length::Fill)
                    // ).height(Length::Fixed(40.))
                    //     .width(Length::FillPortion(10))
                    // )
                    .spacing(15)
            )
            .push(
                text(jezyk.t("ŚcieżkaWyjściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
            )
            .push(
                Row::new()
                    .push(
                        button(text("📄").width(Length::Fill).center())
                            .padding(10)
                            .on_press(Message::Dds(DdsMessage::PakowaniePathOutBtn))
                            .style(styl_przycisków(false, false, kolor,temat))
                            .width(Length::Fixed(40.))
                            .height(Length::Fixed(40.)),
                    )
                    .push(
                        container(
                            text_input(
                                jezyk.t("input_folder_or_file"),
                                &dane.ścieżka_wyjściowa.to_string_lossy()
                            ).font(jezyk.get_font())
                                .padding(10)
                                .on_input(|xx|Message::Dds(DdsMessage::PakowaniePathOut(xx))).style(styl_text_input(dane.ścieżka_wyjściowa.exists(), kolor, temat))
                                .width(Length::Fill)
                        ).height(Length::Fixed(40.))
                            .width(Length::FillPortion(10))


                    ).spacing(15)
            )
            .push(
                Row::new()
                    .push(text("wybierz nazwę").font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI).height(40.).width(Length::FillPortion(3)).center())
                    .push(
                        container(
                            text_input(
                                jezyk.t("input_name"),
                                &dane.nazwa
                            ).font(jezyk.get_font())
                                .padding(10)
                                .style(styl_text_input(!dane.nazwa.is_empty(),kolor, temat))
                                .on_input(|xx|Message::Dds(DdsMessage::PakowanieNazwa(xx))).style(styl_text_input(dane.ścieżka_wyjściowa.exists(), kolor, temat))
                                .width(Length::Fill)
                        ).height(Length::Fixed(40.))
                            .width(Length::FillPortion(7))


                    ).spacing(15)
            )
            .push(ui_standard_oddzielacz())
            .push(
                text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
            )
            .push(
                pick_list(
                    OptFormatDds::iter().collect::<Vec<_>>(),
                    Some(dane.format),
                    |format| Message::Dds(DdsMessage::PakowanieFormat(format))
                )
                    .text_line_height(2.)
                    .width(Length::Fill)
                    .style(styl_pick_list(kolor, temat))
                    .menu_style(styl_menu_pick(kolor, temat)),
            )
            .push(ui_standard_oddzielacz())
            .push(
                text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
            )
            .push(
                pick_list(
                    OptKompresjaDds::iter().collect::<Vec<_>>(),
                    Some(dane.kompresja),
                    |kompresja| Message::Dds(DdsMessage::PakowanieKompresja(kompresja))
                )
                    .text_line_height(2.)
                    .width(Length::Fill)
                    .style(styl_pick_list(kolor, temat))
                    .menu_style(styl_menu_pick(kolor, temat)),
            )
            .push(ui_standard_oddzielacz())
            .push(
                if czy_sie_nada_na_wyslanie_pakowanie && temat.temp.aktywny_proces == ActProces::Żodyn {
                    button(
                        text(jezyk.t("process_btn_start"))
                            .font(jezyk.get_font())
                            .color(Color::from_rgba(1., 1., 1., 0.8))
                            .width(Length::Fill)
                            .center(),
                    )
                        .on_press(Message::Dds(DdsMessage::PakowanieStart))
                        .height(Length::Fixed(40.))
                        .width(Length::Fill)
                        .style(styl_przycisków(false, false, kolor,temat))
                } else {
                    button(
                        text(if temat.temp.aktywny_proces == ActProces::DdsPakowanie {
                            jezyk.t("btn_bussy_processing")
                        } else if temat.temp.aktywny_proces != ActProces::Żodyn {
                            jezyk.t("btn_bussy_processing_other")
                        } else {
                            jezyk.t("btn_gib_data")
                        })
                            .font(jezyk.get_font())
                            .color(Color::from_rgba(1., 1., 1., 0.7))
                            .width(Length::Fill)
                            .center(),
                    )
                        .width(Length::Fill)
                        .height(Length::Fixed(40.))
                        .style(styl_przycisków(
                            false,
                            temat.temp.aktywny_proces == ActProces::DdsPakowanie,
                            kolor, temat
                        ))
                },
            )
            // .push(space().height(Length::Fixed(15.)))
            .push(
                Row::new()
                    .push(text(format!("Postęp procesu:  {}",logi.w_trakcie)).font(jezyk.get_font()).width(Length::FillPortion(1)).center())
                    .push(
                        progress_bar(0.0..=100., logi.w_trakcie as f32)
                            .girth(18.)
                            .style(styl_progress_bar(kolor, temat)).length(Length::FillPortion(1))
                    )
            ).padding(15)

            .push(
                text(
                    if logi.koniec.is_empty(){
                        logi.koniec.to_string()
                    }else{"".to_string()}
                ).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
            )
            .push(
                text(
                    if logi.err.is_empty(){
                        logi.err.to_string()
                    }else{"".to_string()}
                ).font(jezyk.get_font()).color(Color::from_rgba(1., 0.5, 0.5, 0.7))
            )


            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .spacing(15)
            .padding(15)
}