use crate::ui::program_pomniejsze::kolory::{
    KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA,
    WYSOKOSC_CZCIONEK_PRZYCISKI,
};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use enumy::dane_do_przetwarzania::DaneDoŁączeniaZdjęć;
use enumy::ikony::folder_icon;
use enumy::opcje::OptFormatyKoloruObrazOgólny;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, scrollable, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::path::PathBuf;
use enumy::inne_ui::StanKlikaczyDoLaczeniaZdjec;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;

pub fn view_laczenie<'a>(
    dane: DaneDoŁączeniaZdjęć,
    jezyk: WybórJęzyka,
    stan_klikaczy: StanKlikaczyDoLaczeniaZdjec,
    is_loading: bool,
    main_process_check: bool,
) -> Element<'a, Message> {
    let valid = stan_klikaczy.obraz_r_wybrany
        || stan_klikaczy.obraz_b_wybrany
        || stan_klikaczy.obraz_g_wybrany
        || stan_klikaczy.obraz_a_wybrany
        || stan_klikaczy.jpg_wybrany
        || stan_klikaczy.png_wybrany
            && (stan_klikaczy.png_wybrane_8bita
                || stan_klikaczy.png_wybrane_8bit
                || stan_klikaczy.png_wybrane_16bit
                || stan_klikaczy.png_wybrane_16bita)
        || stan_klikaczy.webp_wybrany
            && (stan_klikaczy.webp_wybrany_alpha || stan_klikaczy.webp_wybrany_rgb);
    
    let check_opcjonalne_ścieżki= 
        dane.sciezka_r.clone().unwrap_or(PathBuf::from("")).exists() ||
        dane.sciezka_g.clone().unwrap_or(PathBuf::from("")).exists() ||
        dane.sciezka_b.clone().unwrap_or(PathBuf::from("")).exists() ||
        dane.sciezka_a.clone().unwrap_or(PathBuf::from("")).exists();

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
        .push(if valid && !is_loading && !main_process_check {
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
                text(if is_loading {
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
            .style(styl_przycisków(false, is_loading, KOLOR_SPANISH_ORANGE))
        })
        .padding(15)
        .spacing(15)
        .width(Length::FillPortion(3));

    let prawa_kolumna = scrollable(
        Column::new()
            .push(if stan_klikaczy.jpg_wybrany {
                Column::new()
                    .push(
                        button(
                            text("Jpg")
                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(
                            "jpg".to_string()),
                        ))
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.jpg_wybrany,
                            KOLOR_PEACH_PUFF,
                        )),
                    )
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=100,
                                    stan_klikaczy.jpg_jakosc,
                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciJpg(xx)),
                                )
                                .style(styl_sliderów(KOLOR_PEACH_PUFF))
                                .width(Length::FillPortion(6)),
                            )
                            .push(space().width(Length::FillPortion(1)))
                            .push(
                                text(format!("Q: {}%", stan_klikaczy.jpg_jakosc))
                                    .color(Color::from_rgba(1., 1., 1., 0.6))
                                    .font(jezyk.get_font())
                                    .width(Length::FillPortion(3)),
                            ),
                    )
                    .spacing(15)
            } else {
                Column::new()
                    .push(
                        button(
                            text("Jpg")
                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(
                            "jpg".to_string()),
                        ))
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.jpg_wybrany,
                            KOLOR_PEACH_PUFF,
                        )),
                    )
                    .spacing(0)
            })
            .push(ui_standard_oddzielacz())
            .push(if stan_klikaczy.png_wybrany {
                Column::new()
                    .push(
                        button(
                            text("Png")
                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(
                            "png".to_string()),
                        ))
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.png_wybrany,
                            KOLOR_PEACH_PUFF,
                        )),
                    )
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
                                    stan_klikaczy.png_wybrane_8bit,
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
                                    stan_klikaczy.png_wybrane_8bita,
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
                                    stan_klikaczy.png_wybrane_16bit,
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
                                    stan_klikaczy.png_wybrane_16bita,
                                    KOLOR_PEACH_PUFF,
                                )),
                            )
                            .spacing(2),
                    )
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=9,
                                    stan_klikaczy.png_kompresja,
                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiPng(xx)),
                                )
                                .style(styl_sliderów(KOLOR_PEACH_PUFF)),
                            )
                            .push(
                                text(format!("Kompresja: {}", stan_klikaczy.png_kompresja))
                                    .font(jezyk.get_font())
                                    .color(KOLOR_CZCIONKI_SREDNI)
                                    .width(Length::Fill)
                                    .center(),
                            )
                            .width(Length::FillPortion(12)),
                    )
                    .spacing(15)
            } else {
                Column::new()
                    .push(
                        button(
                            text("Png")
                                .color(Color::from_rgba(1., 1., 1., 0.6))
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(
                            "png".to_string(),
                        )))
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.png_wybrany,
                            KOLOR_PEACH_PUFF,
                        )),
                    )
                    .spacing(0)
            })
            .push(ui_standard_oddzielacz())
            .push(if stan_klikaczy.webp_wybrany {
                Column::new()
                    .push(
                        button(
                            text("webp")
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .padding(10)
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.webp_wybrany,
                            KOLOR_PEACH_PUFF,
                        ))
                        .width(Length::FillPortion(5)),
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
                                    stan_klikaczy.webp_wybrany_rgb,
                                    KOLOR_PEACH_PUFF,
                                )),
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
                                    stan_klikaczy.webp_wybrany_alpha,
                                    KOLOR_PEACH_PUFF,
                                )),
                            )
                            .push(
                                button(
                                    text(if stan_klikaczy.webp_lossless {
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
                                    !stan_klikaczy.webp_lossless,
                                    stan_klikaczy.webp_lossless,
                                    KOLOR_PEACH_PUFF,
                                )),
                            )
                            .spacing(2),
                    )
                    .push(if stan_klikaczy.webp_lossless {
                        Row::new()
                            .push(
                                slider(0..=100, stan_klikaczy.webp_jakosc, Message::DoNothingU8xD)
                                    .style(styl_sliderów(KOLOR_TŁA)),
                            )
                            .push(
                                text(format!(
                                    "{} {}%",
                                    jezyk.t("foto_edit_quality"),
                                    stan_klikaczy.webp_jakosc
                                ))
                                .color(Color::from_rgba(1., 1., 1., 0.5))
                                .font(jezyk.get_font()),
                            )
                            .width(Length::FillPortion(5))
                    } else {
                        Row::new()
                            .push(
                                slider(
                                    0..=100,
                                    stan_klikaczy.webp_jakosc,
                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciWebp(xx)),
                                )
                                .style(styl_sliderów(KOLOR_PEACH_PUFF)),
                            )
                            .push(
                                text(format!(
                                    "{} {}%",
                                    jezyk.t("foto_edit_quality"),
                                    stan_klikaczy.webp_jakosc
                                ))
                                .color(Color::from_rgba(1., 1., 1., 0.5))
                                .font(jezyk.get_font()),
                            )
                            .width(Length::FillPortion(5))
                    })
                    .spacing(15)
            } else {
                Column::new()
                    .push(
                        button(
                            text("Webp")
                                .font(jezyk.get_font())
                                .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                .width(Length::Fill)
                                .center(),
                        )
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(
                            "webp".to_string(),
                        )))
                        .style(styl_przycisków(
                            false,
                            stan_klikaczy.webp_wybrany,
                            KOLOR_PEACH_PUFF,
                        ))
                        .width(Length::FillPortion(5)),
                    )
                    .spacing(0)
            })
            .push(ui_standard_oddzielacz())
            .padding(15)
            .spacing(15)
            .width(Length::FillPortion(2)),
    );

    Row::new().push(lewa_kolumna).push(prawa_kolumna).into()
}
