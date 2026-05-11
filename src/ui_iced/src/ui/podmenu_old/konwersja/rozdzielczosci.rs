use iced::widget::{scrollable, Column, Row};
use iced_core::Length;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::UstawieniaThemeWsio;
use enumy::opcje::OptInterpolacja;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::ui_standard::rozdzielczosci_btn::btn_rozdzielczosci;
use crate::ui::wiadomosci::message_ui::Message;

pub fn rozdzielczosci<'a>(dane: &'a DaneDoBathKonwersjaZdjec, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a,Message>  {
    // let opcje_interpolacja: Vec<String> = Rozdzielczości::ROZDZIELCZOSCI
    //     .iter()
    //     .map(|p| jezyk.t(p.klucz()).to_string())
    //     .collect();
    Column::new()
        .push(scrollable(
            Column::new()
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(Rozdzielczości::R16, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R32, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R64, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R128, jezyk, &temat.kolory.konwersja, temat)).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(Rozdzielczości::R256, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R512, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R1k, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R2k, jezyk, &temat.kolory.konwersja, temat)).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(Rozdzielczości::R4k, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R6k, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R8k, jezyk, &temat.kolory.konwersja, temat))
                        .push(btn_rozdzielczosci(Rozdzielczości::R16k, jezyk, &temat.kolory.konwersja, temat)).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(Rozdzielczości::Oryginalna, jezyk, &temat.kolory.konwersja, temat)).spacing(10)
                )
                .spacing(10),
        ))
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2))
}