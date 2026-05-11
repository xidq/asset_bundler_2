use iced::Element;
use iced_core::Color;
use strum::EnumMessage;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::podmenu::ui_standard::uniwersalny_btn::btn_uniwersalny_tak;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn btn_rozdzielczosci<'a>(
    rozdzielczosc: Rozdzielczości,
    jezyk: &'a WybórJęzyka,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    hint_btn(
        btn_uniwersalny_tak(
            rozdzielczosc.get_message().unwrap_or("błąd danych rozdzielczosc").to_string(),
            rozdzielczosc.bath_konwersja_id(),
            jezyk.get_font(),
            Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozdzielczość(rozdzielczosc.clone())),
            kolor,
            temat
        ),
        jezyk.t(rozdzielczosc.get_detailed_message().unwrap_or("błąd danych rozdzielczosc detailed")),
        temat
    )
    

}