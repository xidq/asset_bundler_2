use crate::ui::program_pomniejsze::style_fn::hint::styl_hint;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::inne_ui::UstawieniaThemeWsio;
use iced::widget::{tooltip, Button, PickList};
use iced::Element;
use std::borrow::Borrow;
use std::fmt::Display;

pub fn hint_btn<'a>(xx: Button<'a,Message>, hint:&'static str,temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
    if temat.ustawienia.halp_menu{
    tooltip(
        xx,
        hint,
        tooltip::Position::Bottom
    )
    .style(styl_hint(temat)).padding(10)
    .into()
    }else{
    xx.into()
    }
}
// pub fn hint_text<'a>(xx: Text<'a,iced::Theme, iced::Renderer>, hint:&'static str, var:&'a bool,temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
//     if *var{
//         tooltip(
//             xx,
//             hint,
//             tooltip::Position::Bottom
//         )
//             .style(styl_hint(temat)).padding(10)
//             .into()
//     }else{
//         xx.into()
//     }
// }
pub fn hint_pick_list<'a, T, L, V>(xx: PickList<'a, T, L, V, Message, iced::Theme, iced::Renderer>, hint:&'static str, temat:&'a UstawieniaThemeWsio) -> Element<'a, Message>
where
// T musi być możliwe do wyświetlenia, porównania i sklonowania
T: Display + PartialEq + Clone + 'static,
// L musi być kolekcją elementów T (np. &[T] lub Vec<T>)
L: Borrow<[T]> + 'a,
// V musi być pojedynczym elementem T (np. Option<T> lub T)
V: Borrow<T> + 'a,
{
    if temat.ustawienia.halp_menu{
        tooltip(
            xx,
            hint,
            tooltip::Position::Bottom
        )
            .style(styl_hint(temat)).padding(10)
            .into()
    }else{
        xx.into()
    }
}