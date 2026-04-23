// use crate::io::enums_structs_io::DaneDoDekompresjaPlików;
use iced::widget::{button, text, text_input, Column, Row};
use iced::{Alignment, Element, Length};

use crate::ui::program_pomniejsze::kolory::{KOLOR_FLIRT, KOLOR_TŁA};
// use iced_core::Background::Gradient;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use enumy::dane_do_przetwarzania::DaneDoDekompresjaPlików;
use enumy::enums_structs_io::LogRozpakowywanie;
use enumy::ikony::folder_icon;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::{Border, Color};
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians};
use crate::ui::wiadomosci::message_ui::Message;

pub fn view_import(
    dane: &DaneDoDekompresjaPlików,
    jezyk: WybórJęzyka,
    dane_procesu: LogRozpakowywanie,
    is_loading: bool,
    main_process_check: bool,
) -> Element<'_, Message> {
    let dane_poprawne = dane.ścieżka_pliku.exists() && dane.ścieżka_docelowa.exists();
    Column::new()
        // .push(text("Konfiguracja Rozpakowywania").font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(22),)
        // text_input(
        //     "Plik .jrzs...",
        //     &dane.ścieżka_pliku.to_string_lossy()
        // ), // Tutaj będziesz potrzebował Message::ImportPathChanged
        // text(jezyk.t("packing_config_menu")).font(jezyk.get_font()).size(22),
        // --- FOLDER WEJŚCIOWY ---
        .push(
            Row::new()
                .push(
                    button("📄")
                        .on_press(Message::WybierzPlikExPakowanie)
                        .padding(10)
                        .style(styl_przycisków(false, false, KOLOR_FLIRT)),
                )
                .push(
                    text_input(jezyk.t("input_file"), &dane.ścieżka_pliku.to_string_lossy())
                        .font(jezyk.get_font())
                        .on_input(Message::StatusDekompresjaPlikówDekompresjaPlikPathChanged)
                        .padding(10)
                        .style(styl_text_input(dane.ścieżka_pliku.exists(),KOLOR_FLIRT, KOLOR_TŁA)),
                )
                .spacing(10)
                .align_y(Alignment::Center),
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(
                        true,
                        if dane.ścieżka_docelowa.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_FLIRT,
                    ))
                    .on_press(Message::WybierzFolderOutExPakowanie)
                    .padding(10)
                    .style(styl_przycisków(false, false, KOLOR_FLIRT)),
                )
                .push(
                    text_input(
                        jezyk.t("input_folder"),
                        &dane.ścieżka_docelowa.to_string_lossy(),
                    )
                    .font(jezyk.get_font())
                    .on_input(Message::PakowaniePathChanged)
                    .padding(10)
                    .style(styl_text_input(dane.ścieżka_docelowa.exists(),KOLOR_FLIRT, KOLOR_TŁA)),
                )
                .spacing(10)
                .align_y(Alignment::Center),
        )
        .push(if !is_loading && !main_process_check && dane_poprawne {
            button(
                text(jezyk.t("un_packing_btn_start"))
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.8))
                    .width(Length::Fill)
                    .center(),
            )
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .on_press(Message::UruchomProcesRozpakowania)
            .style(styl_przycisków(false, true, KOLOR_FLIRT))
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
                    KOLOR_FLIRT
                } else {
                    (0.2, 0.2, 0.2)
                },
            ))
        })
        .push(if dane_procesu.błąd.is_empty() {
            Column::new()
                .push(
                    text(dane_procesu.kontrola_pliku)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.deszyfrowanie)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.StatusDekompresjaPlikówDekompresja)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.rozpakowanie)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
                .push(
                    text(dane_procesu.czas)
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8)),
                )
        } else {
            Column::new().push(text(dane_procesu.błąd).font(jezyk.get_font()))
        })
        .spacing(15)
        .max_width(800) // Opcjonalnie, żeby UI nie rozjechało się na całą szerokość
        .into()
}
