use iced::Element;
use iced::widget::{Column, Row};
use iced_core::Length;
use enumy::dane_do_przetwarzania::{DaneBinUnpak, DaneBinPak};
use enumy::enums_structs_io::{LogPakowanie, LogRozpakowywanie};
use enumy::inne_ui::{ActProces, UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podstrony::binarka::lewa::strona_wyboru;
use crate::ui::podstrony::binarka::pakowanie::binarka_pak;
use crate::ui::podstrony::binarka::rozpakowanie::binarka_rozpakowanie;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::oddzielacze::oddzielacz_pionowy;

pub fn binarka_view<'a>(
    dane_p: &'a DaneBinPak,
    dane_d: &'a DaneBinUnpak,
    log_p: &'a LogPakowanie,
    log_d: &'a LogRozpakowywanie,
    jezyk:&'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio
) -> Element<'a, Message> {
    let prawa = match temat.temp.act_window {
        UiPods::BinPak => {binarka_pak(dane_p, log_p, &temat.kolory.binarka, jezyk, temat)}
        UiPods::BinUnpak => {binarka_rozpakowanie(dane_d, log_d, &temat.kolory.binarka, jezyk, temat)}
        _ => Column::new()
    };
    
    Row::new().push(strona_wyboru(jezyk,temat).width(Length::FillPortion(1)).padding(15).spacing(15)).push(oddzielacz_pionowy()).push(prawa.width(Length::FillPortion(2)).padding(15).spacing(15)).into()
}