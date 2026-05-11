use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::wiadomosci::message_ui::Message;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::widget::{pick_list, text, Column};
use iced::Element;

pub fn ui_ustawienia<'a>(opcje: &UstawieniaMenu, temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
    let aktualnie_wybrany_jezyk = match opcje {
        UstawieniaMenu::UstawieniaJęzyka { jezyk } => Some(*jezyk),
        _ => None, // Na wypadek gdyby dev_opcje były innym wariantem
    };
    


    Column::new()
        .push(
        text("Wybierz język").size(22),
        )
        .push(
        pick_list(
            &WybórJęzyka::ALL[..], // Przekazujemy CAŁĄ listę (slice)
            aktualnie_wybrany_jezyk,
            Message::DevZmienJezyk // Akcja po kliknięciu
        )
        .placeholder("Wybierz...")
        .width(200)
        .style(styl_pick_list(&temat.kolory.ustawienia,  temat))
        .menu_style(styl_menu_pick(&temat.kolory.ustawienia,  temat)),
        )
    .spacing(15)
    .into()
}
