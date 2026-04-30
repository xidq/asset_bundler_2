use crate::ui::program_pomniejsze::kolory::WYSOKOSC_CZCIONEK_PRZYCISKI;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::ROZMIARWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use iced::widget::{button, text};
use iced::Element;
use iced_core::{Color, Length};

pub fn btn_zbiorowe_rozszerzenia<'a>(
    lell:OptRozszerzeniaPlikówZdjęciowychZnacznik, 
    co_istnieje:&DaneDoBathKonwersjaZdjec, 
    font:iced::Font, 
    kolor:&'a Color, 
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{

    let nazwa = match lell {
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg => "Jpg",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Png => "Png",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp => "Webp",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga => "Tga",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff => "FF",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi => "Qoi",
        OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif => "Avif",
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
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozszerzenia(lell.clone()),
        ))
        .style(styl_przycisków(
            false,
            co_istnieje.tag.contains(&lell),
            kolor,
            temat,
        )).into()
}
pub fn btn_zbiorowe_kolor_ogolny<'a>(
    rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,
    lell:OptFormatyKoloruObrazOgólny, 
    co_istnieje:&DaneDoBathKonwersjaZdjec, 
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
// OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny

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
            lell.mid().to_string())
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Bdepth(rozs, lell),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany,
            kolor,
            temat
        )).into()
}
pub fn btn_zbiorowe_kolor_avif<'a>(
    rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,
    lell:OptFormatyKoloruObrazuAvif, 
    co_istnieje:&DaneDoBathKonwersjaZdjec, 
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
    // OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny

    let czy_wybrany = co_istnieje.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
        match (format, &rozs) {

            (OptRozszerzeniaPlikówZdjęciowych::Avif { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif) => bit_depth.contains(&lell),
            // TGA i QOI mają własne enumy kolorów, więc tutaj pewnie nie będą trafiać,
            _ => false,
        }
    });
    button(
        text(
            lell.mid().to_string())
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifBdepth(lell),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            kolor,
            temat
        )).into()
}
pub fn btn_zbiorowe_kolor_tga<'a>(
    rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,
    lell:OptFormatyKoloruObrazuTga, 
    co_istnieje:&DaneDoBathKonwersjaZdjec, 
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
    // OptRozszerzeniaPlikówZdjęciowychZnacznik zawiera tylko enumy z mozliwymi rozszerzeniami, OptRozszerzeniaPlikówZdjęciowych który jest w co_istnieje zawiera enumy z polami i vec dla OptFormatyKoloruObrazOgólny

    let czy_wybrany = co_istnieje.rozszerzenia_plików_zdjęciowych.iter().any(|format| {
        match (format, &rozs) {
            (OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth, .. }, OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga) => bit_depth.contains(&lell),
            _ => false,
        }
    });
    button(
        text(
            lell.mid().to_string())
            .color(Color::from_rgba(1., 1., 1., 0.6))
            .font(font)
            .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
            .width(Length::Fill)
            .center(),
    )
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::TgaBdepth(lell),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            kolor,
            temat
        )).into()
}
pub fn btn_zbiorowe_kolor_qoi<'a>(
    rozs: OptRozszerzeniaPlikówZdjęciowychZnacznik,
    lell:OptFormatyKoloruObrazuQoi, 
    co_istnieje:&DaneDoBathKonwersjaZdjec,
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message>{
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
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::QoiBdepth(lell),
        ))
        .style(styl_przycisków(
            false,
            czy_wybrany, //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            kolor,
            temat
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

pub fn btn_rozdzielczosci<'a>(
    dane:&DaneDoBathKonwersjaZdjec, 
    rozdzielczosc: OptRozdzielczościObrazów, 
    font:iced::Font,
    kolor:&'a Color,
    temat:&'a UstawieniaThemeWsio
) -> Element<'a, Message> {
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
        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::Rozdzielczość(rozdzielczosc.clone()),
        ))
        .style(styl_przycisków(
            false,
            dane.opcje_rozdzielczości.contains(&rozdzielczosc), //jeżeli istnieje rozszerzenie, a w nim dany wariant koloru to true
            kolor,
            temat
        )).into()
}