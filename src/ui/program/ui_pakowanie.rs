use iced::widget::{button, column, pick_list, row, text, text_input, Column, Row};
use iced::Alignment;
use std::fmt;

use crate::io::enums_structs_io::{DaneDoKompresji, FiltracjaPlików, PoziomKompresji};
use crate::ui::program::czcionki::{KOLORBRILIANTCRIMSON, KOLORTŁA};
use crate::ui::program::{LogPakowanie, Message, WybórJęzyka};
use crate::{styl_menu_pick, styl_pick_list, styl_przycisków, styl_text_input};
use iced::{Border, Color, Element, Length, };
// use iced_core::Background::Gradient;
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians, Shadow, Vector};
// use crate::ui::program::Message::DevZmienJezyk;
// use crate::ui::program::WybórJęzyka::HU;
#[allow(dead_code)]
impl FiltracjaPlików {
    pub const ALL: [Self; 5] = [
        Self::Wszystkie, Self::Graficzne, Self::Audio, Self::Tekstowe, Self::Pdf
    ];
}

impl fmt::Display for FiltracjaPlików {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Możesz tu użyć metody .t() jeśli chcesz tłumaczyć nazwy filtrów

        write!(f, "{:?}", self)
        // f.write_str(match self {
        //     Self::Wszystkie => "Apple",
        //     Self::Graficzne => "Orange",
        //     Self::Audio => "Strawberry",
        //     Self::Tekstowe => "Tomato",
        //     Self::Pdf => "PDF",
        // })
    }
}

impl fmt::Display for PoziomKompresji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



pub fn view_eksport(
    dane: &DaneDoKompresji,
    jezyk: WybórJęzyka,
    dane_procesu: LogPakowanie,
    is_loading: bool,
    main_process_check:bool,
) -> Element<'_, Message> {
    // 1. Tworzymy listę przetłumaczonych opcji (Stringi)
    let opcje_kompresja: Vec<String> = PoziomKompresji::WSIOKOMPRESJI.iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    // 2. Pobieramy aktualnie wybraną etykietę
    let wybrana_etykieta_kompresja = jezyk.t(dane.kompresja.klucz()).to_string();

    let opcje_filter: Vec<String> = FiltracjaPlików::WSIOPLIKOW.iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    // 2. Pobieramy aktualnie wybraną etykietę
    let wybrana_etykieta_filter = jezyk.t(dane.filtracja.klucz()).to_string();





    let dane_poprawne = dane.ścieżka_in.exists()
        && dane.ścieżka_out.exists()
        && !dane.nazwa.is_empty();
    Column::new()
        .push(text(jezyk.t("packing_config_menu")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(22),)

        // --- FOLDER WEJŚCIOWY ---
        .push(Row::new()
            .push(button("📂").on_press(Message::WybierzFolderInPakowanie).padding(10).style(styl_przycisków!(false,false,KOLORBRILIANTCRIMSON)),)
            .push(text_input(
                jezyk.t("input_folder"),
                &dane.ścieżka_in.to_string_lossy()
            ).font(jezyk.get_font())
            .on_input(Message::PakowaniePathChanged)
            .padding(10)
            .style(styl_text_input!(KOLORBRILIANTCRIMSON,KOLORTŁA)),
            )
        .spacing(10)
        .align_y(Alignment::Center),)

        // --- FOLDER WYJŚCIOWY ---
        .push(Row::new()
            .push(button("📂").on_press(Message::WybierzFolderOutPakowanie).padding(10).style(styl_przycisków!(false,false,KOLORBRILIANTCRIMSON)),)
            .push(
                text_input(
                    jezyk.t("output_folder"),
                    &dane.ścieżka_out.to_string_lossy()
                )
                    .font(jezyk.get_font())
                    .on_input(Message::PakowanieOutPathChanged)
                    .padding(10)
                    .style(styl_text_input!(KOLORBRILIANTCRIMSON,KOLORTŁA)),
            )
        .spacing(10)
        .align_y(Alignment::Center),)

// --- KOMPRESJA ---
        .push(Column::new()
            .push(
                text(jezyk.t("compression_level_label")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(14),
            )
            .push(
                pick_list(
                    opcje_kompresja,
                    Some(wybrana_etykieta_kompresja),
                    Message::PoziomKompresjiChanged,
                )
                    .width(Length::Fill)
                    .padding(10)
                    .style(styl_pick_list!(KOLORBRILIANTCRIMSON, KOLORTŁA))
                    .menu_style(styl_menu_pick!(KOLORBRILIANTCRIMSON, KOLORTŁA)),
            )
        .spacing(5),)

        // --- FILTROWANIE ---
        .push(Column::new()
            .push(text(jezyk.t("file_filter_label")).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)).size(14),)
            .push(
                pick_list(
                    opcje_filter,
                    Some(wybrana_etykieta_filter), // Pobiera aktualną wartość ze struktury
                    Message::FilterChanged,
                )
                    .width(Length::Fill)
                    .padding(10)
                    .style(styl_pick_list!(KOLORBRILIANTCRIMSON, KOLORTŁA))
                    .menu_style(styl_menu_pick!(KOLORBRILIANTCRIMSON, KOLORTŁA)),
            )
        .spacing(5),)
        // --- NAZWA PLIKU ---
        .push(text_input(
            jezyk.t("output_file_name"),
            &dane.nazwa
        )
            .font(jezyk.get_font())
            .on_input(Message::NazwaPaczkiChanged)
            .padding(10)
            .style(styl_text_input!(KOLORBRILIANTCRIMSON,KOLORTŁA)),)

        // --- PRZYCISK START ---
        .push(if is_loading {
                    // Wariant 1: Proces trwa -> Przycisk zablokowany (napis "Przetwarzanie...")
                    button(text(jezyk.t("processing")).font(jezyk.get_font()).width(Length::Fill).center())
                        .padding(12)
                        .width(Length::Fill)
                        .style(styl_przycisków!(false,true,KOLORBRILIANTCRIMSON))
                } else if main_process_check {
                    // Wariant 2: innyprocestrwa
                    button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
                        // .on_press(Message::UruchomProcesPakowania)
                        .padding(12)
                        .width(Length::Fill)
                        .style(styl_przycisków!(false,true,KOLORBRILIANTCRIMSON))
                } else if dane_poprawne {
                    // Wariant 2: Gotowy do startu -> Przycisk aktywny
                    button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
                        .on_press(Message::UruchomProcesPakowania)
                        .padding(12)
                        .width(Length::Fill)
                        .style(styl_przycisków!(false,false,KOLORBRILIANTCRIMSON))
                } else {
                    // Wariant 3: Braki w danych -> Przycisk zablokowany (napis "Start")
                    button(text(jezyk.t("packing_start")).font(jezyk.get_font()).width(Length::Fill).center())
                        .padding(12)
                        .width(Length::Fill)
                        .style(styl_przycisków!(false,true,(0.2,0.2,0.2)))
                },)


        .push(if dane_procesu.błąd.is_empty(){
            Column::new()
                .push(text(dane_procesu.zbieranie_plików).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.pakowanie).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.kompresja).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.szyfrowanie).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
                .push(text(dane_procesu.czas).font(jezyk.get_font()).color(Color::from_rgba(1.,1.,1.,0.8)),)
        }else{
            Column::new()
                .push(text(dane_procesu.błąd).font(jezyk.get_font()).color(Color::from_rgba(1.,0.5,0.5,0.8)),
                )

        },)


    
        .spacing(15)
        .max_width(800) // Opcjonalnie, żeby UI nie rozjechało się na całą szerokość
        .into()
}