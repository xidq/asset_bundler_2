use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::progress_bar::styl_progress_bar;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use enumy::dane_do_przetwarzania::{DaneDoPakowaniaDds, DaneDoRozpakowaniaDds};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
use enumy::ikony::folder_icon;
pub(crate) use enumy::inne_ui::StronyDds;
use enumy::inne_ui::{ActProces, RodzajeContainer, UstawieniaThemeWsio};
use enumy::opcje::{JpgQuant, JpgSamplingFac, OptFormatDds, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptKompresjaDds, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, progress_bar, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

pub fn view_dds<'a>(
    dane_pakowanie: &'a DaneDoPakowaniaDds,
    dane_rozpakowanie: &'a DaneDoRozpakowaniaDds,
    wybrane_okno: &'a StronyDds,
    jezyk: &'a WybórJęzyka,
    log_pakowanie: &'a LogPakowaniaDds,
    _log_rozpakowywanie: &'a LogRozpakowywanieDds,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    let valid =  temat.temp.aktywny_proces == ActProces::Żodyn;

    // let print_sciezki = |xx:&PathBuf| -> String{
    //     let strinkku=|| -> String {
    //         let yy = xx.to_str().unwrap_or("");
    //         let char_count = yy.chars().count();
    //
    //         if char_count <= 20 {
    //             yy.to_string()
    //         } else {
    //             let start = yy.char_indices().nth(15).map(|(i, _)| i).unwrap_or(yy.len());
    //             let fin = yy.char_indices().nth_back(9).map(|(i, _)| i).unwrap_or(0);
    //             yy[0..=start].to_string() + "..." + &yy[fin ..]
    //         }
    //     };
    //
    //     if xx.exists() && xx.is_file(){
    //         "[Plik]  ".to_string() + &strinkku()
    //     } else{
    //         "[Folder]  ".to_string() + &strinkku()
    //     }
    // };
    fn print_sciezki(xx: &Path) -> String {
        let starrrrtuuuuuu = xx.components().nth(1).unwrap().as_os_str().to_string_lossy().to_string();
        let endo = xx.file_name().unwrap().to_string_lossy().to_string();

        let poczatek = if xx.to_string_lossy().len() < 30 {
            xx.to_string_lossy().to_string()
        } else if starrrrtuuuuuu.len() < 30 {
            let secnd = xx.components().nth(2).unwrap().as_os_str().to_string_lossy().to_string();
            starrrrtuuuuuu + "/" + &secnd + "/"
        } else {
            starrrrtuuuuuu + "/"
        };

        if xx.to_string_lossy().len() < 30 {
            poczatek
        } else {
            poczatek + " ... " + "/" + endo.as_str()
        }
    }

    fn tekst_sciezek<'a>(
        sciezki: &'a Option<Vec<PathBuf>>,
        yy: usize,
        jezyk: &'a WybórJęzyka,
        f_formatuj: &dyn Fn(&Path) -> String // przekazujemy logikę formatowania
    ) -> Element<'a, Message> {
        text(match sciezki {
            Some(xx) if yy < xx.len() => f_formatuj(&xx[yy]),
            _ => "".to_string(),
        })
            .color(KOLOR_CZCIONKI_SREDNI)
            .font(jezyk.get_font())
            .height(20.)
            .into() // Ważne: rzutowanie na Element<'a>
    }

    fn btn_rozszerzenia_ogolne<'a>(
        format:OptFormatyKoloruObrazOgólny,
        tag:OptRozszerzeniaPlikówZdjęciowychZnacznik,
        
        bdepth: Vec<OptFormatyKoloruObrazOgólny>,
        jezyk: &'a WybórJęzyka,
        kolor: &'a Color,
        temat: &'a UstawieniaThemeWsio,
    )-> Element<'a, Message>{

        button(
            text(format.krotka().to_string())
                .font(jezyk.get_font())
                .width(Length::Fill)
                .height(Length::Fixed(40.))
                .center()
        )
            .style(
                styl_przycisków(
                    false,
                    bdepth.contains(&format),
                    kolor,
                    temat
                )
            )
            .on_press(
                Message::Dds(
                    DdsMessage::RozpakBitDepth(
                        tag,format
                    )
                )
            )
            .height(Length::FillPortion(1)).into()
    }

    let czy_sie_nada_na_wyslanie_pakowanie=dane_pakowanie.ścieżka_wejściowa.is_some() && dane_pakowanie.ścieżka_wyjściowa.exists() && !dane_pakowanie.nazwa.is_empty();
    let czy_sie_nada_na_wyslanie_rozpakowanie=dane_rozpakowanie.ścieżka_wejściowa.exists() && dane_rozpakowanie.ścieżka_wyjściowa.exists();

    let lewa = Column::new()
        // .push(space().height(Length::FillPortion(10)))
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZplikuDoDds)))
            .style(styl_przycisków(
                temat.temp.aktywny_proces == ActProces::DdsPakowanie,
                matches!(wybrane_okno, StronyDds::ZplikuDoDds),
                &temat.kolory.dds, temat
            )),
        )
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZddsDoPliku)))
            .style(styl_przycisków(
                temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
                matches!(wybrane_okno, StronyDds::ZddsDoPliku),
                &temat.kolory.dds, temat
            )),
        )
        // .push(space().height(Length::FillPortion(10)))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));

    let pakowanie = 
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
                                .style(styl_przycisków(false, false, &temat.kolory.dds,temat))
                                .width(Length::Fixed(40.))
                                .height(Length::Fixed(40.))
                        )
                        .push(
                            button(text("folder").width(Length::Fill).center())
                                .padding(10)
                                .on_press(Message::Dds(DdsMessage::PakowaniePathInFolders))
                                .style(styl_przycisków(false, false, &temat.kolory.dds,temat))
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
                    match dane_pakowanie.ścieżka_wejściowa.clone().unwrap_or(Vec::from([PathBuf::from("")])).len(){
                        0 => {
                            Column::new()
                                .push(
                                    text("Nie wybrano plików lub folderów").height(Length::Fixed(20.))
                                )
                                .push(space().height(Length::Fixed(60.)))
                            },
                        1 => {
                            Column::new()
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 0,jezyk, &print_sciezki))
                                .push(space().height(Length::Fixed(60.)))
                            },
                        2 => {
                            Column::new()
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 0, jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 1,jezyk, &print_sciezki))
                                .push(space().height(Length::Fixed(40.)))
                            },
                        3 => {
                            Column::new()
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 0,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 1,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 2,jezyk, &print_sciezki))
                                .push(space().height(Length::Fixed(20.)))
                            },
                        4 => {
                            Column::new()
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 0,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 1,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 2,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 3,jezyk, &print_sciezki))
                        }
                        _ => {
                            Column::new()
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 0,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 1,jezyk, &print_sciezki))
                                .push(tekst_sciezek(&dane_pakowanie.ścieżka_wejściowa, 2,jezyk, &print_sciezki))
                                .push(
                                    text(format!("i {} więcej",dane_pakowanie.ścieżka_wejściowa.clone().unwrap().len() - 3)).height(Length::Fixed(20.))
                                )
                            },
                    }
                )
                // .push(
                    // container(
                    //     text_input(
                    //         jezyk.t("input_folder_or_file"),
                    //         &dane_pakowanie.ścieżka_wejściowa.to_string_lossy()
                    //     ).font(jezyk.get_font())
                    //         .padding(10)
                    //         .on_input(|xxx|Message::Dds(DdsMessage::DdsPakowanieZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(),  &temat.kolory.dds,KOLOR_TŁA))
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
                        .style(styl_przycisków(false, false, &temat.kolory.dds,temat))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane_pakowanie.ścieżka_wyjściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xx|Message::Dds(DdsMessage::PakowaniePathOut(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(), &temat.kolory.dds, temat))
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
                            &dane_pakowanie.nazwa
                        ).font(jezyk.get_font())
                            .padding(10)
                            .style(styl_text_input(!dane_pakowanie.nazwa.is_empty(),&temat.kolory.dds, temat))
                            .on_input(|xx|Message::Dds(DdsMessage::PakowanieNazwa(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(), &temat.kolory.dds, temat))
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
                Some(dane_pakowanie.format),
                |format| Message::Dds(DdsMessage::PakowanieFormat(format))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(&temat.kolory.dds, temat))
                .menu_style(styl_menu_pick(&temat.kolory.dds, temat)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            pick_list(
                OptKompresjaDds::iter().collect::<Vec<_>>(),
                Some(dane_pakowanie.kompresja),
                |kompresja| Message::Dds(DdsMessage::PakowanieKompresja(kompresja))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(&temat.kolory.dds, temat))
                .menu_style(styl_menu_pick(&temat.kolory.dds, temat)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            if czy_sie_nada_na_wyslanie_pakowanie && valid {
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
                    .style(styl_przycisków(false, false, &temat.kolory.dds,temat))
            } else {
                button(
                    text(if temat.temp.aktywny_proces == ActProces::DdsPakowanie {
                        jezyk.t("btn_bussy_processing")
                    } else if !valid {
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
                        &temat.kolory.dds, temat
                    ))
            },
        )
        // .push(space().height(Length::Fixed(15.)))
        .push(
            Row::new()
                .push(text(format!("Postęp procesu:  {}",log_pakowanie.w_trakcie)).font(jezyk.get_font()).width(Length::FillPortion(1)).center())
                .push(
                    progress_bar(0.0..=100., log_pakowanie.w_trakcie as f32)
                        .girth(18.)
                        .style(styl_progress_bar(&temat.kolory.dds, temat)).length(Length::FillPortion(1))
                )
        ).padding(15)

        .push(
            text(
                if log_pakowanie.koniec.is_empty(){
                    log_pakowanie.koniec.to_string()
                }else{"".to_string()}
            ).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            text(
                if log_pakowanie.err.is_empty(){
                    log_pakowanie.err.to_string()
                }else{"".to_string()}
            ).font(jezyk.get_font()).color(Color::from_rgba(1., 0.5, 0.5, 0.7))
        )


        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .spacing(15)
        .padding(15);

    let rozpakowanie = Column::new()
        .push(
            text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wejściowa.exists() { 2 } else { 0 }, &temat.kolory.dds))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::RozpakInPathBtn))
                        .style(styl_przycisków(false, false, &temat.kolory.dds, temat))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane_rozpakowanie.ścieżka_wejściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::RozpakInPath(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(), &temat.kolory.dds, temat))
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
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wyjściowa.exists() { 1 } else { 0 }, &temat.kolory.dds))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::RozpakOutPathBtn))
                        .style(styl_przycisków(false, false, &temat.kolory.dds, temat))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane_rozpakowanie.ścieżka_wyjściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::RozpakOutPath(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(), &temat.kolory.dds, temat))
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
                            &dane_rozpakowanie.nazwa
                        ).font(jezyk.get_font())
                            .padding(10)
                            .style(styl_text_input(!dane_rozpakowanie.nazwa.is_empty(),&temat.kolory.dds, temat))
                            .on_input(|xx|Message::Dds(DdsMessage::PakowanieNazwa(xx))).style(styl_text_input(!dane_pakowanie.nazwa.is_empty(), &temat.kolory.dds, temat))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(7))


                ).spacing(15)
        )
        .push(ui_standard_oddzielacz())
        .push(
            Column::new()
                .push(
                    Row::new()
                    .push(przycisk_wyboru_rozszerzenia(
                        "jpg",
                        OptRozszerzeniaPlikówZdjęciowych::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                            sampling: JpgSamplingFac::R420,
                            quant: JpgQuant::Default,
                            scans: 4,
                        },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    )
                    .push(przycisk_wyboru_rozszerzenia(
                        "png",
                        OptRozszerzeniaPlikówZdjęciowych::Png {
                            kompresja: 3,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                        },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    )
                    .push(przycisk_wyboru_rozszerzenia(
                        "webp",
                        OptRozszerzeniaPlikówZdjęciowych::Webp {
                            jakosc: 90,
                            lossless: false,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                        },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    )
                    .push(przycisk_wyboru_rozszerzenia(
                        "tga",
                        OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth: Vec::from([OptFormatyKoloruObrazuTga::TrueColor24]) },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    )
                    .push(przycisk_wyboru_rozszerzenia(
                        "ff",
                        OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji: OptMetodaKompresjiZdjecia::Brak },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    )
                    .push(przycisk_wyboru_rozszerzenia(
                        "qoi",
                        OptRozszerzeniaPlikówZdjęciowych::Qoi {
                            bit_depth: Vec::from([OptFormatyKoloruObrazuQoi::Color24]),
                        },
                        &dane_rozpakowanie.rozszerzenie,
                        jezyk.get_font(),temat)
                    ).spacing(1)
                )
                
                .push(
                    match dane_rozpakowanie.rozszerzenie.clone()  {
                        OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans, } => {
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
                                                                OptRozszerzeniaPlikówZdjęciowych::Jpg {
                                                                    jakosc: nowa_jakosc,
                                                                    progresywny,
                                                                    bit_depth: bit_depth.clone(),
                                                                    sampling,
                                                                    quant,
                                                                    scans,
                                                                }
                                                            ))
                                                        }
                                                    ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                        Row::new()
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B8,OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,bdepth.clone(),jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::L8,OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,bdepth.clone(),jezyk,&temat.kolory.dds,temat))
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Oba,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Png{ kompresja, bit_depth } => {
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
                                                            OptRozszerzeniaPlikówZdjęciowych::Png {
                                                                kompresja: kompresja_nju,
                                                                bit_depth: bit_depth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                        Row::new()
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::L8,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B8,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::L16,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B16,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                    )
                                    .push(
                                        Row::new()
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::L8a,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B8a,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::L16a,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B16a,OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                    )
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Webp{ jakosc, lossless, bit_depth } => {

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
                                                            OptRozszerzeniaPlikówZdjęciowych::Webp {
                                                                jakosc: nowa_jakosc,
                                                                lossless,
                                                                bit_depth: bit_depth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                        Row::new()
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B8,OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                            .push(btn_rozszerzenia_ogolne(OptFormatyKoloruObrazOgólny::B8a,OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp,bdepth.clone(), jezyk,&temat.kolory.dds,temat))
                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Tga{ bit_depth } => {
                            let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    .push(space().height(Length::Fixed(50.)))
                                    .push(
                                        Row::new()
                                            .push(przycisk_wyboru_bit_depth_tga(
                                                "szary",
                                                OptFormatyKoloruObrazuTga::Szary8,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_depth_tga(
                                                "HC16",
                                                OptFormatyKoloruObrazuTga::HighColor16,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_depth_tga(
                                                "TC24",
                                                OptFormatyKoloruObrazuTga::TrueColor24,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_depth_tga(
                                                "TC32",
                                                OptFormatyKoloruObrazuTga::TrueColorA32,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))

                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Ff{ metoda_kompresji } => {
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
                                                                    OptRozszerzeniaPlikówZdjęciowych::Ff {
                                                                        metoda_kompresji: OptMetodaKompresjiZdjecia::Zstd(nowa_jakosc),
                                                                    }
                                                                ))
                                                            }
                                                        ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                                                OptRozszerzeniaPlikówZdjęciowych::Ff {
                                                                    metoda_kompresji: OptMetodaKompresjiZdjecia::Bzip2(nowa_jakosc),
                                                                }
                                                            ))
                                                        }
                                                    ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                                                    OptRozszerzeniaPlikówZdjęciowych::Ff {
                                                                        metoda_kompresji: OptMetodaKompresjiZdjecia::Xz(nowa_jakosc),
                                                                    }
                                                                ))
                                                            }
                                                        ).style(styl_sliderów(&temat.kolory.dds,temat))
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
                                        Row::new()
                                            .push(przycisk_wyboru_bit_kompresja_ff(
                                                "brak",
                                                OptMetodaKompresjiZdjecia::Brak,
                                                &metoda_kompresji,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_kompresja_ff(
                                                "Zstd",
                                                OptMetodaKompresjiZdjecia::Zstd(3),
                                                &metoda_kompresji,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_kompresja_ff(
                                                "Bzip2",
                                                OptMetodaKompresjiZdjecia::Bzip2(6),
                                                &metoda_kompresji,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_kompresja_ff(
                                                "Xz",
                                                OptMetodaKompresjiZdjecia::Xz(6),
                                                &metoda_kompresji,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))

                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Qoi{ bit_depth } => {
                            let bdepth = bit_depth.clone();
                            container(
                                Column::new()
                                    .push(space().height(Length::Fixed(50.)))
                                    .push(
                                        Row::new()
                                            .push(przycisk_wyboru_bit_qoi(
                                                "szary",
                                                OptFormatyKoloruObrazuQoi::Color24,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))
                                            .push(przycisk_wyboru_bit_qoi(
                                                "HC16",
                                                OptFormatyKoloruObrazuQoi::ColorA32,
                                                &bdepth,
                                                jezyk.get_font(),&temat.kolory.dds,temat
                                            ))


                                    )
                                    .push(space().height(Length::Fixed(50.)))
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,&temat.kolory.dds,temat))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Avif { .. } => {container(Column::new())}
                    }
                )
                .push(
                    if czy_sie_nada_na_wyslanie_rozpakowanie && valid {
                        button(
                            text(jezyk.t("process_btn_start"))
                                .font(jezyk.get_font())
                                .color(Color::from_rgba(1., 1., 1., 0.8))
                                .width(Length::Fill)
                                .center(),
                        )
                            .on_press(Message::Dds(DdsMessage::RozpakStart))
                            .height(Length::Fixed(40.))
                            .width(Length::Fill)
                            .style(styl_przycisków(false, false, &temat.kolory.dds,temat))
                    } else {
                        button(
                            text(if temat.temp.aktywny_proces == ActProces::DdsRozpakowanie {
                                jezyk.t("btn_bussy_processing")
                            } else if !valid {
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
                                temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
                                &temat.kolory.dds, temat
                            ))
                    },
                )
        )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .spacing(15)
        .padding(15);

    let prawa = match wybrane_okno {
        StronyDds::ZplikuDoDds => pakowanie,
        StronyDds::ZddsDoPliku => rozpakowanie,
    };

    Row::new().push(lewa).push(prawa).into()
}
fn przycisk_wyboru_rozszerzenia<'a>(
    etykieta: &'static str,
    target: OptRozszerzeniaPlikówZdjęciowych,
    obecne: &'a OptRozszerzeniaPlikówZdjęciowych,
    font: iced::Font,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    // Sprawdzamy, czy ten przycisk reprezentuje obecnie wybrany format
    // Używamy std::mem::discriminant, żeby porównać warianty bez przejmowania się ich zawartością
    let aktywny = std::mem::discriminant(&target) == std::mem::discriminant(obecne);

    button(
        text(etykieta)
            .font(font)
            .width(Length::Fill)
            .height(Length::Fixed(40.))
            .center()
    )
        .style(styl_przycisków(false, aktywny, &temat.kolory.dds,temat))
        .width(Length::FillPortion(1))
        .on_press(Message::Dds(DdsMessage::RozpakExtDane(target)))
        .into()
}
fn przycisk_wyboru_bit_depth_tga<'a>(
    etykieta: &'static str,
    bit_target: OptFormatyKoloruObrazuTga, // konkretny bit, np. HighColor16
    obecna_lista_bitow: &[OptFormatyKoloruObrazuTga], // Twoje bdepth
    font: iced::Font, // przekazujemy obiekt języka dla fontu
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    // Sprawdzamy czy ten konkretny bit jest na liście
    let aktywny = obecna_lista_bitow.contains(&bit_target);

    button(
        text(etykieta)
            .font(font)
            .width(Length::Fill)
            .height(Length::Fixed(40.))
            .center()
    )
        .style(styl_przycisków(false, aktywny, kolor,temat))
        .on_press(
            Message::Dds(
                DdsMessage::RozpakExtDane(
                    OptRozszerzeniaPlikówZdjęciowych::Tga {
                        bit_depth: Vec::from([bit_target]) // tworzymy nową listę z tym jednym bitem
                    }
                )
            )
        )
        .height(Length::FillPortion(1))
        .into()
}
fn przycisk_wyboru_bit_kompresja_ff<'a>(
    etykieta: &'static str,
    kompresja_target: OptMetodaKompresjiZdjecia, // konkretny bit, np. HighColor16
    obecna_kompresja: &OptMetodaKompresjiZdjecia, // Twoje bdepth
    font: iced::Font, // przekazujemy obiekt języka dla fontu
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    // Sprawdzamy czy ten konkretny bit jest na liście
    let aktywny = std::mem::discriminant(&kompresja_target) == std::mem::discriminant(obecna_kompresja);

    button(
        text(etykieta)
            .font(font)
            .width(Length::Fill)
            .height(Length::Fixed(40.))
            .center()
    )
        .style(styl_przycisków(false, aktywny, kolor,temat))
        .on_press(
            Message::Dds(
                DdsMessage::RozpakExtDane(
                    OptRozszerzeniaPlikówZdjęciowych::Ff {
                        metoda_kompresji: kompresja_target // tworzymy nową listę z tym jednym bitem
                    }
                )
            )
        )
        .height(Length::FillPortion(1))
        .into()
}
fn przycisk_wyboru_bit_qoi<'a>(
    etykieta: &'static str,
    bit_target: OptFormatyKoloruObrazuQoi, // konkretny bit, np. HighColor16
    bit_teraz: &[OptFormatyKoloruObrazuQoi], // Twoje bdepth
    font: iced::Font, // przekazujemy obiekt języka dla fontu
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    // Sprawdzamy czy ten konkretny bit jest na liście
    let aktywny = bit_teraz.contains(&bit_target);

    button(
        text(etykieta)
            .font(font)
            .width(Length::Fill)
            .height(Length::Fixed(40.))
            .center()
    )
        .style(styl_przycisków(false, aktywny, kolor,temat))
        .on_press(
            Message::Dds(
                DdsMessage::RozpakExtDane(
                    OptRozszerzeniaPlikówZdjęciowych::Qoi {
                        bit_depth: Vec::from([bit_target]) // tworzymy nową listę z tym jednym bitem
                    }
                )
            )
        )
        .height(Length::FillPortion(1))
        .into()
}

