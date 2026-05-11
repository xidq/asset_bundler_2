use iced::widget::{space, Column};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{ActProces, UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podstrony::konwersja::wybory::wybory;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::{przycisk_podmenu, przycisk_startu};

pub fn strona_wyboru<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new()
        .push(space().height(Length::FillPortion(1)))
        .push(przycisk_podmenu(UiPods::KonwPath, jezyk, temat))
        .push(przycisk_podmenu(UiPods::KonwExt, jezyk, temat))
        .push(przycisk_podmenu(UiPods::KonwRes, jezyk, temat))
        .push(przycisk_podmenu(UiPods::KonwEtc, jezyk, temat))
        .push(space().height(Length::Fixed(10.)))
        .push(wybory(dane,temat))
        // .push(przycisk_startu(&ActProces::KonwersjaZdjęć, kolor, jezyk, temat))

        .push(space().height(Length::FillPortion(1)))

}