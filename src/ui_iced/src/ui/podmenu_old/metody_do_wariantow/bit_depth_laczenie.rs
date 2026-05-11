use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use iced::Element;
use iced_core::Color;
use strum::EnumMessage;
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoObrbki, DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaPojedyncze, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::wewnetrzne::btn_uniwersalny;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;

pub fn btn_bdepth_jpg_laczenie<'a>(
    bdepth: BdepthJpg,
    bdepth_var: &BdepthJpg,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
    bdepth.get_detailed_message().unwrap_or("brak danych"),
    temat
    )
}
pub fn btn_bdepth_png_laczenie<'a>(
    bdepth: BdepthPng,
    bdepth_var: &BdepthPng,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_webp_laczenie<'a>(
    bdepth: BdepthWebp,
    bdepth_var: &BdepthWebp,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_avif_laczenie<'a>(
    bdepth: BdepthAvif,
    bdepth_var: &BdepthAvif,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_tga_laczenie<'a>(
    bdepth: BdepthTga,
    bdepth_var: &BdepthTga,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_qoi_laczenie<'a>(
    bdepth: BdepthQoi,
    bdepth_var: &BdepthQoi,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth == *bdepth_var;

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_komp_ff_laczenie<'a>(
    kompresja: OptMetodaKompresjiZdjecia,
    kompresja_var: & OptMetodaKompresjiZdjecia,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = kompresja == *kompresja_var;

    hint_btn(
        btn_uniwersalny(kompresja.get_message().unwrap_or("brak danych").to_string(), None, font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(kompresja)), var, kolor, temat),
        kompresja.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}