use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::status::{status_text, status_text_bar};
use enumy::enums_structs_io::LogPakowanie;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::lang::odmiana_liczbowa;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{text, Column};
use iced::Element;
use iced_core::Color;

pub fn status_pakowanie<'a>(dane: &'a LogPakowanie, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{

    if dane.błąd.is_empty() {
        Column::new()
            .push(
                status_text(
                    if dane.zbieranie_plików > 0 {
                        format!("{}: {} {}",jezyk.t("proces_binary_pack_collecting_pending"),dane.zbieranie_plików, jezyk.t(&odmiana_liczbowa(dane.zbieranie_plików)))
                    } else {String::new()}
                ,jezyk)
            )
            .push(status_text_bar(jezyk.normal_u32_option(dane.pakowanie), "proces_binary_pack_packing", kolor, jezyk, temat))
            .push(status_text_bar(jezyk.normal_u64_option(dane.kompresja), "proces_binary_pack_compression", kolor, jezyk, temat))
            .push(status_text_bar(jezyk.normal_u32_option(dane.szyfrowanie), "proces_binary_pack_compression", kolor, jezyk, temat))
            .push(
                status_text(
                    if !dane.koniec.is_empty() {
                        format!("{}: {}", jezyk.t("proces_binary_pack_end"), dane.koniec)
                    } else {
                        "".to_string()
                    }
                ,jezyk)
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