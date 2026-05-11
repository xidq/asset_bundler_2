use crate::ui::podmenu::kolory::{KOLOR_CZCIONKI_SREDNI, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::progress_bar::styl_progress_bar;
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::{DaneDoPakowaniaDds, DaneDoRozpakowaniaDds};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
use enumy::ikony::folder_icon;
pub(crate) use enumy::inne_ui::StronyDds;
use enumy::inne_ui::{ActProces, RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, progress_bar, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::kompresje::{OptFormatDds, OptKompresjaDds, OptMetodaKompresjiZdjecia};
use enumy::rozszerzenia::bdepth::{BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::kolor::{JpgQuant, JpgSamplingFac};
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::dds::pakowanie_main::dds_pakowanie_main;
use crate::ui::podmenu::dds::rozpakowanie_main::dds_rozpakowanie_main;

pub fn view_dds<'a>(
    dane_pakowanie: std::sync::Arc<&'a DaneDoPakowaniaDds>,
    dane_rozpakowanie: std::sync::Arc<&'a DaneDoRozpakowaniaDds>,
    wybrane_okno: &'a StronyDds,
    jezyk: &'a WybórJęzyka,
    log_pakowanie: &'a LogPakowaniaDds,
    log_rozpakowywanie: &'a LogRozpakowywanieDds,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    let valid =  temat.temp.aktywny_proces == ActProces::Żodyn;

    // let print_sciezki = |xx:&PathBuf| -> String{
    //     let strinkku=|| -> String {
    //         let yy = xx.to_str().unwrap_or("");
    //         let char_count = yy.chars().count();
    //
    //         if char_count <= 20 {
    //             yy.to_string()
    //         } else {
    //             let start = yy.char_indices().nth(15).map(|(i, _)| i).unwrap_or(yy.len());
    //             let fin = yy.char_indices().nth_back(9).map(|(i, _)| i).unwrap_or(0);
    //             yy[0..=start].to_string() + "..." + &yy[fin ..]
    //         }
    //     };
    //
    //     if xx.exists() && xx.is_file(){
    //         "[Plik]  ".to_string() + &strinkku()
    //     } else{
    //         "[Folder]  ".to_string() + &strinkku()
    //     }
    // };
    // fn print_sciezki(xx: &Path) -> String {
    //     let starrrrtuuuuuu = xx.components().nth(1).unwrap().as_os_str().to_string_lossy().to_string();
    //     let endo = xx.file_name().unwrap().to_string_lossy().to_string();
    // 
    //     let poczatek = if xx.to_string_lossy().len() < 30 {
    //         xx.to_string_lossy().to_string()
    //     } else if starrrrtuuuuuu.len() < 30 {
    //         let secnd = xx.components().nth(2).unwrap().as_os_str().to_string_lossy().to_string();
    //         starrrrtuuuuuu + "/" + &secnd + "/"
    //     } else {
    //         starrrrtuuuuuu + "/"
    //     };
    // 
    //     if xx.to_string_lossy().len() < 30 {
    //         poczatek
    //     } else {
    //         poczatek + " ... " + "/" + endo.as_str()
    //     }
    // }
    // 
    // fn tekst_sciezek<'a>(
    //     sciezki: &'a Option<Vec<PathBuf>>,
    //     yy: usize,
    //     jezyk: &'a WybórJęzyka,
    //     f_formatuj: &dyn Fn(&Path) -> String // przekazujemy logikę formatowania
    // ) -> Element<'a, Message> {
    //     text(match sciezki {
    //         Some(xx) if yy < xx.len() => f_formatuj(&xx[yy]),
    //         _ => "".to_string(),
    //     })
    //         .color(KOLOR_CZCIONKI_SREDNI)
    //         .font(jezyk.get_font())
    //         .height(20.)
    //         .into() // Ważne: rzutowanie na Element<'a>
    // }




    let lewa = Column::new()
        // .push(space().height(Length::FillPortion(10)))
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZplikuDoDds)))
            .style(styl_przycisków(
                temat.temp.aktywny_proces == ActProces::DdsPakowanie,
                matches!(wybrane_okno, StronyDds::ZplikuDoDds),
                &temat.kolory.dds, temat
            )),
        )
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZddsDoPliku)))
            .style(styl_przycisków(
                temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
                matches!(wybrane_okno, StronyDds::ZddsDoPliku),
                &temat.kolory.dds, temat
            )),
        )
        // .push(space().height(Length::FillPortion(10)))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));



    let prawa = match wybrane_okno {
        StronyDds::ZddsDoPliku => dds_rozpakowanie_main(dane_rozpakowanie,log_rozpakowywanie, &temat.kolory.dds, jezyk, temat),
        StronyDds::ZplikuDoDds => dds_pakowanie_main(dane_pakowanie,log_pakowanie, &temat.kolory.dds, jezyk,temat),
    };

    Row::new().push(lewa).push(prawa).into()
}

