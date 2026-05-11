use crate::ui::podmenu::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::podmenu::konwersja::main_konwersja::ROZMIARWYBRANYCHROZSZERZEN;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoObrbki, DaneDoPakowaniaDds, DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::inne_ui::{ActProces, BtnState, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaPojedyncze, RozszerzeniaZnacznik};
use iced::widget::{button, text, Button};
use iced::Element;
use iced_core::{Color, Length};
use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;
use strum::EnumMessage;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;

// pub fn btn_zbiorowe_rozszerzenia<'a>(
//     lell: RozszerzeniaZnacznik,
//     co_istnieje:&dyn DaneDoObrbki,
//     font:iced::Font,
//     kolor:&'a Color,
//     temat:&'a UstawieniaThemeWsio
// ) -> Element<'a, Message>{
// 
//     let msg =  match co_istnieje.jako_any(){
//         any if any.is::<DaneDoBathKonwersjaZdjec>() => {
//             Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozszerzenia(lell.clone()))
//         },
//         any if any.is::<DaneDoRozpakowaniaDds>() => {
//             Message::Dds(DdsMessage::Rozszerzenia(lell.clone()))
//         },
//         any if any.is::<DaneDoŁączeniaZdjęć>() => {
//             Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Rozszerzenia(lell.clone()))
//         },
// 
//         _ => {Message::Nic}
//     };
// 
//     hint_btn(
//         button(
//             text(
//                 lell.get_message().unwrap_or("Brak Danych"))
//                 .color(Color::from_rgba(1., 1., 1., 0.6))
//                 .font(font)
//                 .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
//                 .width(Length::Fill)
//                 .center(),
//         )
//             .on_press(msg)
//             .style(styl_przycisków(
//                 false,
//                 co_istnieje.tag_master().contains(&lell),
//                 kolor,
//                 temat,
//             )),
//         lell.get_detailed_message().unwrap_or("Brak danych"),
//         temat
//     )
// }
// pub fn btn_zbiorowe_bdepth<'a>(
//     rozs: RozszerzeniaZnacznik,
//     lell: Arc<dyn BitDepth>,
//     co_istnieje:&dyn DaneDoObrbki,
//     font:iced::Font,
//     kolor:&'a Color,
//     temat:&'a UstawieniaThemeWsio
// ) -> Element<'a, Message>{
// 
// 
//     let czy_wybrany = match co_istnieje.jako_any() {
//         // 1. Obsługa Zbiorowego Przetwarzania (gdzie masz Vec rozszerzeń)
//         any if any.is::<DaneDoBathKonwersjaZdjec>() => {
//             let dane = any.downcast_ref::<DaneDoBathKonwersjaZdjec>().unwrap();
// 
//             dane.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
//                 match (format, &rozs) {
//                     (Rozszerzenia::Jpg { bit_depth, .. }, RozszerzeniaZnacznik::Jpg)  => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     (Rozszerzenia::Png { bit_depth, .. }, RozszerzeniaZnacznik::Png)  => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     (Rozszerzenia::Webp { bit_depth, .. }, RozszerzeniaZnacznik::Webp)  => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     (Rozszerzenia::Tga { bit_depth, .. }, RozszerzeniaZnacznik::Tga)  => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     (Rozszerzenia::Qoi { bit_depth, .. }, RozszerzeniaZnacznik::Qoi)  => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     (Rozszerzenia::Avif { bit_depth, .. }, RozszerzeniaZnacznik::Avif) => {
//                         bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                     }
//                     _ => false,
//                 }
//             })
//         }
// 
//         // 2. Obsługa Rozpakowywania DDS (gdzie masz pojedyncze pole 'rozszerzenie')
//         any if any.is::<DaneDoRozpakowaniaDds>() => {
//             let dane = any.downcast_ref::<DaneDoRozpakowaniaDds>().unwrap();
// 
//             match (&dane.rozszerzenie, &rozs) {
//                 (Rozszerzenia::Jpg { bit_depth, .. }, RozszerzeniaZnacznik::Jpg) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 (Rozszerzenia::Png { bit_depth, .. }, RozszerzeniaZnacznik::Png) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 (Rozszerzenia::Webp { bit_depth, .. }, RozszerzeniaZnacznik::Webp) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 (Rozszerzenia::Tga { bit_depth, .. }, RozszerzeniaZnacznik::Tga) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 (Rozszerzenia::Qoi { bit_depth, .. }, RozszerzeniaZnacznik::Qoi) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 (Rozszerzenia::Avif { bit_depth, .. }, RozszerzeniaZnacznik::Avif) => {
//                     bit_depth.iter().any(|b| lell.as_ref().jest_rowny(b as &dyn Any))
//                 }
//                 _ => false,
//             }
//         }
// 
//         any if any.is::<DaneDoŁączeniaZdjęć>() => {
//             let dane = any.downcast_ref::<DaneDoŁączeniaZdjęć>().unwrap();
// 
//             match (&dane.out_format, &rozs) {
//                 (RozszerzeniaPojedyncze::Jpg { bit_depth, .. }, RozszerzeniaZnacznik::Jpg) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 (RozszerzeniaPojedyncze::Png { bit_depth, .. }, RozszerzeniaZnacznik::Png) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 (RozszerzeniaPojedyncze::Webp { bit_depth, .. }, RozszerzeniaZnacznik::Webp) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 (RozszerzeniaPojedyncze::Tga { bit_depth, .. }, RozszerzeniaZnacznik::Tga) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 (RozszerzeniaPojedyncze::Qoi { bit_depth, .. }, RozszerzeniaZnacznik::Qoi) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 (RozszerzeniaPojedyncze::Avif { bit_depth, .. }, RozszerzeniaZnacznik::Avif) => {
//                     lell.as_ref().jest_rowny(bit_depth as &dyn Any)
//                 }
//                 _ => false,
//             }
//         }
// 
// 
//         _ => false,
//     };
// 
//     let msg =  match co_istnieje.jako_any(){
//         any if any.is::<DaneDoBathKonwersjaZdjec>() => {
//             // dbg!("bath",&rozs,&lell);
//             Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(rozs, Arc::clone(&lell)))
//         },
//         any if any.is::<DaneDoRozpakowaniaDds>() => {
//             // dbg!("dds",&rozs,&lell);
//             Message::Dds(DdsMessage::Bdepth(rozs, Arc::clone(&lell)))
//         },
//         any if any.is::<DaneDoŁączeniaZdjęć>() => {
//             // dbg!("łączenie",&rozs,&lell);
//             Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::Bdepth(rozs, Arc::clone(&lell)))
//         },
// 
//         _ => {Message::Nic}
//     };
// 
// 
// 
//     hint_btn(
//         btn_do_wyborów(lell.label_min().to_string(), font, msg, czy_wybrany, kolor, temat),
//         lell.label_max(),
//         temat
//     )
//     // button(
//     //
//     //     text(
//     //         lell.label_min())
//     //         .color(Color::from_rgba(1., 1., 1., 0.6))
//     //         .font(font)
//     //         .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
//     //         .width(Length::Fill)
//     //         .center(),
//     // )
//     //     .on_press(msg)
//     //     .style(styl_przycisków(
//     //         false,
//     //         czy_wybrany,
//     //         kolor,
//     //         temat
//     //     )).into()
// }
// 
// fn btn_do_wyborów<'a>(label: String, font: iced::Font, msg: Message, warunek: bool, kolor: &'a Color, temat: &'a UstawieniaThemeWsio  ) -> Button<'a, Message>{
//     button(
// 
//         text(
//             label)
//             .color(KOLOR_CZCIONKI_SREDNI)
//             .font(font)
//             .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
//             .width(Length::Fill)
//             .center(),
//     )
//         .on_press(msg)
//         .style(styl_przycisków(
//             false,
//             warunek,
//             kolor,
//             temat
//         ))
// }
// 
// pub fn btn_wyboru_kompresja_ff<'a>(
//     kompresja_target: OptMetodaKompresjiZdjecia,
//     obecna_kompresja: &OptMetodaKompresjiZdjecia,
//     font: iced::Font,
//     kolor: &'a Color,
//     temat: &'a UstawieniaThemeWsio,
// ) -> Element<'a, Message> {
// 
//     // Sprawdzamy czy ten konkretny bit jest na liście
//     let aktywny = std::mem::discriminant(&kompresja_target) == std::mem::discriminant(obecna_kompresja);
// 
//     hint_btn(
//         btn_do_wyborów(kompresja_target.get_message().unwrap_or("brak danych").to_string(), font, Message::Dds(
//             DdsMessage::RozpakExtDane(
//                 Rozszerzenia::Ff {
//                     metoda_kompresji: kompresja_target // tworzymy nową listę z tym jednym bitem
//                 }
//             )
//         ), aktywny, kolor, temat),
//         kompresja_target.get_detailed_message().unwrap(),
//         temat
//     )
// 
// }

