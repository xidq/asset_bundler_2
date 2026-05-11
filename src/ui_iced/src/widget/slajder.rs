use iced::Element;
use iced::widget::slider;
use iced_core::{Color, Length};
use enumy::inne_ui::{SliderType, UstawieniaThemeWsio};
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::styles::styl_sliderów;

pub fn slajderr<'a>(wartość: i32, min_max: (i32,i32), rodzaj: &'a SliderType, kolor: &'a Color, temat: &'a UstawieniaThemeWsio, len: Length) -> Element<'a, Message> {
    slider(
        min_max.0..=min_max.1,
        wartość,
        |vv|Message::Slidery(rodzaj.clone(), vv)
    )
        .height(50.)
        .width(len)
        .style(styl_sliderów(kolor,temat)).into()
}