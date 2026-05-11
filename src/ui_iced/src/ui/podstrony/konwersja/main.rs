use iced::Element;
use iced::widget::{Column, Row};
use iced_core::Length;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::enums_structs_io::LogPrzetwarzanieFot;
use enumy::inne_ui::{UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podstrony::konwersja::inne::reszta;
use crate::ui::podstrony::konwersja::lewa::strona_wyboru;
use crate::ui::podstrony::konwersja::rozdzielczosci::rozdzielczosci;
use crate::ui::podstrony::konwersja::rozszerzenia::main::rozszerzenia;
use crate::ui::podstrony::konwersja::sciezki::sciezki;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::oddzielacze::oddzielacz_pionowy;

pub fn konwersja_view<'a>(
    dane: &'a DaneKonw,
    log: &'a LogPrzetwarzanieFot,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    let prawe = match  temat.temp.act_window {
        UiPods::KonwPath => {sciezki(dane, log, &temat.kolory.konwersja, jezyk, temat) }
        UiPods::KonwExt => {rozszerzenia(dane, &temat.kolory.konwersja, jezyk, temat)}
        UiPods::KonwRes => {rozdzielczosci(dane, &temat.kolory.konwersja, jezyk, temat)}
        UiPods::KonwEtc => {reszta(dane, &temat.kolory.konwersja, jezyk, temat)}
        _ => {Column::new()}
    };

    Row::new()
        .push(strona_wyboru(dane, &temat.kolory.konwersja,jezyk,temat).width(Length::FillPortion(1)).padding(15).spacing(15))
        .push(oddzielacz_pionowy())
        .push(prawe.width(Length::FillPortion(2))).into()

}