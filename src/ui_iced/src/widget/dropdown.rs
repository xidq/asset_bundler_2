use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::styles::{styl_menu_pick, styl_pick_list};
use enumy::implementacje::{DaneDropdown, ElementyDropdown};
use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::pick_list;
use iced::Element;
use iced_core::{Color, Length};
use std::fmt::Display;
use std::sync::Arc;

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