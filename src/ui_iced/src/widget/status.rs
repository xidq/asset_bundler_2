use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::colors_n_stuff::KOLOR_CZCIONKI_SREDNI;
use crate::widget::styles::styl_progress_bar;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{progress_bar, space, text, Row};
use iced::Element;
use iced_core::{Color, Length};

pub fn status_text<'a>(tekst: String, jezyk: &'a WybórJęzyka  ) -> Element<'a, Message> {
    text(tekst).color(KOLOR_CZCIONKI_SREDNI).font(jezyk.get_font()).size(12.).width(Length::FillPortion(4)).into()
}
pub fn status_text_bar<'a>( dane: (u32, Option<u32>), napis: &'static str, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio  ) -> Element<'a, Message> {

    if let Some(dane_max) = dane.1 {
        Row::new()
            .push(status_text(
                    format!("{}: ",jezyk.t(&(napis.to_owned() + "_pending"))),
                     jezyk
                // }else{
                //     format!("{} {} {}",jezyk.t(napis),dane_max, jezyk.t(&odmiana_liczbowa(dane_max)) )
                // }, jezyk))
            ))
            .push(space().width(Length::FillPortion(1)))

            .push(
                progress_bar(0.0 ..= dane_max as f32, dane.0 as f32).style(styl_progress_bar(kolor, temat)).girth(5.).length(Length::FillPortion(4))
            )

            .push(space().width(Length::FillPortion(1)))

    } else {Row::new().push(space().height(15.5))}.into()

}