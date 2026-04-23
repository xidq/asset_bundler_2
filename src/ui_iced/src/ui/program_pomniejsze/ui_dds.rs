use crate::ui::program_pomniejsze::kolory::{KOLOR_COTTON_CANDY, KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_TŁA, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
// use enumy::inne_ui::WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych;
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use enumy::dane_do_przetwarzania::{DaneDoPakowaniaDds, DaneDoRozpakowaniaDds};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
use enumy::ikony::folder_icon;
pub(crate) use enumy::inne_ui::StronyDds;
use enumy::inne_ui::{StanKlikaczyDoLaczeniaZdjec, WybranyFormatZdjecia};
use enumy::opcje::{OptFormatDds, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptKompresjaDds, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, row, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn view_dds(
    dane_pakowanie: DaneDoPakowaniaDds,
    dane_rozpakowanie: DaneDoRozpakowaniaDds,
    wybrane_okno: &StronyDds,
    jezyk: WybórJęzyka,
    czy_jest_proces_zaczety: (bool,bool),
    log_pakowanie: LogPakowaniaDds,
    log_rozpakowywanie: LogRozpakowywanieDds,
    main_process_check: bool,
    stan_klikaczy: StanKlikaczyDoLaczeniaZdjec,
    wybrane_rozszerzenie:WybranyFormatZdjecia,
) -> Element<'_, Message> {

    let czy_sie_nada_na_wyslanie_pakowanie=dane_pakowanie.ścieżka_wejściowa.exists() && dane_pakowanie.ścieżka_wyjściowa.exists() && !dane_pakowanie.nazwa.is_empty();
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
                czy_jest_proces_zaczety.0,
                matches!(wybrane_okno, StronyDds::ZplikuDoDds),
                KOLOR_COTTON_CANDY,
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
                czy_jest_proces_zaczety.1,
                matches!(wybrane_okno, StronyDds::ZddsDoPliku),
                KOLOR_COTTON_CANDY,
            )),
        )
        // .push(space().height(Length::FillPortion(10)))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));

    let pakowanie = Column::new()
        .push(
            text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane_pakowanie.ścieżka_wejściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
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
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
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
                            .on_input(|xx|Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowej(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
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
                            .style(styl_text_input(!dane_pakowanie.nazwa.is_empty(),KOLOR_COTTON_CANDY, KOLOR_TŁA))
                            .on_input(|xx|Message::Dds(DdsMessage::DdsPakowanieZmianaNazwy(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
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
                |format| Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaWybranegoFormatu(format))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(KOLOR_COTTON_CANDY, KOLOR_TŁA))
                .menu_style(styl_menu_pick(KOLOR_COTTON_CANDY, KOLOR_TŁA)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            pick_list(
                OptKompresjaDds::iter().collect::<Vec<_>>(),
                Some(dane_pakowanie.kompresja),
                |kompresja| Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaWybranejKompresji(kompresja))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(KOLOR_COTTON_CANDY, KOLOR_TŁA))
                .menu_style(styl_menu_pick(KOLOR_COTTON_CANDY, KOLOR_TŁA)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            if czy_sie_nada_na_wyslanie_pakowanie && !czy_jest_proces_zaczety.0 && !main_process_check {
                button(
                    text(jezyk.t("process_btn_start"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .width(Length::Fill)
                        .center(),
                )
                    .on_press(Message::Dds(DdsMessage::DdsPakowanieWysylanieDanych))
                    .height(Length::Fixed(40.))
                    .width(Length::Fill)
                    .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
            } else {
                button(
                    text(if czy_jest_proces_zaczety.0 {
                        jezyk.t("btn_bussy_processing")
                    } else if main_process_check {
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
                        czy_jest_proces_zaczety.0,
                        KOLOR_COTTON_CANDY,
                    ))
            },
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
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wejściowa.exists() { 2 } else { 0 }, KOLOR_COTTON_CANDY))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
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
                            .on_input(|xxx|Message::Dds(DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
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
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wyjściowa.exists() { 1 } else { 0 }, KOLOR_COTTON_CANDY))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
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
                            .on_input(|xxx|Message::Dds(DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(), KOLOR_COTTON_CANDY, KOLOR_TŁA))
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
                            .style(styl_text_input(!dane_rozpakowanie.nazwa.is_empty(),KOLOR_COTTON_CANDY, KOLOR_TŁA))
                            .on_input(|xx|Message::Dds(DdsMessage::DdsPakowanieZmianaNazwy(xx))).style(styl_text_input(!dane_pakowanie.nazwa.is_empty(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
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
                    .push(
                        button(
                            text("Jpg")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Jpg{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Jpg{
                                jakosc: 90,
                                progresywny: false,
                                bit_depth: vec![OptFormatyKoloruObrazOgólny::B8],
                            }))
                            )

                    )
                    .push(
                        button(
                            text("png")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Png{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Png{
                                kompresja: 3,
                                bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                            }))
                            )
                    )
                    .push(
                        button(
                            text("webp")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Webp{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Webp{
                                jakosc: 90,
                                lossless: false,
                                bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                            }))
                            )
                    )
                    .push(
                        button(
                            text("tga")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Tga{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Tga{
                                bit_depth: Vec::from([OptFormatyKoloruObrazuTga::TrueColor24]),
                            }))
                            )
                    )
                    .push(
                        button(
                            text("ff")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Ff{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Ff{ metoda_kompresji: OptMetodaKompresjiZdjecia::Brak }))
                            )
                    )
                    .push(
                        button(
                            text("qoi")
                                .font(jezyk.get_font())
                                .width(Length::Fill)
                                .height(Length::Fixed(40.))
                                .center()
                        )
                            .style(styl_przycisków(false, matches!(dane_rozpakowanie.rozszerzenie,OptRozszerzeniaPlikówZdjęciowych::Qoi{..}), KOLOR_COTTON_CANDY))
                            .width(Length::FillPortion(1))
                            .on_press(Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(OptRozszerzeniaPlikówZdjęciowych::Qoi{ bit_depth: Vec::from([OptFormatyKoloruObrazuQoi::Color24]) }))
                            )
                    ).spacing(1)
                )
                
                .push(
                    match dane_rozpakowanie.rozszerzenie.clone()  {
                        OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, progresywny, bit_depth, } => {
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
                                                            Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Jpg {
                                                                    jakosc: nowa_jakosc,
                                                                    progresywny,
                                                                    bit_depth: bit_depth.clone()
                                                                }
                                                            ))
                                                        }
                                                    ).style(styl_sliderów(KOLOR_PEACH_PUFF))
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
                                            .push(
                                                button(
                                                    text("rgb")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B8),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Jpg{
                                                                    jakosc, progresywny, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B8]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("bw")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::L8),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Jpg{
                                                                    jakosc, progresywny, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::L8]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                    )
                            ).width(Length::Fill).height(100.).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
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
                                                        Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                            OptRozszerzeniaPlikówZdjęciowych::Png {
                                                                kompresja: kompresja_nju,
                                                                bit_depth: bit_depth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(KOLOR_PEACH_PUFF))
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
                                            .push(
                                                button(
                                                    text("8l")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::L8),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::L8]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("8b")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B8),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B8]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("16l")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::L16),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::L16]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("16b")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B16),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B16]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                    )
                                    .push(
                                        Row::new()
                                            .push(
                                                button(
                                                    text("8la")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::L8a),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::L8a]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("8ba")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B8a),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B8a]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("16la")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::L16a),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::L16a]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("16ba")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B16a),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Png{
                                                                    kompresja, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B16a]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                    )
                            ).width(Length::Fill).height(150.).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
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
                                                        Message::Dds(DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                            OptRozszerzeniaPlikówZdjęciowych::Webp {
                                                                jakosc: nowa_jakosc,
                                                                lossless,
                                                                bit_depth: bit_depth.clone()
                                                            }
                                                        ))
                                                    }
                                                ).style(styl_sliderów(KOLOR_PEACH_PUFF))
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
                                            .push(
                                                button(
                                                    text("rgb")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B8),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Webp{
                                                                    jakosc, lossless, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B8]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                            .push(
                                                button(
                                                    text("alpha")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fixed(40.))
                                                        .center()
                                                )
                                                    .style(
                                                        styl_przycisków(
                                                            false,
                                                            bdepth.contains(&OptFormatyKoloruObrazOgólny::B8a),
                                                            KOLOR_COTTON_CANDY
                                                        )
                                                    )
                                                    .on_press(
                                                        Message::Dds(
                                                            DdsMessage::DdsRozpakowaniZemianaRozszerzeniaDane(
                                                                OptRozszerzeniaPlikówZdjęciowych::Webp{
                                                                    jakosc, lossless, bit_depth: Vec::from(
                                                                        [OptFormatyKoloruObrazOgólny::B8a]
                                                                    )
                                                                }
                                                            )
                                                        )
                                                    )
                                                    .height(Length::FillPortion(1))
                                            )
                                    )
                            ).width(Length::Fill).height(100.).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Tga{ bit_depth } => {
                            container(text("")).width(Length::Fill).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Ff{ metoda_kompresji } => {
                            container(text("")).width(Length::Fill).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
                        }
                        OptRozszerzeniaPlikówZdjęciowych::Qoi{ bit_depth } => {
                            container(text("")).width(Length::Fill).style(styl_kontenera(true,KOLOR_COTTON_CANDY))
                        }
                    }
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

    row![lewa, prawa].into()
}
