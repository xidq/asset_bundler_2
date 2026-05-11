use iced::widget::{pick_list, slider, text, Column};
use iced_core::Length;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::opcje::OptInterpolacja;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn inne<'a>(dane: &'a DaneDoBathKonwersjaZdjec, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    let opcje_interpolacja: Vec<String> = OptInterpolacja::WSIOINTERPOLACJI
        .iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    Column::new()
        //OptInterpolacja
        .push(text(jezyk.t("foto_edit_interpolation")).font(jezyk.get_font()))
        .push(
            pick_list(
                opcje_interpolacja,
                Some(jezyk.t(dane.inter.klucz()).to_string()),
                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Interpolacja(xx)), // Wysyła String
            )
                .width(Length::Fill)
                .padding(10)
                .style(styl_pick_list(&temat.kolory.konwersja,temat))
                .menu_style(styl_menu_pick(&temat.kolory.konwersja,temat)),
        )
        .push(text(match dane.noising {
            Some(x) => format!("{} {}%", jezyk.t("foto_edit_noising"), x),
            None => format!(
                "{} {}",
                jezyk.t("foto_edit_noising"),
                jezyk.t("general_off")
            ),
        }))
        .push(
            slider(
                0..=100,
                dane.noising.unwrap_or(0),
                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Noising(xx)) ,
            )
                .style(styl_sliderów(&temat.kolory.konwersja,temat)),
        )
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2))
}