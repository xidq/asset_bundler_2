use std::mem::discriminant;
use crate::ui::program_pomniejsze::kolory::{
    KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA,
    WYSOKOSC_CZCIONEK_PRZYCISKI,
};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoŁączeniaZdjęć;
use enumy::ikony::folder_icon;
use enumy::inne_ui::CheckActiveProcess;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowychPojedyncze, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, scrollable, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::path::PathBuf;

pub fn view_laczenie<'a>(
    dane: &DaneDoŁączeniaZdjęć,
    jezyk: &WybórJęzyka,

    main_process_check: &CheckActiveProcess,
) -> Element<'a, Message> {

    
    let check_opcjonalne_ścieżki=
        dane.sciezka_out.exists() &&
        !dane.nazwa.is_empty() &&
        (
            dane.sciezka_r.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_g.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_b.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_a.clone().unwrap_or(PathBuf::from("")).exists()
        );

    let lewa_kolumna = Column::new()
        //RED
        .push(
            Row::new()
                .push(
                    text("R:")
                        .color(Color::from_rgba(1., 0.5, 0.5, 0.7))
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieR))
                        .style(styl_przycisków(false, false, KOLOR_PEACH_PUFF))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_r.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieRPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki,KOLOR_PEACH_PUFF, KOLOR_TŁA))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //green
        .push(
            Row::new()
                .push(
                    text("G:")
                        .color(Color::from_rgba(0.5, 1., 0.5, 0.7))
                        .size(18.)
                        .font(jezyk.get_font())
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieG))
                        .style(styl_przycisków(false, false, KOLOR_PEACH_PUFF))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_g.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieGPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki,KOLOR_PEACH_PUFF, KOLOR_TŁA))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //blue
        .push(
            Row::new()
                .push(
                    text("B:")
                        .color(Color::from_rgba(0.5, 0.5, 1., 0.7))
                        .font(jezyk.get_font())
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieB))
                        .style(styl_przycisków(false, false, KOLOR_PEACH_PUFF))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane
                                .sciezka_b
                                .as_ref()
                                .unwrap_or(&PathBuf::new())
                                .to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieBPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki,KOLOR_PEACH_PUFF, KOLOR_TŁA))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //alpha
        .push(
            Row::new()
                .push(
                    text("A:")
                        .color(KOLOR_CZCIONKI_SREDNI)
                        .font(jezyk.get_font())
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieA))
                        .style(styl_przycisków(false, false, KOLOR_PEACH_PUFF))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane
                                .sciezka_a
                                .as_ref()
                                .unwrap_or(&PathBuf::new())
                                .to_string_lossy(),
                        )
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieAPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki,KOLOR_PEACH_PUFF, KOLOR_TŁA))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        .push(
            text("Ścieżka Wyjściowa:")
                .font(jezyk.get_font())
                .width(Length::Fill)
                .color(Color::from_rgba(1., 1., 1., 0.7))
                .size(18.)
                .center(),
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(
                        !dane.sciezka_out.to_string_lossy().is_empty() ,
                        if dane.sciezka_out.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_PEACH_PUFF,
                    ))
                    .padding(10)
                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzFolderOutFotoLaczenie))
                    .style(styl_przycisków(false, false, KOLOR_PEACH_PUFF))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_out.to_string_lossy(),
                        )
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieOutPathChanged(xx)))
                        .style(styl_text_input(dane.sciezka_out.exists(),KOLOR_PEACH_PUFF, KOLOR_TŁA))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(
            container(
                text_input(jezyk.t("input_name"), &dane.nazwa)
                    .padding(10)
                    .font(jezyk.get_font())
                    .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieNazwaChanged(xx)))
                    .style(styl_text_input(!dane.nazwa.is_empty(),KOLOR_PEACH_PUFF, KOLOR_TŁA))
                    .width(Length::Fill),
            )
            .height(Length::Fixed(40.))
            .width(Length::FillPortion(10)),
        )
        .push(if check_opcjonalne_ścieżki && *main_process_check == CheckActiveProcess::ProcessŻodyn {
            button(
                text(jezyk.t("process_btn_start"))
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.8))
                    .width(Length::Fill)
                    .center(),
            )
            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec))
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(false, false, KOLOR_SPANISH_ORANGE))
        } else {
            button(
                text(if *main_process_check == CheckActiveProcess::ProcessŁączenieZdjęć {
                    jezyk.t("btn_bussy_processing")
                } else if *main_process_check != CheckActiveProcess::ProcessŻodyn {
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
            .style(styl_przycisków(false, *main_process_check == CheckActiveProcess::ProcessŁączenieZdjęć, KOLOR_SPANISH_ORANGE))
        })
        .padding(15)
        .spacing(15)
        .width(Length::FillPortion(3));

    let prawa_kolumna = scrollable(
        Column::new()
            // match dane.out_format {
            //     OptRozszerzeniaPlikówZdjęciowych::Jpg { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Png { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Webp { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Tga { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Ff { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Qoi { .. } => {}
            // } => {}
            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg, dane,jezyk.get_font()))
            .push(if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg { jakosc, .. } = &dane.out_format {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=100,
                                        *jakosc,
                                        |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciJpg(xx)),
                                    )
                                        .style(styl_sliderów(KOLOR_PEACH_PUFF))
                                        .width(Length::FillPortion(6)).height(50.),
                                )
                                .push(space().width(Length::FillPortion(1)))
                                .push(
                                    text(format!("Q: {}%", jakosc))
                                        .color(Color::from_rgba(1., 1., 1., 0.6))
                                        .font(jezyk.get_font())
                                        .width(Length::FillPortion(4)).height(Length::Fill).center(),
                                ).height(Length::FillPortion(1)).padding(15)
                        )
                        .push(
                            text("Rgb").font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI).width(Length::Fill).height(Length::FillPortion(1)).center()
                        )

                ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))

            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png, dane,jezyk.get_font()))
            .push(if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Png { kompresja, bit_depth } = &dane.out_format{
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=9,
                                        *kompresja,
                                        |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiPng(xx)),
                                    )
                                        .style(styl_sliderów(KOLOR_PEACH_PUFF)).width(Length::FillPortion(6)).height(20.),
                                )
                                .push(
                                    text(format!("C: {}", kompresja))
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .height(Length::Fill)
                                        .width(Length::FillPortion(4))
                                        .center(),
                                ).padding(15)
                                .width(Length::FillPortion(12)),
                        ).height(Length::FillPortion(1))
                        .push(
                            Row::new()
                                .push(
                                    button(
                                        text("8bit")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(
                                        OptFormatyKoloruObrazOgólny::B8,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        *bit_depth == OptFormatyKoloruObrazOgólny::B8,
                                        KOLOR_PEACH_PUFF,
                                    )),
                                )
                                .push(
                                    button(
                                        text("8bit-a")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(
                                        OptFormatyKoloruObrazOgólny::B8a,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        *bit_depth == OptFormatyKoloruObrazOgólny::B8a,
                                        KOLOR_PEACH_PUFF,
                                    )),
                                )
                                .push(
                                    button(
                                        text("16bit")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(
                                        OptFormatyKoloruObrazOgólny::B16,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        *bit_depth == OptFormatyKoloruObrazOgólny::B16,
                                        KOLOR_PEACH_PUFF,
                                    )),
                                )
                                .push(
                                    button(
                                        text("16bit-a")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzeniePng(
                                        OptFormatyKoloruObrazOgólny::B16a,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        *bit_depth == OptFormatyKoloruObrazOgólny::B16a,
                                        KOLOR_PEACH_PUFF,
                                    )),
                                )

                        ).height(Length::FillPortion(1))


                ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))
            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp, dane,jezyk.get_font()))
            .push(if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp {jakosc, lossless, bit_depth,} = dane.out_format {
                container(
                    Column::new()
                        .push(
                            Row::new()

                            .push(
                                slider(
                                    0..=100,
                                    jakosc,
                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciWebp(xx)),
                                )
                                    .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
                            )
                            .push(
                                text(format!(
                                    "Q: {}%",
                                    jakosc
                                ))
                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                            ).padding(15).height(Length::FillPortion(1))

                        )
                        .push(
                            Row::new()
                                .push(
                                    button(
                                        text("Rgb")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieWebp(
                                        OptFormatyKoloruObrazOgólny::B8,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        bit_depth == OptFormatyKoloruObrazOgólny::B8,
                                        KOLOR_PEACH_PUFF,
                                    )).height(Length::Fill),
                                )
                                .push(
                                    button(
                                        text("Rgb-A")
                                            .color(Color::from_rgba(1., 1., 1., 0.6))
                                            .font(jezyk.get_font())
                                            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                            .width(Length::Fill)
                                            .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieWebp(
                                        OptFormatyKoloruObrazOgólny::B8a,
                                    )))
                                    .style(styl_przycisków(
                                        false,
                                        bit_depth == OptFormatyKoloruObrazOgólny::B8a,
                                        KOLOR_PEACH_PUFF,
                                    )).height(Length::Fill),
                                )
                                .push(
                                    button(
                                        text(if lossless {
                                            "Lossless"
                                        } else {
                                            "Lossy"
                                        })
                                        .color(Color::from_rgba(1., 1., 1., 0.6))
                                        .font(jezyk.get_font())
                                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                        .width(Length::Fill)
                                        .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianalosslessWebp))
                                    .style(styl_przycisków(
                                        false,
                                        lossless,
                                        KOLOR_PEACH_PUFF,
                                    )).height(Length::Fixed(50.)),
                                ).height(Length::FillPortion(1))

                        )
                ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))
            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga, dane,jezyk.get_font()))
            .push(
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Tga{ bit_depth } = dane.out_format{
                    container(
                        Column::new()
                            .push(space().height(Length::FillPortion(1)))
                            .push(
                                Row::new()
                                    .push(
                                        button(
                                            text("Hc16")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieTga(
                                                OptFormatyKoloruObrazuTga::HighColor16,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                bit_depth == OptFormatyKoloruObrazuTga::HighColor16,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Tc24")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieTga(
                                                OptFormatyKoloruObrazuTga::TrueColor24,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                bit_depth == OptFormatyKoloruObrazuTga::TrueColor24,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Tc32")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieTga(
                                                OptFormatyKoloruObrazuTga::TrueColorA32,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                bit_depth == OptFormatyKoloruObrazuTga::TrueColorA32,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))
                }else{
                    container(Column::new())
                }
            )



            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff, dane,jezyk.get_font()))
            .push(
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff{ metoda_kompresji } = dane.out_format{
                    container(
                        Column::new()
                            .push(
                                match metoda_kompresji{
                                    OptMetodaKompresjiZdjecia::Zstd(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=22,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfZstd(xx)),
                                                )
                                                    .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))

                                    }
                                    OptMetodaKompresjiZdjecia::Bzip2(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=9,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfBzip2(xx)),
                                                )
                                                    .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))
                                    }
                                    OptMetodaKompresjiZdjecia::Xz(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=9,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfXz(xx)),
                                                )
                                                    .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))
                                    }
                                    OptMetodaKompresjiZdjecia::Brak => {
                                        Row::new().height(Length::Fixed(50.))
                                    }
                                }
                            )
                            .push(
                                Row::new()
                                    .push(
                                        button(
                                            text("Brak")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(
                                                OptMetodaKompresjiZdjecia::Brak,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                metoda_kompresji == OptMetodaKompresjiZdjecia::Brak,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Zstd")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(
                                                OptMetodaKompresjiZdjecia::Zstd(3),
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                discriminant(&metoda_kompresji) == discriminant(&OptMetodaKompresjiZdjecia::Zstd(3)),
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Bzip2")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(
                                                OptMetodaKompresjiZdjecia::Bzip2(6),
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                discriminant(&metoda_kompresji) == discriminant(&OptMetodaKompresjiZdjecia::Bzip2(6)),
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Xz")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(
                                                OptMetodaKompresjiZdjecia::Xz(6),
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                discriminant(&metoda_kompresji) == discriminant(&OptMetodaKompresjiZdjecia::Xz(6)),
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))
                }else{
                    container(Column::new())
                }
            )

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(btn_zmiany_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi, dane,jezyk.get_font()))
            .push(
                if let OptRozszerzeniaPlikówZdjęciowychPojedyncze::Qoi{ bit_depth } = dane.out_format{
                    container(
                        Column::new()
                            .push(space().height(Length::FillPortion(1)))
                            .push(
                                Row::new()
                                    .push(
                                        button(
                                            text("Color24")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieQoi(
                                                OptFormatyKoloruObrazuQoi::Color24,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                bit_depth == OptFormatyKoloruObrazuQoi::Color24,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )
                                    .push(
                                        button(
                                            text("Color32")
                                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                                .font(jezyk.get_font())
                                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                                .width(Length::Fill)
                                                .center(),
                                        )
                                            .width(Length::FillPortion(1))
                                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieQoi(
                                                OptFormatyKoloruObrazuQoi::ColorA32,
                                            )))
                                            .style(styl_przycisków(
                                                false,
                                                bit_depth == OptFormatyKoloruObrazuQoi::ColorA32,
                                                KOLOR_PEACH_PUFF,
                                            )).height(Length::Fill),
                                    )

                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, KOLOR_PEACH_PUFF))
                }else{
                    container(Column::new())
                }
            )
            .padding(15)
            // .spacing(15)
            .width(Length::FillPortion(2)),
    );

    Row::new().push(lewa_kolumna).push(prawa_kolumna).into()
}

fn btn_zmiany_rozszerzenia<'a>(lell:OptRozszerzeniaPlikówZdjęciowychZnacznik, co_istnieje:&DaneDoŁączeniaZdjęć, font:iced::Font) -> Element<'a, Message>{

    let (nazwa,msg) = match lell {
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg => ("Jpg",
            OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg
        ),
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Png=> ("Png",
            OptRozszerzeniaPlikówZdjęciowychZnacznik::Png
        ),
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp => ("Webp",
            OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp
        ),
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga=> ("Tga",
             OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga
        ),
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff => ("FF",
            OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff
        ),
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi=> ("Qoi",
            OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi
        )
    };
    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(msg.clone()),
        ))
        .style(styl_przycisków(
            false,
            co_istnieje.tag == msg,
            KOLOR_PEACH_PUFF,
        )).into()
}