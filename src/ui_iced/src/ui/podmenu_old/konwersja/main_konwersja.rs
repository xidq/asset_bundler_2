use crate::ui::podmenu::konwersja::inne::inne;
use crate::ui::podmenu::konwersja::rozdzielczosci::rozdzielczosci;
use crate::ui::podmenu::konwersja::rozszerzenia::rozszerzenia;
use crate::ui::podmenu::konwersja::sciezki::sciezki;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_avif::podmenu_avif_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_ff::podmenu_ff_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::ui_podmenu_qoi::podmenu_qoi_misc;
use crate::ui::podmenu::konwersja::rozszerzenia_podstrony::{
    ui_podmenu_jpg::podmenu_jpg_misc,
    ui_podmenu_lewe::{podmenu_lewe_rozdzielczosci, podmenu_lewe_wybor},
    ui_podmenu_png::podmenu_png_misc,
    ui_podmenu_tga::podmenu_tga_misc,
    ui_podmenu_webp::podmenu_webp_misc,
};
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::progress_bar::styl_progress_bar;
use crate::ui::program::{LogPrzetwarzanieFot, WybórJęzyka};
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
pub(crate) use enumy::inne_ui::WybraneOknoEdycjiZdjęć;
use enumy::inne_ui::{ActProces, UiPodstrony, UstawieniaThemeWsio};
use iced::widget::{button, progress_bar, space, text, Column, Row};
use iced::{Color, Element, Length};
use crate::ui::podmenu::konwersja::lewa::lewa_kolumna;

pub(crate) const ROZMIARWYBRANYCHROZSZERZEN: iced::Pixels = iced::Pixels(14.);
pub(crate) const PRZERWAWYBRANYCHROZSZERZEN: f32 = 3.;



pub fn menu_konwersja<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    czy_wyjscie_te_same: &'a bool,
    log: &'a LogPrzetwarzanieFot,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    
    let prawa_kolumna = match temat.temp.aktywne_okno {
        UiPodstrony::KonwersjaFotoŚcieżki => sciezki(dane, czy_wyjscie_te_same, &temat.kolory.konwersja, jezyk, temat),
        UiPodstrony::KonwersjaFotoRozszerzenia => {
            rozszerzenia(dane, &temat.kolory.konwersja, jezyk, temat)
        }
        UiPodstrony::KonwersjaFotoRozdzielczości => rozdzielczosci(dane, jezyk,temat),
        UiPodstrony::KonwersjaFotoMenuReszta => inne(dane, jezyk,temat),
        _ => {Column::new()}
    };

    Row::new().push(lewa_kolumna(dane, log, jezyk,temat)).push(prawa_kolumna).into()
}
