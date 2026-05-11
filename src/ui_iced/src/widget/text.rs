use crate::ui::wiadomosci::message_ui::Message;
use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::text;
use iced::Element;
use iced_core::Color;
pub(crate) const ROZMIARWYBRANYCHROZSZERZEN: iced::Pixels = iced::Pixels(12.);

pub fn info_male<'a>(nazwa:String, var: bool, _temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
    text(nazwa)
        .font(iced::Font {
            family: iced::font::Family::Name("VT323"),
            ..Default::default()
        })
        .color(Color::from_rgba(
            1.,
            1.,
            1.,
            if var { 0.5 } else { 0.2 },
        ))
        .size(ROZMIARWYBRANYCHROZSZERZEN).into()
}