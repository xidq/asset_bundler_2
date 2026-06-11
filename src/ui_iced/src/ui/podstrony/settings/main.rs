use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::styles::{styl_menu_pick, styl_pick_list, styl_przycisków, styl_sliderów};
use enumy::inne_ui::{BtnState, UiPods, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::widget::{button, pick_list, slider, text, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn ustawienia_view<'a>(opcje: &UstawieniaMenu, jezyk: &'a WybórJęzyka, temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
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
                WybórJęzyka::iter().collect::<Vec<_>>(), // Przekazujemy CAŁĄ listę (slice)
                aktualnie_wybrany_jezyk,
                Message::DevZmienJezyk // Akcja po kliknięciu
            )
                .placeholder("Wybierz...")
                .width(200)
                .style(styl_pick_list(&temat.kolory.ustawienia,  temat))
                .menu_style(styl_menu_pick(&temat.kolory.ustawienia,  temat)),
        )
        .push(
            button("resetuj").on_press(Message::DevResetUstawien).style(styl_przycisków(&BtnState::Disabled, &temat.kolory.ustawienia, temat))
        )
        .push(wybor_koloru("Binarka".to_string(), &UiPods::BinPak, jezyk, &temat.kolory.binarka, temat))
        .push(wybor_koloru("Konwersja".to_string(), &UiPods::KonwEtc, jezyk, &temat.kolory.konwersja, temat))
        .push(wybor_koloru("Łączenie".to_string(), &UiPods::Merge, jezyk, &temat.kolory.laczenie, temat))
        .push(wybor_koloru("Dds".to_string(), &UiPods::DdsExt, jezyk, &temat.kolory.dds, temat))
        .push(wybor_koloru("Ustawienia".to_string(), &UiPods::Ustawienia, jezyk, &temat.kolory.ustawienia, temat))
        .spacing(15)
        .into()

}
fn wybor_koloru<'a>(tekst: String, typ: &'a UiPods, jezyk: &'a WybórJęzyka, kolor_wybrany: &'a Color, temat: &'a UstawieniaThemeWsio) -> Row<'a, Message> {
    Row::new().padding(15).spacing(15).height(50.)
        .push(
            text(tekst).size(20).font(jezyk.get_font()).height(Length::Fill).center()
        )
        .push(text("R").size(20).font(jezyk.get_font()).height(Length::Fill).center())
        .push(
            slider(0..=255_u8,(kolor_wybrany.r * 255.) as u8,|xx|Message::DevZmienKolory(*typ, "r".to_string(), xx)).style(styl_sliderów(kolor_wybrany, temat))
        )
        .push(text("G").size(20).font(jezyk.get_font()).height(Length::Fill).center())
        .push(
            slider(0..=255_u8,(kolor_wybrany.g * 255.) as u8,|xx|Message::DevZmienKolory(*typ, "g".to_string(), xx)).style(styl_sliderów(kolor_wybrany, temat))
        )
        .push(text("B").size(20).font(jezyk.get_font()).height(Length::Fill).center())
        .push(
            slider(0..=255_u8,(kolor_wybrany.b * 255.) as u8,|xx|Message::DevZmienKolory(*typ, "b".to_string(), xx)).style(styl_sliderów(kolor_wybrany, temat))
        )
}