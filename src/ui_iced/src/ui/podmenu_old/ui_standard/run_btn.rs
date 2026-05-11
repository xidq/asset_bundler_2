use iced::Element;
use iced::widget::{button, text, Button};
use iced_core::{Color, Length, Widget};
use enumy::inne_ui::{ActProces, BtnState, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;

pub fn start_btn<'a>(
    proces: ActProces,
    var: bool,
    hint:&'static str, 
    hint2:&'static str, 
    kolor: &'a iced::Color, 
    jezyk:&'a WybórJęzyka, 
    temat: &'a UstawieniaThemeWsio
) -> Element<'a,Message> {

    let msgg = match proces{

        ActProces::PakowaniePliku => Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom),
        ActProces::RozpakowaniePliku => Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::Uruchom),
        ActProces::DdsPakowanie => Message::Dds(DdsMessage::PakowanieStart),
        ActProces::DdsRozpakowanie => Message::Dds(DdsMessage::RozpakStart),
        ActProces::ŁączenieZdjęć => Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec),
        ActProces::KonwersjaZdjęć => Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Uruchom),
        // _ => Message::Nic

    };
    let (status, msg, label) = match temat.temp.aktywny_proces {
        Some(p) if p == proces => (BtnState::Processing, None, "mgt_btn_busy_processing"),
        Some(_) => (BtnState::Disabled, None, "mgt_btn_busy_processing_other"),
        None if !var => (BtnState::LackData, None, "mgt_btn_gib_data"),
        None => (BtnState::Active, Some(msgg), "mgt_btn_ready"),
    };


    hint_btn(

            button(
                text(
                    jezyk.t(label)
                )
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.8))
                    .width(Length::Fill)
                    .center(),
            )
                .padding(12)
                .height(Length::Fixed(50.))
                .width(Length::Fill)
                .on_press_maybe(msg)
                .style(styl_przycisków(
                    &status,
                    kolor,
                    temat)
                )
        ,
    jezyk.t(if temat.btn_state.get(proces.get_proces_id()).unwrap_or(&BtnState::Disabled) == &BtnState::Active {hint}else{hint2}),
    temat
    )
}