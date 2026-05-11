use iced::widget::{space, Column};
use iced_core::Length;
use enumy::inne_ui::{UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::przycisk_podmenu;

pub fn strona_wyboru<'a>(jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new()
        .push(space().height(Length::FillPortion(1)))
        .push(przycisk_podmenu(UiPods::DdsPak, jezyk, temat))
        .push(przycisk_podmenu(UiPods::DdsUnpak, jezyk, temat))
        .push(
        if matches!(temat.temp.aktywne_okno, UiPods::DdsUnpak | UiPods::DdsExt) {
                przycisk_podmenu(UiPods::DdsExt, jezyk, temat)
            } else {
                space().height(50.).into()
            }
        )
        .push(space().height(Length::FillPortion(1)))

}