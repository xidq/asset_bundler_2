use std::sync::Arc;
use iced::Element;
use iced_core::Color;
use strum::EnumMessage;
use enumy::dane_do_przetwarzania::{DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::inne_ui::{BtnState, UstawieniaThemeWsio};
use enumy::rozszerzenia::rozszerzenia::RozszerzeniaZnacznik;
use crate::ui::podmenu::metody_do_wariantow::wewnetrzne::btn_uniwersalny;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::message_ui::Message::ZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn rozszerzenie_laczenie<'a>(dane:Arc<&DaneDoŁączeniaZdjęć>, rozszerzenie:RozszerzeniaZnacznik, kolor: &'a Color, font: iced::Font, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{

    let var = dane.tag == rozszerzenie;



        hint_btn(
        btn_uniwersalny(rozszerzenie.get_message().unwrap_or("brak danych").to_string(), font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Rozszerzenia(rozszerzenie.clone())), var, kolor, temat),
        rozszerzenie.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}

pub fn rozszerzenie_dds<'a>(dane:Arc<&DaneDoRozpakowaniaDds>,rozszerzenie:RozszerzeniaZnacznik, kolor: &'a Color, font: iced::Font, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{

    let var = dane.tag == rozszerzenie;



    hint_btn(
        btn_uniwersalny(rozszerzenie.get_message().unwrap_or("brak danych").to_string(), font, Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Rozszerzenia(rozszerzenie.clone())), var, kolor, temat),
        rozszerzenie.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
}

pub fn rozszerzenie_konwersja<'a>(rozszerzenie:RozszerzeniaZnacznik, status: BtnState, kolor: &'a Color, font: iced::Font, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{
    
    hint_btn(
        btn_uniwersalny(rozszerzenie.get_message().unwrap_or("brak danych").to_string(), status, font, Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozszerzenia(rozszerzenie.clone())),  kolor, temat),
        rozszerzenie.get_detailed_message().unwrap_or("brak danych"),
        temat
    )
    
}