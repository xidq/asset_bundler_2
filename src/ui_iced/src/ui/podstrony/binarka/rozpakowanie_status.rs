use iced::Element;
use iced::widget::{text, Column};
use iced_core::Color;
use enumy::enums_structs_io::LogRozpakowywanie;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::status::{status_text, status_text_bar};

pub fn status_rozpakowanie<'a>(dane: &'a LogRozpakowywanie, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{
    if dane.błąd.is_empty() {
        Column::new()
            .push(status_text_bar(jezyk.normal_u64_option(dane.kontrola_pliku), "proces_binary_unpack_files", kolor, jezyk, temat))
            .push(status_text_bar(jezyk.normal_u32_option(dane.deszyfrowanie), "proces_binary_unpack_decoding", kolor, jezyk, temat))

            .push(
                status_text(
                if dane.dekompresja > 0 {
                    format!("{}: {}", jezyk.t("proces_binary_unpack_decompression"), jezyk.bajt(dane.dekompresja))
                }else{
                    String::new()
                },
                jezyk
                )

            )
            .push(status_text_bar(jezyk.normal_u32_option(dane.rozpakowanie), "proces_binary_unpack_unpacking", kolor, jezyk, temat))

            .push(
                status_text(
                    if !dane.czas.is_empty() {
                        format!("{}: {}", jezyk.t("proces_binary_unpack_end"), dane.czas)
                    }else{String::new()},
                    jezyk
                )
            ).into()
    } else {
        text(format!("{}: {}",jezyk.t("mgt_proces_error"),dane.błąd)).font(jezyk.get_font()).color(temat.obecny_theme.err_font).into()
    }
}