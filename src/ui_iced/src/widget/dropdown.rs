use std::fmt::Display;
use std::sync::Arc;
use iced::advanced::graphics::damage::list;
use iced::Element;
use iced::widget::pick_list;
use iced_core::border::width;
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneDoObrbki;
use enumy::implementacje::{DaneDropdown, ElementyDropdown};
use enumy::inne_ui::UstawieniaThemeWsio;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::styles::{styl_menu_pick, styl_pick_list};

pub fn dropdown<'a, T, F>(
    dane: &'a F,
    kolor: &'a Color,
    temat: &'a UstawieniaThemeWsio
) -> Element<'a, Message>
where
    F: DaneDropdown<T>, T: ElementyDropdown + strum::IntoEnumIterator + Clone + Display + PartialEq + Send + Sync + 'a + 'static
{
    pick_list(
        dane.get_opcje(),
        dane.get_data(),
        |data|Message::Dropdown(Arc::new(data), F::get_dropdown_type())
    )
        .width(Length::FillPortion(2))
        .padding(10)
        .style(styl_pick_list(kolor,temat))
        .menu_style(styl_menu_pick(kolor, temat)).into()
}