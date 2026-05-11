use iced::widget::{space, Column};
use iced_core::{Color, Length};
use enumy::inne_ui::{UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::przycisk_podmenu;

pub fn strona_wyboru<'a>(jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new()
        .push(space().height(Length::FillPortion(1)))
        .push(przycisk_podmenu(UiPods::BinPak, jezyk, temat))
        .push(przycisk_podmenu(UiPods::BinUnpak, jezyk, temat))
        .push(space().height(Length::FillPortion(1)))

}