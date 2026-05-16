use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::btn_rozdzielczosci;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, UstawieniaThemeWsio};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row};
use iced_core::Color;

pub fn rozdzielczosci<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new()
        .push(
            Row::new()
                .push(btn_rozdzielczosci(Rozdzielczości::R16, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R16){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R32, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R32){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R64, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R64){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R128, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R128){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat)).spacing(10)
        )
        .push(
            Row::new()
                .push(btn_rozdzielczosci(Rozdzielczości::R256, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R256){&BtnState::Processing}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R512, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R512){&BtnState::Processing}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R1k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R1k){&BtnState::Processing}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R2k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R2k){&BtnState::Processing}else{&BtnState::Disabled}, kolor, temat)).spacing(10)
        )
        .push(
            Row::new()
                .push(btn_rozdzielczosci(Rozdzielczości::R4k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R4k){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R6k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R6k){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R8k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R8k){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat))
                .push(btn_rozdzielczosci(Rozdzielczości::R16k, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::R16k){&BtnState::Active}else{&BtnState::Disabled}, kolor, temat)).spacing(10)
        )
        .push(
            Row::new()
                .push(btn_rozdzielczosci(Rozdzielczości::Oryginalna, jezyk, if dane.opcje_rozdzielczości.contains(&Rozdzielczości::Oryginalna){&BtnState::Processing}else{&BtnState::Disabled}, kolor, temat)).spacing(10)
        )
            .spacing(10)
}