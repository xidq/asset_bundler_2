use iced::widget::{button, pick_list, progress_bar, space, text, text_input, Column, Row};
use iced::Alignment;

use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::hint_master::{hint_btn, hint_pick_list};
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::progress_bar::styl_progress_bar;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;
use enumy::dane_do_przetwarzania::{DaneDoDekompresjaPlików, DaneDoKompresjaPlików};
use enumy::enums_structs_io::{LogPakowanie, LogRozpakowywanie};
use enumy::fn_ogolne_przeliczeniowe::przelicz_bajty;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, UiPodstrony, UstawieniaThemeWsio};
use enumy::lang::odmiana_liczbowa;
use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::{Color, Element, Length};

pub fn view_binarka<'a>(
    dane_p: &'a DaneDoKompresjaPlików,
    dane_d: &'a DaneDoDekompresjaPlików,
    jezyk: &'a WybórJęzyka,
    proces_pakowanie: &'a LogPakowanie,
    proces_rozpakowanie: &'a LogRozpakowywanie,
    temat:&'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    // let przelicz_bajty = |xx:u64| -> String{
    //     let kb: u64 = 1024_u64 * 8;
    //     let mb: u64 = 1024_u64.pow(2) * 8;
    //     let gb: u64 = 1024_u64.pow(3) * 8;
    //     let tb: u64 = 1024_u64.pow(4) * 8;
    //     let ccvbfd = match xx{
    //         0..=8 => { format!("{}b",xx) }
    //         n if n < kb => { format!("{:.2}B",xx as f64/8.) }
    //         n if n < mb => { format!("{:.2}kB",xx as f64/kb as f64) }
    //         n if n < gb => { format!("{:.2}MB",xx as f64/mb as f64) }
    //         n if n < tb => { format!("{:.2}GB",xx as f64/gb as f64) }
    //         _ => { format!("{:.2}TB", xx as f64/tb as f64 ) }
    //     };
    //     ccvbfd
    // };

    // let hint_btn =|xx: Button<'a, Message>, hint:&'static str, var:bool| -> Element<'a, Message> {
    //     if var{
    //         tooltip(
    //             xx,
    //             hint,
    //             tooltip::Position::Bottom
    //         )
    //         .style(styl_hint())
    //         .into()
    //     }else{
    //         xx.into()
    //     }
    // };

    let tekst_logów = |x:String| -> Element<'a,Message>{
        text(x).color(KOLOR_CZCIONKI_SREDNI).font(jezyk.get_font()).size(12.).width(Length::FillPortion(4)).into()
    };
    let dane_p_poprawne =
        dane_p.ścieżka_in.exists() && dane_p.ścieżka_out.exists() && !dane_p.nazwa.is_empty();
    let pakowanie =
        Column::new()
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button(
                                folder_icon(
                                    true,
                                    if dane_p.ścieżka_in.to_string_lossy().is_empty() {
                                        0
                                    } else {
                                        2
                                    },
                                    &temat.kolory.binarka,
                                )
                            )
                                .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::InputPath))
                                .padding(10)
                                .style(styl_przycisków(
                                    false,
                                    false,
                                    &temat.kolory.binarka,temat,
                                )),
                            jezyk.t("hint_binary_pack_choose_in_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(jezyk.t("mgt_input_folder"), &dane_p.ścieżka_in.to_string_lossy())
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::InputPathText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_p.ścieżka_in.exists(), &temat.kolory.binarka,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),
            )

            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button(
                                folder_icon(
                                true,
                                if dane_p.ścieżka_out.to_string_lossy().is_empty() {
                                    0
                                } else {
                                    2
                                },
                                &temat.kolory.binarka,
                                )
                            )
                                .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::OutputPath))
                                .padding(10)
                                .style(styl_przycisków(
                                    false,
                                    false,
                                    &temat.kolory.binarka,temat,
                                )),
                            jezyk.t("hint_binary_pack_choose_out_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(
                            jezyk.t("mgt_output_folder"),
                            &dane_p.ścieżka_out.to_string_lossy(),
                        )
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::OutputPathText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_p.ścieżka_out.exists(), &temat.kolory.binarka,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),
            )
            // --- KOMPRESJA ---
            .push(
                Column::new()
                    .push(
                        text(jezyk.t("mgt_compression_level_label"))
                            .font(jezyk.get_font())
                            .color(Color::from_rgba(1., 1., 1., 0.8))
                            .size(14),
                    )
                    .push(
                        hint_pick_list(
                            pick_list(
                                OptKompresjaPlikówPoziomKompresjiZstd::WSIOKOMPRESJI,
                                Some(dane_p.kompresja),
                                |xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::Kompresja(xx)),
                            )
                                .width(Length::Fill)
                                .padding(10)
                                .style(styl_pick_list(&temat.kolory.binarka,temat))
                                .menu_style(styl_menu_pick(&temat.kolory.binarka, temat)),
                            jezyk.t("hint_binary_pack_choose_compression"),
                            temat,
                        )
                    )
                    .spacing(5),
            )
            // --- FILTROWANIE ---
            .push(
                Column::new()
                    .push(
                        text(jezyk.t("mgt_filter_label"))
                            .font(jezyk.get_font())
                            .color(Color::from_rgba(1., 1., 1., 0.8))
                            .size(14),
                    )
                    .push(
                        hint_pick_list(
                            pick_list(
                                OptKompresjaPlikówFiltracjaPlików::WSIOPLIKOW,
                                Some(dane_p.filtracja), // Pobiera aktualną wartość ze struktury
                                |xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::Filtr(xx)),
                            )
                                .width(Length::Fill)
                                .padding(10)
                                .style(styl_pick_list(&temat.kolory.binarka,temat))
                                .menu_style(styl_menu_pick(&temat.kolory.binarka,temat)),
                            jezyk.t("mgt_filter_label"),
                            temat,
                        )
                    )
                    .spacing(5),
            )
            // --- NAZWA PLIKU ---
            .push(
                text_input(jezyk.t("mgt_file_name"), &dane_p.nazwa)
                    .font(jezyk.get_font())
                    .on_input(|xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::Nazwa(xx)))
                    .padding(10)
                    .style(styl_text_input(!dane_p.nazwa.is_empty(), &temat.kolory.binarka,temat)),
            )
            // --- PRZYCISK START ---
            .push(
                if temat.temp.aktywny_proces == ActProces::Żodyn && dane_p_poprawne {
                    hint_btn(
                        button(
                            text(jezyk.t("mgt_btn_ready")) //menu general translation
                                .font(jezyk.get_font())
                                .color(Color::from_rgba(1., 1., 1., 0.8))
                                .width(Length::Fill)
                                .center(),
                            )
                            .padding(12)
                            .height(Length::Fixed(40.))
                            .width(Length::Fill)
                            .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom))
                            .style(styl_przycisków(false, true, &temat.kolory.binarka,temat,)),
                        jezyk.t("hint_binary_pack_process_btn"),
                        temat,
                    )
                    // button(
                    //     text(jezyk.t("mgt_btn_ready")) //menu general translation
                    //         .font(jezyk.get_font())
                    //         .color(Color::from_rgba(1., 1., 1., 0.8))
                    //         .width(Length::Fill)
                    //         .center(),
                    // )
                    //     .padding(12)
                    //     .height(Length::Fixed(40.))
                    //     .width(Length::Fill)
                    //     .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom))
                    //     .style(styl_przycisków(false, true, &temat.kolory.binarka))
                } else {
                    hint_btn(
                        button(
                            text(jezyk.t(if temat.temp.aktywny_proces == ActProces::PakowaniePliku {
                                "mgt_btn_busy_processing"
                            } else if temat.temp.aktywny_proces != ActProces::Żodyn {
                                "mgt_btn_busy_processing_other"
                            } else {
                                "mgt_btn_gib_data"
                            }))
                                .font(jezyk.get_font())
                                .color(Color::from_rgba(1., 1., 1., 0.7))
                                .width(Length::Fill)
                                .center(),
                        )
                            .padding(12)
                            .height(Length::Fixed(40.))
                            .width(Length::Fill)
                            .style(styl_przycisków(
                                temat.temp.aktywny_proces == ActProces::PakowaniePliku,
                                false,
                                if temat.temp.aktywny_proces == ActProces::PakowaniePliku {
                                    &temat.kolory.binarka
                                } else {
                                    &temat.obecny_theme.bground
                                }, temat,
                            )),
                        jezyk.t("hint_binary_pack_process_btn"),
                        temat,
                    )
                }
            )
            .push(if proces_pakowanie.błąd.is_empty() {
                Column::new()
                    .push(
                        tekst_logów(
                            if proces_pakowanie.zbieranie_plików > 0 {
                                format!("{}: {} {}",jezyk.t("proces_binary_pack_collecting_pending"),proces_pakowanie.zbieranie_plików, jezyk.t(&odmiana_liczbowa(proces_pakowanie.zbieranie_plików)))
                            } else {String::new()}
                        )
                    )
                    .push(
                        if let Some(wartość) = proces_pakowanie.pakowanie.1 {
                            Row::new()
                                .push(
                                    text(
                                        if proces_pakowanie.pakowanie.0 == wartość {
                                            format!("{}: {} z {} {}", jezyk.t("proces_binary_pack_packing_pending"), proces_pakowanie.pakowanie.0, wartość, odmiana_liczbowa(wartość))
                                        } else {
                                            format!("{} {} {}", jezyk.t("proces_binary_pack_packing_finished"), wartość, odmiana_liczbowa(wartość))
                                        }
                                    )
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI).width(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                                .push(
                                    progress_bar(
                                        0.0..=wartość as f32,
                                        proces_pakowanie.pakowanie.0 as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka,temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        } else {
                            Row::new()
                        }
                    )
                    .push(
                        if proces_pakowanie.kompresja.is_some() {
                            Row::new()
                                .push(
                                    text(
                                        if proces_pakowanie.kompresja.unwrap_or(0) > 100 {
                                            jezyk.t("proces_binary_pack_compression_pending")
                                        } else {
                                            jezyk.t("proces_binary_pack_compression_finished")
                                        }
                                    )
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI).width(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                                .push(
                                    progress_bar(
                                        0.0..=100.,
                                        proces_pakowanie.kompresja.unwrap_or(0) as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka,temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        } else {
                            Row::new()
                        }
                    )
                    .push(
                        if proces_pakowanie.szyfrowanie.is_some() {
                            Row::new()
                                .push(
                                    text(
                                        if proces_pakowanie.szyfrowanie.unwrap_or(0) > 100 {
                                            jezyk.t("proces_binary_pack_encoding_pending")
                                        } else {
                                            jezyk.t("proces_binary_pack_encoding_finished")
                                        }
                                    )
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI).width(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))

                                .push(
                                    progress_bar(
                                        0.0..=100.,
                                        proces_pakowanie.szyfrowanie.unwrap_or(0) as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka,temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        } else {
                            Row::new()
                        }
                    )
                    .push(
                        text(
                            if !proces_pakowanie.koniec.is_empty() {
                                format!("{}: {}", jezyk.t("proces_binary_pack_end"), proces_pakowanie.koniec)
                            } else {
                                "".to_string()
                            }
                        )
                            .font(jezyk.get_font())
                            .color(KOLOR_CZCIONKI_SREDNI),
                    )
            } else {
                Column::new().push(
                    text(
                        format!("{}: {}", jezyk.t("mgt_proces_error"), proces_pakowanie.błąd)
                    )
                        .font(jezyk.get_font())
                        .color(KOLOR_CZCIONKI_SREDNI),
                )
            }
            )
            .spacing(15)
            .width(Length::FillPortion(2));

    let dane_d_poprawne = dane_d.ścieżka_pliku.exists() && dane_d.ścieżka_docelowa.exists();
    
    let rozpakowanie =
        Column::new()
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button("📄")
                                .on_press(Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::InputFile))
                                .padding(10)
                                .style(styl_przycisków(false, false, &temat.kolory.binarka,temat)),
                            jezyk.t("hint_binary_unpack_choose_in_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(jezyk.t("mgt_input_file"), &dane_d.ścieżka_pliku.to_string_lossy())
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::InputFileText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_d.ścieżka_pliku.exists(),&temat.kolory.binarka,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),


            )
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button(folder_icon(
                                true,
                                if dane_d.ścieżka_docelowa.to_string_lossy().is_empty() {
                                    0
                                } else {
                                    2
                                },
                                &temat.kolory.binarka,
                            ))
                                .on_press(Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::OutputPath))
                                .padding(10)
                                .style(styl_przycisków(false, false, &temat.kolory.binarka,temat)),
                            jezyk.t("hint_binary_unpack_choose_out_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(
                            jezyk.t("mgt_input_folder"),
                            &dane_d.ścieżka_docelowa.to_string_lossy(),
                        )
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::OutputPathText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_d.ścieżka_docelowa.exists(),&temat.kolory.binarka,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),
            )
            .push(if temat.temp.aktywny_proces == ActProces::Żodyn && dane_d_poprawne {
                hint_btn(
                button(
                    text(jezyk.t("mgt_btn_ready"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .width(Length::Fill)
                        .center(),
                )
                    .padding(12)
                    .height(Length::Fixed(40.))
                    .width(Length::Fill)
                    .on_press(Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::Uruchom))
                    .style(styl_przycisków(false, true, &temat.kolory.binarka,temat)),
                jezyk.t("hint_binary_unpack_process_btn"),
                temat,
                )
            } else {

                hint_btn(
                button(
                    text(jezyk.t(if temat.temp.aktywny_proces == ActProces::RozpakowaniePliku {
                        "mgt_btn_bussy_processing"
                    } else if temat.temp.aktywny_proces != ActProces::Żodyn {
                        "mgt_btn_bussy_processing_other"

                    } else {
                        "mgt_btn_gib_data"
                    }))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.7))
                        .width(Length::Fill)
                        .center(),
                )
                    .padding(12)
                    .height(Length::Fixed(40.))
                    .width(Length::Fill)
                    .style(styl_przycisków(
                        temat.temp.aktywny_proces == ActProces::RozpakowaniePliku,
                        false,
                        if temat.temp.aktywny_proces == ActProces::RozpakowaniePliku {
                            &temat.kolory.binarka
                        } else {
                            &temat.obecny_theme.bground
                        },
                        temat,
                    )
                    )
                    ,
                jezyk.t("hint_binary_unpack_process_btn"),
                temat,
                )
            }
            )
            .push(if proces_rozpakowanie.błąd.is_empty() {
                Column::new()
                    .push(
                        if let Some(wartość) = proces_rozpakowanie.kontrola_pliku.1{
                            Row::new()
                                .push(tekst_logów(
                                    if wartość > proces_rozpakowanie.kontrola_pliku.0 {
                                        format!("{}: ",jezyk.t("proces_binary_unpack_files_pending"))
                                    }else{
                                        format!("{} {} {}",jezyk.t("proces_binary_unpack_files"),wartość, jezyk.t(&odmiana_liczbowa(wartość)) )
                                    }
                                ))
                                .push(space().width(Length::FillPortion(1)))

                                .push(
                                    progress_bar(
                                        0.0..=wartość as f32,
                                        proces_rozpakowanie.kontrola_pliku.0 as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka,temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        }else{
                            Row::new()
                        }
                    )
                    .push(
                        if let Some(wartość) = proces_rozpakowanie.deszyfrowanie.1{
                            Row::new()
                                .push(tekst_logów(
                                    if wartość > proces_rozpakowanie.deszyfrowanie.0 {
                                        format!("{}: ",jezyk.t("proces_binary_unpack_decoding_pending"))
                                    }else{
                                        format!("{} {} {}",jezyk.t("proces_binary_unpack_decoding"),wartość, jezyk.t(&odmiana_liczbowa(wartość)) )
                                    }
                                ))
                                .push(space().width(Length::FillPortion(1)))

                                .push(
                                    progress_bar(
                                        0.0..=wartość as f32,
                                        proces_rozpakowanie.deszyfrowanie.0 as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka, temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        }else{
                            Row::new()
                        }
                    )

                    .push(
                        if proces_rozpakowanie.dekompresja > 0 {
                            tekst_logów(format!("{}: {}", jezyk.t("proces_binary_unpack_decompression"), przelicz_bajty(proces_rozpakowanie.dekompresja)))
                        }else{
                            tekst_logów("".to_string())
                        }

                    )
                    .push(
                        if let Some(ggg) = proces_rozpakowanie.rozpakowanie.1{
                            Row::new()
                                .push(tekst_logów(
                                    if ggg > proces_rozpakowanie.rozpakowanie.0 {
                                        format!("{}: ",jezyk.t("proces_binary_unpack_unpacking_pending"))
                                    }else{
                                        format!("{} {} {}",jezyk.t("proces_binary_unpack_unpacking"),proces_rozpakowanie.rozpakowanie.1.unwrap(), jezyk.t(&odmiana_liczbowa(proces_rozpakowanie.rozpakowanie.1.unwrap())) )
                                    }
                                ))
                                .push(space().width(Length::FillPortion(1)))
                                .push(
                                    progress_bar(
                                        0.0..=ggg as f32,
                                        proces_rozpakowanie.rozpakowanie.0 as f32
                                    ).style(styl_progress_bar(&temat.kolory.binarka, temat)).girth(12.).length(Length::FillPortion(4))
                                )
                                .push(space().width(Length::FillPortion(1)))
                        }else{
                            Row::new()
                        }
                    )
                    .push(
                        tekst_logów(
                            if !proces_rozpakowanie.czas.is_empty() {
                                format!("{}: {}", jezyk.t("proces_binary_unpack_end"), proces_rozpakowanie.czas)
                            }else{String::new()}
                        )
                    )
            } else {
                Column::new().push(text(format!("{}: {}",jezyk.t("mgt_proces_error"),proces_rozpakowanie.błąd)).font(jezyk.get_font()).color(Color::from_rgba(1.,0.5,0.5,0.8)))
            })
            .spacing(15)
            .width(Length::FillPortion(2));



    let lewa = Column::new()
        .push(
            hint_btn(
                button(
                    text(jezyk.t("ui_menu_bin_pack"))
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::Fill)
                        .center(),
                )
                    .width(Length::Fill)
                    .on_press(Message::ZmienWariant(UiPodstrony::BinPakowanie))
                    .style(styl_przycisków(
                        temat.temp.aktywny_proces == ActProces::PakowaniePliku,
                        matches!(temat.temp.aktywne_okno, UiPodstrony::BinPakowanie),
                        &temat.kolory.binarka, temat,
                    )),
                jezyk.t("ui_menu_bin_pack"),
                temat,
            )
        )
        .push(
            hint_btn(
            button(
                text(jezyk.t("ui_menu_bin_unpack"))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
                .width(Length::Fill)
                .on_press(Message::ZmienWariant(UiPodstrony::BinRozpakowanie))
                .style(styl_przycisków(
                    temat.temp.aktywny_proces == ActProces::RozpakowaniePliku,
                    matches!(temat.temp.aktywne_okno, UiPodstrony::BinRozpakowanie),
                    &temat.kolory.binarka, temat,
                )),
            jezyk.t("ui_menu_bin_unpack"),
            temat,
            )
        )
        // .push(space().height(Length::FillPortion(10)))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));
    
    
    
    Row::new()
        .push(lewa)
        .push(
        match temat.temp.aktywne_okno{
                UiPodstrony::BinPakowanie => {pakowanie}
                UiPodstrony::BinRozpakowanie => {rozpakowanie}
                _ => {Column::new()}
            }
        ).into()
}
