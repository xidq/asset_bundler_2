use crate::io::enums_structs_io::DaneDoDekompresji;
// use crate::io::enums_structs_io::DaneDoDekompresji;
use crate::ui::program::{LogRozpakowywanie, Message, WybórJęzyka};
use crate::{styl_przycisków, styl_text_input};
use iced::widget::{button, column, row, text, text_input, Column, Row};
use iced::{Alignment, Element, Length};

use crate::ui::program::czcionki::{KOLORFLIRT, KOLORTŁA};
use iced::{Border, Color, };
// use iced_core::Background::Gradient;
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians};

pub fn view_import(
    dane: &DaneDoDekompresji,
    jezyk: WybórJęzyka,
    dane_procesu: LogRozpakowywanie,
    is_loading: bool,
    main_process_check:bool,
) -> Element<'_, Message> {

    let dane_poprawne = dane.ścieżka_pliku.exists()
        && dane.ścieżka_docelowa.exists();
    Column::new()
        .push(text("Konfiguracja Rozpakowywania").font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(22),)
        // text_input(
        //     "Plik .jrzs...",
        //     &dane.ścieżka_pliku.to_string_lossy()
        // ), // Tutaj będziesz potrzebował Message::ImportPathChanged
        // text(jezyk.t("packing_config_menu")).font(jezyk.get_font()).size(22),

        // --- FOLDER WEJŚCIOWY ---
        .push(Row::new()
            .push(button("📄").on_press(Message::WybierzPlikExPakowanie).padding(10).style(styl_przycisków!(false,false,KOLORFLIRT)),)
            .push(text_input(
                    jezyk.t("input_file"),
                    &dane.ścieżka_pliku.to_string_lossy()
                ).font(jezyk.get_font())
                .on_input(Message::DekompresjaPlikPathChanged)
                .padding(10)
                .style(styl_text_input!(KOLORFLIRT,KOLORTŁA)),
            )
        .spacing(10)
        .align_y(Alignment::Center),)

        .push(Row::new()
            .push(button("📂").on_press(Message::WybierzFolderOutExPakowanie).padding(10).style(styl_przycisków!(false,false,KOLORFLIRT)),)
            .push(text_input(
                    jezyk.t("input_folder"),
                    &dane.ścieżka_docelowa.to_string_lossy()
                ).font(jezyk.get_font())
                .on_input(Message::PakowaniePathChanged)
                .padding(10)
                .style(styl_text_input!(KOLORFLIRT,KOLORTŁA)),
            )
        .spacing(10)
        .align_y(Alignment::Center),)

        .push(if is_loading {
                // Wariant 1: Proces trwa -> Przycisk zablokowany (napis "Przetwarzanie...")
                button(text(jezyk.t("un_processing")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).width(Length::Fill).center())
                    .padding(12)
                    .width(Length::Fill)
                    .style(styl_przycisków!(false,true,KOLORFLIRT))
            } else if main_process_check{
                // Wariant 3: inny proces trwa
                button(text(jezyk.t("un_packing_start")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).width(Length::Fill).center())
                    .padding(12)
                    .width(Length::Fill)
                    .style(styl_przycisków!(false,true,(0.2,0.2,0.2)))
            } else if dane_poprawne {
                // Wariant 2: Gotowy do startu -> Przycisk aktywny
                button(text(jezyk.t("un_packing_start")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).width(Length::Fill).center())
                    .on_press(Message::ResetLogRozpakowania)
                    .on_press(Message::UruchomProcesRozpakowania)
                    .padding(12)
                    .width(Length::Fill)
                    .style(styl_przycisków!(false,false,KOLORFLIRT))

            } else {
                // Wariant 3: Braki w danych -> Przycisk zablokowany (napis "Start")
                button(text(jezyk.t("un_packing_start")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).width(Length::Fill).center())
                    .padding(12)
                    .width(Length::Fill)
                    .style(styl_przycisków!(false,true,(0.2,0.2,0.2)))
        },)
        .push(if dane_procesu.błąd.is_empty(){
            Column::new()
                .push(text(dane_procesu.kontrola_pliku).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.deszyfrowanie).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.dekompresja).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.rozpakowanie).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.czas).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)

        }else{
            Column::new()
                .push(text(dane_procesu.błąd).font(jezyk.get_font()),)

        },)



        .spacing(15)
        .max_width(800) // Opcjonalnie, żeby UI nie rozjechało się na całą szerokość
        .into()
}