pub fn info_male<'a>(nazwa:String, id: &'static str, temat:&'a UstawieniaThemeWsio) -> Element<'a, Message> {
    text(nazwa)
        .font(iced::Font {
            family: iced::font::Family::Name("VT323"),
            ..Default::default()
        })
        .color(Color::from_rgba(
            1.,
            1.,
            1.,
            if temat.btn_state.get(id) == Some(&BtnState::Active) { 0.5 } else { 0.2 },
        ))
        .size(ROZMIARWYBRANYCHROZSZERZEN).into()
}
// pub fn btn_startu<'a>(id: &'static str, proces: ActProces, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
//
//
//     let msg = match proces{
//         ActProces::PakowaniePliku => {Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom)}
//         ActProces::RozpakowaniePliku => {Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::Uruchom)}
//         ActProces::DdsPakowanie => {Message::Dds(DdsMessage::PakowanieStart)}
//         ActProces::DdsRozpakowanie => {Message::Dds(DdsMessage::RozpakStart)}
//         ActProces::ŁączenieZdjęć => {Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec)}
//         ActProces::KonwersjaZdjęć => {Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Uruchom)}
//         _ => {Message::Nic}
//     };
//     if var && temat.temp.aktywny_proces == ActProces::Żodyn {
//         hint_btn(
//             button(
//                 text(jezyk.t("process_btn_start"))
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.8))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .on_press(msg)
//                 .height(Length::Fixed(50.))
//                 .width(Length::Fill)
//                 .style(styl_przycisków(false, false, kolor,temat)),
//             jezyk.t(""),
//             temat
//         )
//     } else {
//         hint_btn(
//             button(
//                 text(if temat.temp.aktywny_proces == proces {
//                     jezyk.t("btn_bussy_processing")
//                 } else if temat.temp.aktywny_proces != ActProces::Żodyn {
//                     jezyk.t("btn_bussy_processing_other")
//                 } else {
//                     jezyk.t("btn_gib_data")
//                 })
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.7))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .width(Length::Fill)
//                 .height(Length::Fixed(50.))
//                 .style(styl_przycisków(
//                     false,
//                     temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
//                     kolor, temat
//                 )),
//             jezyk.t(""),
//             temat
//         )
//     }
// }

