use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use iced::Element;
use iced_core::Color;
use strum::EnumMessage;
use enumy::dane_do_przetwarzania::DaneDoRozpakowaniaDds;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::wewnetrzne::btn_uniwersalny;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;

pub fn btn_bdepth_jpg_dds<'a>(
    bdepth: BdepthJpg,
    bdepth_var:Vec<BdepthJpg>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
    bdepth.get_detailed_message().unwrap_or("brak danych"),
    temat
    )
}
pub fn btn_bdepth_png_dds<'a>(
    bdepth: BdepthPng,
    bdepth_var:Vec<BdepthPng>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_webp_dds<'a>(
    bdepth: BdepthWebp,
    bdepth_var:Vec<BdepthWebp>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);

    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_avif_dds<'a>(
    bdepth: BdepthAvif,
    bdepth_var:Vec<BdepthAvif>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_tga_dds<'a>(
    bdepth: BdepthTga,
    bdepth_var:Vec<BdepthTga>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_bdepth_qoi_dds<'a>(
    bdepth: BdepthQoi,
    bdepth_var:Vec<BdepthQoi>,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = bdepth_var.contains(&bdepth);


    hint_btn(
        btn_uniwersalny(bdepth.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::Bdepth(RozszerzeniaZnacznik::Jpg, Arc::new(bdepth))), var, kolor, temat),
        bdepth.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}
pub fn btn_komp_ff_dds<'a>(
    kompresja: OptMetodaKompresjiZdjecia,
    dane:&OptMetodaKompresjiZdjecia,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let var = *dane == kompresja;

    hint_btn(
        btn_uniwersalny(kompresja.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(DdsMessage::ZdjeciaLaczenieZmianaRozszerzenieFf(kompresja)), var, kolor, temat),
        kompresja.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}

