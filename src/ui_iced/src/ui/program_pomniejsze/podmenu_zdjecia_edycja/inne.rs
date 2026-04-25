use iced::Element;
use iced::widget::{button, text};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoŁączeniaZdjęć};
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use crate::ui::program_pomniejsze::kolory::{KOLOR_PEACH_PUFF, WYSOKOSC_CZCIONEK_PRZYCISKI,KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn btn_zbiorowe_rozszerzenia<'a>(lell:OptRozszerzeniaPlikówZdjęciowychZnacznik, co_istnieje:&DaneDoBathKonwersjaZdjec, font:iced::Font) -> Element<'a, Message>{

    let nazwa = match lell {
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg => "Jpg",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Png => "Png",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp => "Webp",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga => "Tga",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff => "FF",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi => "Qoi",
    };
    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjecieEdycjaZmianaWybraneToggleRozszerzenie(lell.clone()),
        ))
        .style(styl_przycisków(
            false,
            co_istnieje.tag.contains(&lell),
            KOLOR_SPANISH_ORANGE,
        )).into()
}
pub fn btn_zbiorowe_kolor_ogolny<'a>(rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,lell:OptFormatyKoloruObrazOgólny, co_istnieje:&DaneDoBathKonwersjaZdjec, font:iced::Font) -> Element<'a, Message>{
// OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny
    let nazwa = match lell {
        OptFormatyKoloruObrazOgólny::L8 => "L 8bit",
        OptFormatyKoloruObrazOgólny::L8a => "LA 8bit",
        OptFormatyKoloruObrazOgólny::B8 => "RGB 8bit",
        OptFormatyKoloruObrazOgólny::B8a => "RGBA 8bit",
        OptFormatyKoloruObrazOgólny::L16 => "L 16bit",
        OptFormatyKoloruObrazOgólny::L16a => "LA 16bit",
        OptFormatyKoloruObrazOgólny::B16 => "RGB 16bit",
        OptFormatyKoloruObrazOgólny::B16a => "RGBA 16bit",
        OptFormatyKoloruObrazOgólny::B32 => "RGB 32bit",
        OptFormatyKoloruObrazOgólny::B32a => "RGBA 32bit",
    };
    let czy_wybrany = co_istnieje.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
        match (format, &rozs) {
            (OptRozszerzeniaPlikówZdjęciowych::Jpg { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg) => bit_depth.contains(&lell),
            (OptRozszerzeniaPlikówZdjęciowych::Png { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Png) => bit_depth.contains(&lell),
            (OptRozszerzeniaPlikówZdjęciowych::Webp { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp) => bit_depth.contains(&lell),
            // TGA i QOI mają własne enumy kolorów, więc tutaj pewnie nie będą trafiać,
            _ => false,
        }
    });
    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjecieEdycjaZmianaWybraneToggleKolor(rozs,lell.clone()),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            KOLOR_SPANISH_ORANGE,
        )).into()
}
pub fn btn_zbiorowe_kolor_tga<'a>(rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,lell:OptFormatyKoloruObrazuTga, co_istnieje:&DaneDoBathKonwersjaZdjec, font:iced::Font) -> Element<'a, Message>{
    // OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny
    let nazwa = match lell {
        OptFormatyKoloruObrazuTga::Szary8 => {"G8"}
        OptFormatyKoloruObrazuTga::HighColor16 => {"HC16"}
        OptFormatyKoloruObrazuTga::TrueColor24 => {"TC24"}
        OptFormatyKoloruObrazuTga::TrueColorA32 => {"TC32"}
    };
    let czy_wybrany = co_istnieje.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
        match (format, &rozs) {
            (OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga) => bit_depth.contains(&lell),
            _ => false,
        }
    });
    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaBitDepthTga(lell.clone()),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            KOLOR_SPANISH_ORANGE,
        )).into()
}
pub fn btn_zbiorowe_kolor_qoi<'a>(rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,lell:OptFormatyKoloruObrazuQoi, co_istnieje:&DaneDoBathKonwersjaZdjec, font:iced::Font) -> Element<'a, Message>{
    // OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny
    let nazwa = match lell {
        OptFormatyKoloruObrazuQoi::Color24 => {"Color24"}
        OptFormatyKoloruObrazuQoi::ColorA32 => {"Color32"}
    };
    let czy_wybrany = co_istnieje.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
        match (format, &rozs) {
            (OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi) => bit_depth.contains(&lell),

            _ => false,
        }
    });
    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaBitDepthQoi(lell.clone()),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            KOLOR_SPANISH_ORANGE,
        )).into()
}

pub fn info_male<'a>(nazwa:String,warunek:bool) -> Element<'a, Message> {
    text(nazwa)
        .font(iced::Font {
            family: iced::font::Family::Name("VT323"),
            ..Default::default()
        })
        .color(Color::from_rgba(
            1.,
            1.,
            1.,
            if warunek { 0.5 } else { 0.2 },
        ))
        .size(ROZMIARWYBRANYCHROZSZERZEN).into()
}

pub fn btn_rozdzielczosci<'a>(dane:&DaneDoBathKonwersjaZdjec, rozdzielczosc: OptRozdzielczościObrazów, font:iced::Font) -> Element<'a, Message> {
    let nazwa = match rozdzielczosc {
        OptRozdzielczościObrazów::R16 => {"16px"}
        OptRozdzielczościObrazów::R32 => {"32px"}
        OptRozdzielczościObrazów::R64 => {"64px"}
        OptRozdzielczościObrazów::R128 => {"128px"}
        OptRozdzielczościObrazów::R256 => {"256px"}
        OptRozdzielczościObrazów::R512 => {"512px"}
        OptRozdzielczościObrazów::R1k => {"1.024px"}
        OptRozdzielczościObrazów::R2k => {"2.048px"}
        OptRozdzielczościObrazów::R4k => {"4.096px"}
        OptRozdzielczościObrazów::R6k => {"6.144px"}
        OptRozdzielczościObrazów::R8k => {"8.192px"}
        OptRozdzielczościObrazów::R16k => {"16.384px"}
        OptRozdzielczościObrazów::Oryginalna => {"Org"}
    };
    

    button(
        text(
            nazwa)
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::DopasujRozdzielczosci(rozdzielczosc.clone()),
        ))
        .style(styl_przycisków(
            false,
            dane.opcje_rozdzielczości.contains(&rozdzielczosc), //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            KOLOR_SPANISH_ORANGE,
        )).into()
}