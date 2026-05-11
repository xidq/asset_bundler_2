use iced::widget::{button, progress_bar, space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::enums_structs_io::LogPrzetwarzanieFot;
use enumy::inne_ui::{ActProces, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_avif::podmenu_avif_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_ff::podmenu_ff_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_jpg::podmenu_jpg_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_lewe::{podmenu_lewe_rozdzielczosci, podmenu_lewe_wybor};
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_png::podmenu_png_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_qoi::podmenu_qoi_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_tga::podmenu_tga_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_webp::podmenu_webp_misc;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::progress_bar::styl_progress_bar;
use crate::ui::podmenu::ui_standard::run_btn::start_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn lewa_kolumna<'a>(dane: &'a DaneDoBathKonwersjaZdjec, log: &'a LogPrzetwarzanieFot, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a,Message>{

    Column::new()
        // Przycisk Ścieżki
        .push(podmenu_lewe_wybor(jezyk,&temat.kolory.konwersja, temat))
        .push(space().height(Length::Fixed(50.)))
        .push(podmenu_jpg_misc(dane, temat))
        .push(podmenu_avif_misc(dane, temat))
        .push(podmenu_png_misc(dane, temat))
        .push(podmenu_webp_misc(dane, temat))
        .push(podmenu_tga_misc(temat))
        .push(podmenu_ff_misc(dane, temat))
        .push(podmenu_qoi_misc(dane, temat))
        .push(podmenu_lewe_rozdzielczosci(temat))
        .push(space().height(Length::Fixed(10.)))
        .push(
            start_btn(
                ActProces::KonwersjaZdjęć, 
                temat.temp.start_btn_status.konwersja,
                "hint_conversion_process_btn",
                "hint_conversion_process_btn_pending", 
                &temat.kolory.konwersja,
                jezyk,
                temat 
            )
        )
        .push(text(&log.plik_początek))
        .push(text(&log.msg_walidacja))
        .push(if log.plik_procent != 0 {
            Row::new()
                .push(text(jezyk.t("proces_conversion_proces_pending")).font(jezyk.get_font()))
                .push(
                    progress_bar(0.0..=100., log.plik_procent as f32)
                        .girth(18.)
                        .style(styl_progress_bar(&temat.kolory.konwersja,temat)),
                )
        } else {
            Row::new()
        })
        .push(text(&log.msg_end).font(jezyk.get_font()))
        .push(text(&log.błąd).font(jezyk.get_font()))
        // .spacing(15)
        .width(Length::FillPortion(1))
}