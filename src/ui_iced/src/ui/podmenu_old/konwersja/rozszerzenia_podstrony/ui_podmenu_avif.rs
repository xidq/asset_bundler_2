use std::sync::Arc;
use crate::ui::podmenu::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{BtnState, RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, slider, space, text, tooltip, Column, Row};
use iced::Length;
use strum::{EnumMessage, IntoEnumIterator};
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::kolor::AvifChroma;
use enumy::rozszerzenia::kompresje::AvifMetodaKompresji;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::fn_ogolne::info_male;
use crate::ui::podmenu::metody_do_wariantow::bit_depth_konwersja::btn_bdepth_avif_konwersja;
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;

pub fn podmenu_avif_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {

     Column::new()
        .push(rozszerzenie_konwersja(RozszerzeniaZnacznik::Avif, if dane.tag.contains(&RozszerzeniaZnacznik::Avif){BtnState::Active} else {BtnState::Disabled}, kolor, jezyk.get_font(), temat))

        .push(
            if let Some(
                    Rozszerzenia::Avif {
                            chroma,
                            speed,
                            metoda_kompresji, 
                            lossy,
                            bit_depth,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Avif { .. }))
                {
                container(
                    Column::new()
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=10,
                                    *speed,
                                    |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifSpeed(vv))
                                )
                                    .height(20.)
                                    .width(Length::FillPortion(6))
                                .style(styl_sliderów(kolor,temat)),
                            )
                            .push(
                                text(format!(
                                    "S: {}",
                                    speed
                                ))
                                .color(KOLOR_CZCIONKI_SREDNI)
                                .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                            )
                            .padding(15)
                    )
                    .push(
                        if lossy.is_some() {
                            Row::new()
                                .push(
                                    slider(
                                        0..=100,
                                        lossy.unwrap_or(90),
                                        |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifLossy(vv))
                                    )
                                        .height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów(kolor,temat)),
                                )
                                .push(
                                    text(format!(
                                        "Q: {}%",
                                        lossy.unwrap_or(90)
                                    ))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                                )
                                .padding(15).spacing(10).height(Length::Fixed(50.))
                        }else{
                            Row::new()
                                .push(space().height(Length::Fixed(50.)))
                        }

                    )
                    .push(

                        BdepthAvif::iter()
                            .into_iter()
                            .fold(Row::new(), |row, wariant| {
                                row.push(
                                    btn_bdepth_avif_konwersja(
                                        wariant,
                                        bit_depth.contains(&wariant),
                                        jezyk.get_font(),
                                        kolor,
                                        temat
                                    ),
                                )
                            }
                            )

                        .push(

                            hint_btn(
                                button(
                                    text("lossless.")
                                        .font(jezyk.get_font())
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .center(),
                                )
                                    .padding(10)
                                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifLossyToggle))
                                    .style(styl_przycisków(
                                        if lossy.is_none() { &BtnState::Disabled } else { &BtnState::Active },
                                        kolor,
                                        temat
                                    ))
                                    .width(Length::FillPortion(1)),

                                "Zapis progresywny on/off",
                                temat
                            )
                        ).height(Length::Fixed(50.))
                    )

                    .push(
                        Row::new()
                            .push(
                                pick_list(AvifMetodaKompresji::iter().collect::<Vec<_>>(), Some(metoda_kompresji.clone()), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifKompresja(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(kolor,temat))
                                    .menu_style(styl_menu_pick(kolor,temat))
                                    .width(Length::FillPortion(4))
                            )
                            .push(
                                pick_list(AvifChroma::iter().collect::<Vec<_>>(), Some(chroma.clone()), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::AvifChroma(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(kolor,temat))
                                    .menu_style(styl_menu_pick(kolor,temat))
                                    .width(Length::FillPortion(4))
                            ).padding(15)
                    )
                ).height(200.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
            } else {
                    container(Row::new())
            }
        )

        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_avif_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    
    let avif_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, Rozszerzenia::Avif { .. })
    });



    let jakosc_str = avif_data
        .and_then(|f| {
            if let Rozszerzenia::Avif { lossy, .. } = f {
                // Mapujemy u8 na String, a jeśli lossy to None -> dany "lelele"
                Some(lossy.map(|v| v.to_string()).unwrap_or_else(|| "Lossless".to_string()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| "-".to_string());

    let kompresja = match avif_data {
        Some(Rozszerzenia::Avif { metoda_kompresji, .. }) => metoda_kompresji.to_string(),
        _ => "---".to_string(),
    };
    let chrum = match avif_data {
        Some(Rozszerzenia::Avif {chroma, .. }) => chroma.to_string(),
        _ => "---".to_string(),
    };



    // let aktywne_bdepthy_avif = dane.rozszerzenia_plików_zdjęciowych.iter().find_map(|f| {
    //     if let Rozszerzenia::Avif { bit_depth, .. } = f {
    //         Some(bit_depth)
    //     } else {
    //         None
    //     }
    // });
    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat));

    for wariant in BdepthAvif::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat))
        .push(info_male(jakosc_str, RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat))
        .push(info_male("|".to_string(), RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat))
        .push(info_male(kompresja, RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat))
        .push(info_male("|".to_string(), RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat))
        .push(info_male(chrum, RozszerzeniaZnacznik::Avif.bath_konwersja_id(),temat));

    Column::new()
        /*tekst*/.push(info_male("Avif".to_string(),"btn_id_batch_avif_ext",temat))
        .push(row_rozszerzen)

}
