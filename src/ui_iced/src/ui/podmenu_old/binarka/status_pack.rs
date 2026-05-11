use iced::Element;
use iced::widget::{text, Column};
use iced_core::{Color, Length};
use enumy::enums_structs_io::{LogPakowanie, LogRozpakowywanie};
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::lang::odmiana_liczbowa;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::ui_standard::statusy::status_progress_bar_pliki;
use crate::ui::wiadomosci::message_ui::Message;

pub fn status_pakowanie<'a>(dane: &'a LogPakowanie,jezyk: &'a WybórJęzyka,temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{
    let tekst_logów = |x:String| -> Element<'a,Message>{
        text(x).color(KOLOR_CZCIONKI_SREDNI).font(jezyk.get_font()).size(12.).width(Length::FillPortion(4)).into()
    };
    if dane.błąd.is_empty() {
        Column::new()
            .push(
                tekst_logów(
                    if dane.zbieranie_plików > 0 {
                        format!("{}: {} {}",jezyk.t("proces_binary_pack_collecting_pending"),dane.zbieranie_plików, jezyk.t(&odmiana_liczbowa(dane.zbieranie_plików)))
                    } else {String::new()}
                )
            )
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.pakowanie), "proces_binary_pack_packing", jezyk, &temat.kolory.binarka, temat))
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.kompresja), "proces_binary_pack_compression", jezyk, &temat.kolory.binarka, temat))
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.szyfrowanie), "proces_binary_pack_compression", jezyk, &temat.kolory.binarka, temat))
            .push(
                    tekst_logów(
                    if !dane.koniec.is_empty() {
                        format!("{}: {}", jezyk.t("proces_binary_pack_end"), dane.koniec)
                    } else {
                        "".to_string()
                    }
                )
            ).into()
    } else {


        text(
            format!("{}: {}", jezyk.t("mgt_proces_error"), dane.błąd)
        )
            .font(jezyk.get_font())
            .color(temat.obecny_theme.err_font)
        .into()
    }
    
}
pub fn status_rozpakowanie<'a>(dane: &'a LogRozpakowywanie,jezyk: &'a WybórJęzyka,temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{
    let tekst_logów = |x:String| -> Element<'a,Message>{
        text(x).color(KOLOR_CZCIONKI_SREDNI).font(jezyk.get_font()).size(12.).width(Length::FillPortion(4)).into()
    };
    if dane.błąd.is_empty() {
        Column::new()
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.kontrola_pliku), "proces_binary_unpack_files", jezyk, &temat.kolory.binarka, temat))
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.deszyfrowanie), "proces_binary_unpack_decoding", jezyk, &temat.kolory.binarka, temat))

            .push(
                if dane.dekompresja > 0 {
                    tekst_logów(format!("{}: {}", jezyk.t("proces_binary_unpack_decompression"), jezyk.bajt(dane.dekompresja)))
                }else{
                    tekst_logów("".to_string())
                }

            )
            .push(status_progress_bar_pliki(jezyk.normal_u32_option(dane.rozpakowanie), "proces_binary_unpack_unpacking", jezyk, &temat.kolory.binarka, temat))

            .push(
                tekst_logów(
                    if !dane.czas.is_empty() {
                        format!("{}: {}", jezyk.t("proces_binary_unpack_end"), dane.czas)
                    }else{String::new()}
                )
            ).into()
    } else {
        text(format!("{}: {}",jezyk.t("mgt_proces_error"),dane.błąd)).font(jezyk.get_font()).color(temat.obecny_theme.err_font).into()
    }
}