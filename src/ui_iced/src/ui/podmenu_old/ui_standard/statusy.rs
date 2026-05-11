use iced::Element;
use iced::widget::{progress_bar, space, text, Row};
use iced::widget::sensor::Key;
use iced_core::{Color, Length};
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::lang::odmiana_liczbowa;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::style_fn::progress_bar::styl_progress_bar;
use crate::ui::wiadomosci::message_ui::Message;

// trait ToU32 {
//     fn to_u32(&self) -> u32;
// }
// 
// impl ToU32 for i32 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for i16 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for i8 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for f64 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for f32 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for u16 { fn to_u32(&self) -> u32 { *self as u32 } }
// impl ToU32 for u8 { fn to_u32(&self) -> u32 { *self as u32 } }
// // ... i tak dalej dla innych typów
// fn uniwersalna_konwersja(var: (&dyn ToU32, Option<&dyn ToU32>)) -> (u32, Option<u32>) {
//     let f1 = var.0.to_f32();
//     let mut xxx = None;
// 
//     if let Some(wartosc2) = var.1 {
//         xxx = Some(wartosc2.to_f32())
//     } else {xxx = None}
//     // let f2 = var.1.map(|v| v.to_f32()).unwrap_or(0.0);
// 
//     (f1, xxx)
// }

pub fn status_progress_bar_pliki<'a>(var: (u32, Option<u32>), napis: &'static str, jezyk: &'a WybórJęzyka, kolor: &'a Color, temat: &'a UstawieniaThemeWsio) -> Element<'a,Message>{
    let tekst_logów = |x:String| -> Element<'a,Message>{
        text(x).color(KOLOR_CZCIONKI_SREDNI).font(jezyk.get_font()).size(12.).width(Length::FillPortion(4)).into()
    };

        if let Some(wartość) = var.1{
            Row::new()
                .push(tekst_logów(
                    if wartość > var.0 {
                        format!("{}: ",jezyk.t(&*(napis.to_owned() + "_pending")))
                    }else{
                        format!("{} {} {}",jezyk.t(napis),wartość, jezyk.t(&odmiana_liczbowa(wartość)) )
                    }
                ))
                .push(space().width(Length::FillPortion(1)))

                .push(
                    progress_bar(
                        0.0..=wartość as f32,
                        var.0 as f32
                    ).style(styl_progress_bar(kolor,temat)).girth(12.).length(Length::FillPortion(4))
                )
                .push(space().width(Length::FillPortion(1))).into()
        }else{
            Row::new().into()
        }

}

