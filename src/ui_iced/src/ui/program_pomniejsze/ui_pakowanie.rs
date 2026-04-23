use iced::Alignment;
use iced::widget::{Column, Row, button, pick_list, text, text_input};

use crate::ui::program_pomniejsze::kolory::{KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use enumy::dane_do_przetwarzania::DaneDoKompresjaPlików;
use enumy::enums_structs_io::LogPakowanie;
use enumy::ikony::folder_icon;
use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::{Color, Element, Length};
use crate::ui::wiadomosci::message_ui::Message;

pub fn view_eksport(
    dane: &DaneDoKompresjaPlików,
    jezyk: WybórJęzyka,
    dane_procesu: LogPakowanie,
    is_loading: bool,
    main_process_check: bool,
) -> Element<'_, Message> {
    // 1. Tworzymy listę przetłumaczonych opcji (Stringi)
    let opcje_kompresja: Vec<String> = OptKompresjaPlikówPoziomKompresjiZstd::WSIOKOMPRESJI
        .iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    // 2. Pobieramy aktualnie wybraną etykietę
    let wybrana_etykieta_kompresja = jezyk.t(dane.kompresja.klucz()).to_string();

    let opcje_filter: Vec<String> = OptKompresjaPlikówFiltracjaPlików::WSIOPLIKOW
        .iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    // 2. Pobieramy aktualnie wybraną etykietę
    let wybrana_etykieta_filter = jezyk.t(dane.filtracja.klucz()).to_string();

    let dane_poprawne =
        dane.ścieżka_in.exists() && dane.ścieżka_out.exists() && !dane.nazwa.is_empty();
    Column::new()
        // .push(text(jezyk.t("packing_config_menu")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(22),)
        // --- FOLDER WEJŚCIOWY ---
        .push(
            Row::new()
                .push(
                    button(folder_icon(
                        true,
                        if dane.ścieżka_in.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_BRILIANT_CRIMSON,
                    ))
                    .on_press(Message::WybierzFolderInPakowanie)
                    .padding(10)
                    .style(styl_przycisków(
                        false,
                        false,
                        KOLOR_BRILIANT_CRIMSON,
                    )),
                )
                .push(
                    text_input(jezyk.t("input_folder"), &dane.ścieżka_in.to_string_lossy())
                        .font(jezyk.get_font())
                        .on_input(Message::PakowaniePathChanged)
                        .padding(10)
                        .style(styl_text_input(dane.ścieżka_in.exists(),KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA)),
                )
                .spacing(10)
                .align_y(Alignment::Center),
        )
        // --- FOLDER WYJŚCIOWY ---
        .push(
            Row::new()
                .push(
                    button(folder_icon(
                        true,
                        if dane.ścieżka_out.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_BRILIANT_CRIMSON,
                    ))
                    .on_press(Message::WybierzFolderOutPakowanie)
                    .padding(10)
                    .style(styl_przycisków(
                        false,
                        false,
                        KOLOR_BRILIANT_CRIMSON,
                    )),
                )
                .push(
                    text_input(
                        jezyk.t("output_folder"),
                        &dane.ścieżka_out.to_string_lossy(),
                    )
                    .font(jezyk.get_font())
                    .on_input(Message::PakowanieOutPathChanged)
                    .padding(10)
                    .style(styl_text_input(dane.ścieżka_out.exists(),KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA)),
                )
                .spacing(10)
                .align_y(Alignment::Center),
        )
        // --- KOMPRESJA ---
        .push(
            Column::new()
                .push(
                    text(jezyk.t("compression_level_label"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .size(14),
                )
                .push(
                    pick_list(
                        opcje_kompresja,
                        Some(wybrana_etykieta_kompresja),
                        Message::OptKompresjaPlikówPoziomKompresjiZstdChanged,
                    )
                    .width(Length::Fill)
                    .padding(10)
                    .style(styl_pick_list(KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA))
                    .menu_style(styl_menu_pick(KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA)),
                )
                .spacing(5),
        )
        // --- FILTROWANIE ---
        .push(
            Column::new()
                .push(
                    text(jezyk.t("file_filter_label"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .size(14),
                )
                .push(
                    pick_list(
                        opcje_filter,
                        Some(wybrana_etykieta_filter), // Pobiera aktualną wartość ze struktury
                        Message::FilterChanged,
                    )
                    .width(Length::Fill)
                    .padding(10)
                    .style(styl_pick_list(KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA))
                    .menu_style(styl_menu_pick(KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA)),
                )
                .spacing(5),
        )
        // --- NAZWA PLIKU ---
        .push(
            text_input(jezyk.t("output_file_name"), &dane.nazwa)
                .font(jezyk.get_font())
                .on_input(Message::NazwaPaczkiChanged)
                .padding(10)
                .style(styl_text_input(!dane.nazwa.is_empty(),KOLOR_BRILIANT_CRIMSON, KOLOR_TŁA)),
        )
        // --- PRZYCISK START ---
        .push(if !is_loading && !main_process_check && dane_poprawne {
            button(
                text(jezyk.t("packing_btn_start"))
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.8))
                    .width(Length::Fill)
                    .center(),
            )
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .on_press(Message::UruchomProcesPakowania)
            .style(styl_przycisków(false, true, KOLOR_BRILIANT_CRIMSON))
        } else {
            // Wariant 3: inny proces trwa
            button(
                text(jezyk.t(if main_process_check {
                    "btn_bussy_processing_other"
                } else if is_loading {
                    "btn_bussy_processing"
                } else {
                    "btn_gib_data"
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
                is_loading,
                false,
                if is_loading {
                    KOLOR_BRILIANT_CRIMSON
                } else {
                    (0.2, 0.2, 0.2)
                },
            ))
        })
        // .push(if is_loading {
        //             // Wariant 1: Proces trwa -> Przycisk zablokowany (napis "Przetwarzanie...")
        //             button(text(jezyk.t("processing")).font(jezyk.get_font()).width(Length::Fill).center())
        //                 .padding(12)
        //                 .width(Length::Fill)
        //                 .style(styl_przycisków(false,true,KOLOR_BRILIANT_CRIMSON))
        //         } else if main_process_check {
        //             // Wariant 2: innyprocestrwa
        //             button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
        //                 // .on_press(Message::UruchomProcesPakowania)
        //                 .padding(12)
        //                 .width(Length::Fill)
        //                 .style(styl_przycisków(false,true,KOLOR_BRILIANT_CRIMSON))
        //         } else if dane_poprawne {
        //             // Wariant 2: Gotowy do startu -> Przycisk aktywny
        //             button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
        //                 .on_press(Message::UruchomProcesPakowania)
        //                 .padding(12)
        //                 .width(Length::Fill)
        //                 .style(styl_przycisków(false,false,KOLOR_BRILIANT_CRIMSON))
        //         } else {
        //             // Wariant 3: Braki w danych -> Przycisk zablokowany (napis "Start")
        //             button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
        //                 .padding(12)
        //                 .width(Length::Fill)
        //                 .style(styl_przycisków(false,true,(0.2,0.2,0.2)))
        //         },)
        .push(if dane_procesu.błąd.is_empty() {
            Column::new()
                .push(
                    text(dane_procesu.zbieranie_plików)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.pakowanie)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.kompresja)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.szyfrowanie)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.czas)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
        } else {
            Column::new().push(
                text(dane_procesu.błąd)
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 0.5, 0.5, 0.8)),
            )
        })
        .spacing(15)
        .max_width(800) // Opcjonalnie, żeby UI nie rozjechało się na całą szerokość
        .into()
}
