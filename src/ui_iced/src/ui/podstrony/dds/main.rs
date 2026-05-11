use iced::Element;
use iced::widget::{space, Column, Row};
use iced_core::Length;
use enumy::dane_do_przetwarzania::{DaneDdsPak, DaneDdsUnpak};
use enumy::inne_ui::{UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podstrony::dds::lewy::strona_wyboru;
use crate::ui::podstrony::dds::pakowanie::pakowanie;
use crate::ui::podstrony::dds::rozpakowywanie::rozpakowywanie;
use crate::ui::podstrony::dds::rozszerzenia::rozszerzenia;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::oddzielacze::oddzielacz_pionowy;

pub fn dds_view<'a>(dane_p: &'a DaneDdsPak, dane_d: &'a DaneDdsUnpak, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    let prawy = match temat.temp.aktywne_okno{
        UiPods::DdsPak => {pakowanie(dane_p, &temat.kolory.dds, jezyk, temat)}
        UiPods::DdsUnpak => {rozpakowywanie(dane_d, &temat.kolory.dds, jezyk, temat).padding(15).spacing(15)}
        UiPods::DdsExt => {rozszerzenia(dane_d, &temat.kolory.dds, jezyk, temat)}
        _ => {Column::new().push(space())}
    };
    Row::new()
        .push(strona_wyboru(jezyk,temat).width(Length::FillPortion(1)).padding(15).spacing(15))
        .push(oddzielacz_pionowy())
        .push(prawy.width(Length::FillPortion(2)))
        .into()
}