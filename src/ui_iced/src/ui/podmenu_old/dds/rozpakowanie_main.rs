use std::sync::Arc;
use iced::widget::{button, container, slider, space, text, text_input, tooltip, Column, Row};
use iced_core::{Color, Length};
use strum::{EnumMessage, IntoEnumIterator};
use enumy::dane_do_przetwarzania::{DaneDoPakowaniaDds, DaneDoRozpakowaniaDds};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, RodzajeContainer, StronyDds, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::{BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::kolor::{JpgQuant, JpgSamplingFac};
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::fn_ogolne::{btn_startu, btn_wyboru_kompresja_ff, btn_zbiorowe_bdepth, btn_zbiorowe_rozszerzenia};
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::metody_do_wariantow::bit_depth_dds::{btn_bdepth_jpg_dds, btn_bdepth_png_dds, btn_bdepth_qoi_dds, btn_bdepth_tga_dds, btn_bdepth_webp_dds, btn_komp_ff_dds};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_laczenie::btn_bdepth_png_laczenie;
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_dds;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;

pub fn dds_rozpakowanie_main<'a>(dane: Arc<&'a DaneDoRozpakowaniaDds>, logi: &'a LogRozpakowywanieDds, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    let var=dane.ścieżka_wejściowa.exists() && dane.ścieżka_wyjściowa.exists();

    Column::new()
        .push(
            text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(true, if dane.ścieżka_wejściowa.exists() { 2 } else { 0 }, kolor))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::RozpakInPathBtn))
                        .style(styl_przycisków(false, false, kolor, temat))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane.ścieżka_wejściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::RozpakInPath(xxx))).style(styl_text_input(dane.ścieżka_wyjściowa.exists(), kolor, temat))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(10))


                ).spacing(15)
        )
        .push(
            text(jezyk.t("ŚcieżkaWyjściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(true, if dane.ścieżka_wyjściowa.exists() { 1 } else { 0 }, kolor))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::RozpakOutPathBtn))
                        .style(styl_przycisków(false, false, kolor, temat))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane.ścieżka_wyjściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::RozpakOutPath(xxx))).style(styl_text_input(dane.ścieżka_wyjściowa.exists(), kolor, temat))
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
                            .on_input(|xx|Message::Dds(DdsMessage::PakowanieNazwa(xx))).style(styl_text_input(!dane.nazwa.is_empty(), kolor, temat))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(7))


                ).spacing(15)
        )
        .push(ui_standard_oddzielacz())
        .push(
            Column::new()
                
                .push(
                    RozszerzeniaZnacznik::iter()
                        .into_iter()
                        .fold(Row::new().spacing(1), |row, wariant| {
                            row.push(
                                rozszerzenie_dds(
                                    dane.clone(),
                                    wariant,
                                    kolor,
                                    jezyk.get_font(),
                                    temat
                                ),
                            )
                        })
                )
                

                .push(
                    match dane.rozszerzenie.clone()  {
                        Rozszerzenia::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans, } => {
                            let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    // .push(space().height(Length::Fixed(30.)))
                                    .push(
                                        Row::new()
                                            .push(
                                                text("")
                                            )
                                            .push(
                                                slider(
                                                    0..=100,
                                                    jakosc,
                                                    move |nowa_jakosc| {
                                                        Message::Dds(DdsMessage::RozpakExtDane(
                                                            Rozszerzenia::Jpg {
                                                                jakosc: nowa_jakosc,
                                                                progresywny,
                                                                bit_depth: bdepth.clone(),
                                                                sampling,
                                                                quant,
                                                                scans,
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(kolor,temat))
                                                    .width(Length::FillPortion(6))
                                            )
                                            .push(space().width(Length::FillPortion(1)))
                                            .push(
                                                text(format!("Q: {}%", jakosc))
                                                    .color(Color::from_rgba(1., 1., 1., 0.6))
                                                    .font(jezyk.get_font())
                                                    .width(Length::FillPortion(3)),
                                            ).height(Length::Fixed(50.)).padding(15)
                                    )
                                    .push(
                                        BdepthJpg::iter()
                                            .filter(|wariant|{!wariant.get_message().unwrap_or("brak danych").to_lowercase().contains("luma")})
                                            .into_iter()
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_jpg_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Oba,kolor,temat))
                        }
                        Rozszerzenia::Png{ kompresja, bit_depth } => {
                            let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    // .push(space().height(Length::Fixed(30.)))
                                    .push(
                                        Row::new()
                                            .push(
                                                text("")
                                            )
                                            .push(
                                                slider(
                                                    0..=9,
                                                    kompresja,
                                                    move |kompresja_nju| {
                                                        Message::Dds(DdsMessage::RozpakExtDane(
                                                            Rozszerzenia::Png {
                                                                kompresja: kompresja_nju,
                                                                bit_depth: bdepth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(kolor,temat))
                                                    .width(Length::FillPortion(6))
                                            )
                                            .push(space().width(Length::FillPortion(1)))
                                            .push(
                                                text(format!("Q: {}", kompresja))
                                                    .color(Color::from_rgba(1., 1., 1., 0.6))
                                                    .font(jezyk.get_font())
                                                    .width(Length::FillPortion(3)),
                                            ).height(Length::Fixed(50.)).padding(15)
                                    )
                                    .push(
                                        BdepthPng::iter()
                                            .step_by(2)
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_png_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                                    .push(
                                        BdepthPng::iter()
                                            .skip(1)
                                            .step_by(2)
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_png_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true, RodzajeContainer::Góra,kolor,temat))
                        }
                        Rozszerzenia::Webp{ jakosc, lossless, bit_depth } => {
                            let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    // .push(space().height(Length::Fixed(30.)))
                                    .push(
                                        Row::new()
                                            .push(
                                                text("")
                                            )
                                            .push(
                                                slider(
                                                    0..=100,
                                                    jakosc,
                                                    move |nowa_jakosc| {
                                                        Message::Dds(DdsMessage::RozpakExtDane(
                                                            Rozszerzenia::Webp {
                                                                jakosc: nowa_jakosc,
                                                                lossless,
                                                                bit_depth: bdepth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(kolor,temat))
                                                    .width(Length::FillPortion(6))
                                            )
                                            .push(space().width(Length::FillPortion(1)))
                                            .push(
                                                text(format!("Q: {}%", jakosc))
                                                    .color(Color::from_rgba(1., 1., 1., 0.6))
                                                    .font(jezyk.get_font())
                                                    .width(Length::FillPortion(3)),
                                            ).height(Length::Fixed(50.)).padding(15)
                                    )
                                    .push(
                                        BdepthWebp::iter()
                                            .into_iter()
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_webp_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
                        }
                        Rozszerzenia::Tga{ bit_depth } => {
                            container(
                                Column::new()
                                    .push(space().height(Length::Fixed(50.)))
                                    .push(
                                        BdepthTga::iter()
                                            .into_iter()
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_tga_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
                        }
                        Rozszerzenia::Ff{ metoda_kompresji } => {
                            
                            // let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    .push(
                                        match metoda_kompresji{
                                            OptMetodaKompresjiZdjecia::Zstd(xxx) => {
                                                Row::new()
                                                    .push(
                                                        slider(
                                                            1..=22,
                                                            xxx,
                                                            move |nowa_jakosc| {
                                                                Message::Dds(DdsMessage::RozpakExtDane(
                                                                    Rozszerzenia::Ff {
                                                                        metoda_kompresji: OptMetodaKompresjiZdjecia::Zstd(nowa_jakosc),
                                                                    }
                                                                ))
                                                            }
                                                        ).style(styl_sliderów(kolor,temat))
                                                            .width(Length::FillPortion(6)).height(50.)
                                                    )
                                                    .push(
                                                        text(format!("Q: {}", xxx))
                                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                                            .font(jezyk.get_font())
                                                            .width(Length::FillPortion(4)).center(),
                                                    ).height(Length::Fixed(50.)).padding(15)
                                                    .height(Length::Fixed(50.))
                                            }
                                            OptMetodaKompresjiZdjecia::Bzip2(xxx) => {
                                                Row::new()
                                                    .push(
                                                        slider(
                                                            1..=9,
                                                            xxx,
                                                            move |nowa_jakosc| {
                                                                Message::Dds(DdsMessage::RozpakExtDane(
                                                                    Rozszerzenia::Ff {
                                                                        metoda_kompresji: OptMetodaKompresjiZdjecia::Bzip2(nowa_jakosc),
                                                                    }
                                                                ))
                                                            }
                                                        ).style(styl_sliderów(kolor,temat))
                                                            .width(Length::FillPortion(6)).height(50.)
                                                    )
                                                    .push(
                                                        text(format!("Q: {}", xxx))
                                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                                            .font(jezyk.get_font())
                                                            .width(Length::FillPortion(4)).center(),
                                                    ).height(Length::Fixed(50.)).padding(15)
                                            }
                                            OptMetodaKompresjiZdjecia::Xz(xxx) => {
                                                Row::new()
                                                    .push(
                                                        slider(
                                                            1..=9,
                                                            xxx,
                                                            move |nowa_jakosc| {
                                                                Message::Dds(DdsMessage::RozpakExtDane(
                                                                    Rozszerzenia::Ff {
                                                                        metoda_kompresji: OptMetodaKompresjiZdjecia::Xz(nowa_jakosc),
                                                                    }
                                                                ))
                                                            }
                                                        ).style(styl_sliderów(kolor,temat))
                                                            .width(Length::FillPortion(6)).height(50.)
                                                    )
                                                    .push(
                                                        text(format!("Q: {}", xxx))
                                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                                            .font(jezyk.get_font())
                                                            .width(Length::FillPortion(4)).center(),
                                                    ).height(Length::Fixed(50.)).padding(15)

                                            }
                                            OptMetodaKompresjiZdjecia::Brak => { Row::new().height(Length::Fixed(50.))}
                                        }
                                    )
                                    .push(
                                        OptMetodaKompresjiZdjecia::iter()
                                            .into_iter()
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_komp_ff_dds(
                                                        wariant,
                                                        &metoda_kompresji,
                                                        jezyk.get_font(),kolor,temat
                                                    ),
                                                )
                                            })
                                        // Row::new()
                                        //     .push(btn_wyboru_kompresja_ff(
                                        //         "brak",
                                        //         OptMetodaKompresjiZdjecia::Brak,
                                        //         &metoda_kompresji,
                                        //         jezyk.get_font(),kolor,temat
                                        //     ))
                                        //     .push(btn_wyboru_kompresja_ff(
                                        //         "Zstd",
                                        //         OptMetodaKompresjiZdjecia::Zstd(3),
                                        //         &metoda_kompresji,
                                        //         jezyk.get_font(),kolor,temat
                                        //     ))
                                        //     .push(btn_wyboru_kompresja_ff(
                                        //         "Bzip2",
                                        //         OptMetodaKompresjiZdjecia::Bzip2(6),
                                        //         &metoda_kompresji,
                                        //         jezyk.get_font(),kolor,temat
                                        //     ))
                                        //     .push(btn_wyboru_kompresja_ff(
                                        //         "Xz",
                                        //         OptMetodaKompresjiZdjecia::Xz(6),
                                        //         &metoda_kompresji,
                                        //         jezyk.get_font(),kolor,temat
                                        //     ))

                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
                        }
                        Rozszerzenia::Qoi{ bit_depth } => {
                            container(
                                Column::new()
                                    .push(space().height(Length::Fixed(50.)))
                                    .push(
                                        BdepthQoi::iter()
                                            .into_iter()
                                            .fold(Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_qoi_dds(
                                                        wariant,
                                                        bit_depth.clone(),
                                                        jezyk.get_font(),
                                                        &temat.kolory.laczenie,
                                                        temat
                                                    ),
                                                )
                                            })
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
                        }
                        Rozszerzenia::Avif { .. } => {container(Column::new())}
                    }
                )
                .push(btn_startu(var,ActProces::DdsRozpakowanie, kolor, jezyk, temat))
        )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)

}