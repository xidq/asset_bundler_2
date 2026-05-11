use std::sync::Arc;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::fn_ogolne::{ info_male};
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, slider, space, text, Column, Row};
use iced_core::Length;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthPng};
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::{btn_bdepth_avif_konwersja, btn_bdepth_png_konwersja};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;

pub fn podmenu_png_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Png, kolor, jezyk.get_font(), temat))
        .push(
            if let Some(Rozszerzenia::Png {
                            kompresja,
                            bit_depth:_,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Png { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=9,
                                        *kompresja,
                                        |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PngKompresja(xx)),
                                    ).height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów(kolor,temat)),
                                )
                                .push(
                                    text(format!(
                                        "C: {}",
                                        kompresja
                                    ))
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .height(Length::Fill)
                                        .width(Length::FillPortion(4))
                                        .center(),
                                ).padding(15)
                        )
                        .push(
                            BdepthPng::iter()
                                .step_by(2)
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_png_konwersja(
                                            wariant,
                                            wariant.bath_konwersja_id(),
                                            jezyk.get_font(),
                                            kolor,
                                            temat
                                        ),
                                    )
                                })
                        )
                        .push(
                            BdepthPng::iter()
                                .skip(1)
                                .step_by(2)
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_png_konwersja(
                                            wariant,
                                            wariant.bath_konwersja_id(),
                                            jezyk.get_font(),
                                            kolor,
                                            temat
                                        ),
                                    )
                                })
                        )
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat,))
            } else {
                container(Column::new())
            }
            .width(Length::FillPortion(5)),
        )
        //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_png_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {

    let png_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, Rozszerzenia::Png { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: BdepthPng| {
        if let Some(Rozszerzenia::Png { bit_depth, .. }) = png_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    // 3. Przygotowanie jakości
    let kompresja_str = png_data
        .and_then(|f| if let Rozszerzenia::Png { kompresja, .. } = f { Some(kompresja.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());

    let jest_aktywny_png = dane.tag.contains(&RozszerzeniaZnacznik::Png);



    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_png_ext",temat));

    for wariant in BdepthPng::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_png_ext",temat))
        .push(info_male(kompresja_str, "btn_id_batch_png_ext",temat));

    Column::new()
        /*tekst*/.push(info_male("Png".to_string(),"btn_id_batch_png_ext",temat))
        .push(row_rozszerzen)

}