// pub fn btn_rozdzielczosci<'a>(
//     dane:&DaneDoBathKonwersjaZdjec,
//     rozdzielczosc: Rozdzielczości,
//     jezyk: &'a WybórJęzyka,
//     kolor:&'a Color,
//     temat:&'a UstawieniaThemeWsio
// ) -> Element<'a, Message> {
// 
//     hint_btn(
//         btn_do_wyborów(rozdzielczosc.to_string(), jezyk.get_font(), Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozdzielczość(rozdzielczosc.clone())), dane.opcje_rozdzielczości.contains(&rozdzielczosc), kolor, temat),
//         jezyk.t(rozdzielczosc.get_detailed_message().unwrap()),
//         temat
//     )
//     
// 
// }
// 
// pub fn btn_startu<'a>(var:bool, proces: ActProces, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
// 
// 
//     let msg = match proces{
//         ActProces::PakowaniePliku => {Message::PakowanieBinarki(PakowanieBinarkiMessage::Uruchom)}
//         ActProces::RozpakowaniePliku => {Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::Uruchom)}
//         ActProces::DdsPakowanie => {Message::Dds(DdsMessage::PakowanieStart)}
//         ActProces::DdsRozpakowanie => {Message::Dds(DdsMessage::RozpakStart)}
//         ActProces::ŁączenieZdjęć => {Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec)}
//         ActProces::KonwersjaZdjęć => {Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Uruchom)}
//         ActProces::Żodyn => {Message::Nic}
//     };
//     if var && temat.temp.aktywny_proces == ActProces::Żodyn {
//         hint_btn(
//             button(
//                 text(jezyk.t("process_btn_start"))
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.8))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .on_press(msg)
//                 .height(Length::Fixed(50.))
//                 .width(Length::Fill)
//                 .style(styl_przycisków(false, false, kolor,temat)),
//             jezyk.t(""),
//             temat
//         )
//     } else {
//         hint_btn(
//             button(
//                 text(if temat.temp.aktywny_proces == proces {
//                     jezyk.t("btn_bussy_processing")
//                 } else if temat.temp.aktywny_proces != ActProces::Żodyn {
//                     jezyk.t("btn_bussy_processing_other")
//                 } else {
//                     jezyk.t("btn_gib_data")
//                 })
//                     .font(jezyk.get_font())
//                     .color(Color::from_rgba(1., 1., 1., 0.7))
//                     .width(Length::Fill)
//                     .center(),
//             )
//                 .width(Length::Fill)
//                 .height(Length::Fixed(50.))
//                 .style(styl_przycisków(
//                     false,
//                     temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
//                     kolor, temat
//                 )),
//             jezyk.t(""),
//             temat
//         )
//     }
// }