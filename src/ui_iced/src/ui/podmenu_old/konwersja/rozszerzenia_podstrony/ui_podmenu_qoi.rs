use std::sync::Arc;
use crate::ui::podmenu::fn_ogolne::{info_male};
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced_core::Length;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthPng, BdepthQoi};
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::{btn_bdepth_png_konwersja, btn_bdepth_qoi_konwersja};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;

pub fn podmenu_qoi_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Qoi, kolor, jezyk.get_font(), temat))

        .push(
            if let Some(Rozszerzenia::Qoi {
                            bit_depth:_,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Qoi { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(space()).height(Length::FillPortion(1))
                        )
                        .push(
                            BdepthQoi::iter()
                                .skip(1)
                                .step_by(2)
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_qoi_konwersja(
                                            wariant,
                                            wariant.bath_konwersja_id(),
                                            jezyk.get_font(),
                                            kolor,
                                            temat
                                        ),
                                    )
                                })
                        )
                    
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra, kolor,temat))
            }else{container(Column::new())}
        ).padding(15).width(Length::FillPortion(2))
    
}

pub fn podmenu_qoi_misc<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a,Message> {

    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_qoi_ext",temat));

    for wariant in BdepthQoi::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }
    

    Column::new()
        /*tekst*/.push(info_male("Qoi".to_string(),"btn_id_batch_qoi_ext",temat))
        .push(row_rozszerzen)
}
