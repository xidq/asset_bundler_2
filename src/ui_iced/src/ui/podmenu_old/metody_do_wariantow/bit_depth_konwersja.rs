use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use iced::Element;
use iced_core::Color;
use strum::EnumMessage;
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec};
use enumy::inne_ui::{BtnState, UstawieniaThemeWsio};
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::wewnetrzne::btn_uniwersalny;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn btn_bdepth_jpg_konwersja<'a>(
    bdepth: BdepthJpg,
    id: &'static str,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
    
    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth), id)), kolor, temat),
    bdepth.get_detailed_message().unwrap_or("brak danych"),
    temat
    )
    
}
pub fn btn_bdepth_png_konwersja<'a>(
    bdepth: BdepthPng,
    id: &'static str,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Png, Arc::new(bdepth), id)), kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )

}
pub fn btn_bdepth_webp_konwersja<'a>(
    bdepth: BdepthWebp,
    id: &'static str,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Webp, Arc::new(bdepth), id)), kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )

}
pub fn btn_bdepth_avif_konwersja<'a>(
    bdepth: BdepthAvif,
    bool: bool,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
    
    let state = if bool{&BtnState::Active} else { &BtnState::Disabled };

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), state, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Avif, Arc::new(bdepth))), kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )

}
pub fn btn_bdepth_tga_konwersja<'a>(
    bdepth: BdepthTga,
    id: &'static str,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Tga, Arc::new(bdepth), id)), kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )

}
pub fn btn_bdepth_qoi_konwersja<'a>(
    bdepth: BdepthQoi,
    id: &'static str,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Qoi, Arc::new(bdepth), id)), kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )

}
// pub fn btn_komp_ff_konwersja<'a>(
//     kompresja: OptMetodaKompresjiZdjecia,
//     dane:Arc<DaneDoBathKonwersjaZdjec>,
//     font:iced::Font,
//     kolor:&'a Color,
//     temat:&'a UstawieniaThemeWsio
// ) -> Element<'a, Message>{
// 
//     let  var=
//         if let Some(Rozszerzenia::Ff { metoda_kompresji, .. }) =
//             dane
//                 .rozszerzenia_plików_zdjęciowych
//                 .iter()
//                 .find(|f| matches!(f, Rozszerzenia::Ff { .. }))
//         {
//             matches!(metoda_kompresji,&kompresja)
//         } else { false };
// 
//     hint_btn(
//         btn_uniwersalny(kompresja.get_message().unwrap_or("brak danych").to_string(), None, id, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(kompresja)), var, kolor, temat),
//         kompresja.get_detailed_message().unwrap_or("brak danych"),
//         temat
//     )
// }