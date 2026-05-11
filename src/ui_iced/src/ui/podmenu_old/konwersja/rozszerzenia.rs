use iced::widget::{container, scrollable, slider, space, text, Column, Row};
use iced_core::{Border, Color, Length};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::UstawieniaThemeWsio;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::slidery::{ slajdery_fn};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::style_fn::scroll::styl_scrollable;
use crate::ui::podmenu::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_avif::podmenu_avif_wybor;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_ff::podmenu_ff_wybor;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_jpg::podmenu_jpg_wybor_top;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_png::podmenu_png_wybor;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_qoi::podmenu_qoi_wybor;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_tga::podmenu_tga_wybor;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_webp::podmenu_webp_wybor;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn rozszerzenia<'a>(dane:&'a DaneDoBathKonwersjaZdjec, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a,Message>{
    Column::new()
        .push(
            slajdery_fn(dane, jezyk)
        )
        .push(ui_standard_oddzielacz())
        .width(Length::FillPortion(5))
        .push(scrollable(
            Column::new()

                .push(podmenu_avif_wybor(dane, jezyk, &kolor, temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_jpg_wybor_top(dane, jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_png_wybor(dane,  jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_webp_wybor(dane,  jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_tga_wybor(dane, jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_ff_wybor(dane, jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                .push(podmenu_qoi_wybor( dane, jezyk,&kolor,temat))
                .push(ui_standard_oddzielacz())
                // .push(Row::new().push(text("WIP")))
                .spacing(15) //oesu ale to długie... a tyle krwi napsuło...
                .padding(15)
                .width(Length::FillPortion(2)),
        ).style(styl_scrollable(&temat.kolory.konwersja,temat)))
        .spacing(15) //tu sie kończy kolumna.....................................................
        .padding(15)
        .width(Length::FillPortion(2))
}