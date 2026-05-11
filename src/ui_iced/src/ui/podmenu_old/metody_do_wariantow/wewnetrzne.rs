use std::any::Any;
use std::sync::Arc;
use iced::Element;
use iced::widget::{button, text, Button};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoObrbki, DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::inne_ui::{ActProces, BtnState, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaPojedyncze, RozszerzeniaZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::podmenu::konwersja::main_konwersja::ROZMIARWYBRANYCHROZSZERZEN;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;



pub fn btn_uniwersalny<'a>(label: String, state: &'a BtnState,font: iced::Font, msg: Message,  kolor: &'a Color, temat: &'a UstawieniaThemeWsio  ) -> Button<'a, Message>{
    button(
        text(
            label)
            .color(KOLOR_CZCIONKI_SREDNI)
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(msg)
        .style(
            styl_przycisków(
                state,
                kolor,
                temat
            )
        )
}
pub fn info_male<'a>(nazwa:String, id: &'static str, temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
    text(nazwa)
        .font(iced::Font {
            family: iced::font::Family::Name("VT323"),
            ..Default::default()
        })
        .color(Color::from_rgba(
            1.,
            1.,
            1.,
            if temat.btn_state.get(id) == Some(&BtnState::Pressed) { 0.5 } else { 0.2 },
        ))
        .size(ROZMIARWYBRANYCHROZSZERZEN).into()
}



// pub fn btn_startu<'a>(var:bool, proces: ActProces, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
//
//
//     let msg = match proces{
//         ActProces::PakowaniePliku => {Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom)}
//         ActProces::RozpakowaniePliku => {Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::Uruchom)}
//         ActProces::DdsPakowanie => {Message::Dds(DdsMessage::PakowanieStart)}
//         ActProces::DdsRozpakowanie => {Message::Dds(DdsMessage::RozpakStart)}
//         ActProces::ŁączenieZdjęć => {Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec)}
//         ActProces::KonwersjaZdjęć => {Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Uruchom)}
//         _ => {Message::Nic}
//     };
//     if var && temat.temp.aktywny_proces == ActProces::Żodyn {
//         hint_btn(
//             button(
//                 text(jezyk.t("process_btn_start"))
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.8))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .on_press(msg)
//                 .height(Length::Fixed(50.))
//                 .width(Length::Fill)
//                 .style(styl_przycisków(false, false, kolor,temat)),
//             jezyk.t(""),
//             temat
//         )
//     } else {
//         hint_btn(
//             button(
//                 text(if temat.temp.aktywny_proces == proces {
//                     jezyk.t("btn_bussy_processing")
//                 } else if temat.temp.aktywny_proces != ActProces::Żodyn {
//                     jezyk.t("btn_bussy_processing_other")
//                 } else {
//                     jezyk.t("btn_gib_data")
//                 })
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.7))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .width(Length::Fill)
//                 .height(Length::Fixed(50.))
//                 .style(styl_przycisków(
//                     false,
//                     temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
//                     kolor, temat
//                 )),
//             jezyk.t(""),
//             temat
//         )
//     }
// }