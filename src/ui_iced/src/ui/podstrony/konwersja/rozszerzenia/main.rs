use crate::ui::podstrony::konwersja::rozszerzenia::qoi::qoi;
use crate::ui::podstrony::konwersja::rozszerzenia::{
    avif::avif,
    ff::ff,
    jpg::jpg,
    png::png,
    tga::tga,
    webp::webp
};
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::oddzielacze::oddzielacz_poziomy;
use crate::widget::styles::styl_scrollable;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{scrollable, Column};
use iced_core::Color;

pub fn rozszerzenia<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message>{
    Column::new()
        .push(
            scrollable(
                Column::new().spacing(12.5).padding(15)
                    .push(jpg(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(avif(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(png(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(webp(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(tga(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(ff(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
                    .push(qoi(dane, kolor, jezyk, temat))
                    .push(oddzielacz_poziomy())
            ).style(styl_scrollable(kolor, temat))
        )
}