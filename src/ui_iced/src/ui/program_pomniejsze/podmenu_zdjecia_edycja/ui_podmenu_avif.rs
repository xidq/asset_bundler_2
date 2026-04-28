use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{AvifChroma, AvifMetodaKompresji, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row, button, container, slider, space, text, tooltip, pick_list};
use iced::{Color, Length};
use enumy::inne_ui::{CheckerDoZbiorowePrzetwarzanieZdjęć, RodzajeContainer};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_avif, btn_zbiorowe_kolor_ogolny, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn podmenu_avif_wybor<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'a, Message> {
    let opcje:Vec<AvifMetodaKompresji> = vec![
        AvifMetodaKompresji::Av1,
        AvifMetodaKompresji::HtJ2k,
        AvifMetodaKompresji::Mask,
        AvifMetodaKompresji::Uncompressed,
        AvifMetodaKompresji::Jpeg2000,
        AvifMetodaKompresji::Evc,
        AvifMetodaKompresji::Vvc,
        AvifMetodaKompresji::Jpeg,
        AvifMetodaKompresji::Avc,
        AvifMetodaKompresji::Hevc,
        AvifMetodaKompresji::Undefined,
    ];

    let opcje2:Vec<AvifChroma> = vec![
        AvifChroma::C420,
        AvifChroma::C422,
        AvifChroma::C444,
    ];
     Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif, dane, jezyk.get_font()))

        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif {
                            chroma,
                            speed,
                            metoda_kompresji, 
                            lossy,
                            bit_depth,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. }))
                {
                container(
                    Column::new()
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=10,
                                    *speed,
                                    |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifSpeed(vv))
                                )
                                    .height(20.)
                                    .width(Length::FillPortion(6))
                                .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
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
                                        |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifLossy(vv))
                                    )
                                        .height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
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
                        Row::new()

                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_avif(OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif,OptFormatyKoloruObrazuAvif::B8,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_avif(OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif,OptFormatyKoloruObrazuAvif::B8a,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_avif(OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif,OptFormatyKoloruObrazuAvif::B10,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_avif(OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif,OptFormatyKoloruObrazuAvif::B10a,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(

                            tooltip(
                                button(
                                    text("lossless.")
                                        .font(jezyk.get_font())
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .center(),
                                )
                                    .padding(10)
                                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifToggleLossy))
                                    .style(styl_przycisków(
                                        false,
                                        lossy.is_none(),
                                        KOLOR_SPANISH_ORANGE,
                                    ))
                                    .width(Length::FillPortion(1)),

                                "Zapis progresywny on/off",
                                tooltip::Position::Top,
                            )
                        ).height(Length::Fixed(50.))
                    )

                    .push(
                        Row::new()
                            .push(
                                pick_list(opcje, Some(metoda_kompresji.clone()), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifKompresja(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                                    .menu_style(styl_menu_pick(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                                    .width(Length::FillPortion(4))
                            )
                            .push(
                                pick_list(opcje2, Some(chroma.clone()), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaAvifChroma(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                                    .menu_style(styl_menu_pick(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                                    .width(Length::FillPortion(4))
                            ).padding(15)
                    )
                ).height(200.).style(styl_kontenera(true, KOLOR_SPANISH_ORANGE,RodzajeContainer::Góra))
            } else {
                    container(Row::new())
            }
        )
        // MENU Z WYBORAMI
        // DRUGI ROW
        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_avif_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
) -> Column<'a, Message> {
    let avif_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Avif { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazuAvif| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Avif { bit_depth, .. }) = avif_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };


    // 3. Przygotowanie jakości
    let jakosc_str = avif_data
        .and_then(|f| {
            if let OptRozszerzeniaPlikówZdjęciowych::Avif { lossy, .. } = f {
                // Mapujemy u8 na String, a jeśli lossy to None -> dany "lelele"
                Some(lossy.map(|v| v.to_string()).unwrap_or_else(|| "Lossless".to_string()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| "-".to_string());

    let jest_aktywny_avif = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Avif);
    let kompresja = match avif_data {
        Some(OptRozszerzeniaPlikówZdjęciowych::Avif { metoda_kompresji, .. }) => metoda_kompresji.to_string(),
        _ => "---".to_string(),
    };
    let chrum = match avif_data {
        Some(OptRozszerzeniaPlikówZdjęciowych::Avif {chroma, .. }) => chroma.to_string(),
        _ => "---".to_string(),
    };

    Column::new()
        /*tekst*/.push(info_male("Avif".to_string(),jest_aktywny_avif))
        .push(
            Row::new()

                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
                .push(info_male("|".to_string(),false))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazuAvif::B8)))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male("8a".to_string(),ma_kolor(OptFormatyKoloruObrazuAvif::B8a)))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male("10".to_string(),ma_kolor(OptFormatyKoloruObrazuAvif::B10)))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male("10a".to_string(),ma_kolor(OptFormatyKoloruObrazuAvif::B10a)))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
                .push(info_male("|".to_string(),false))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male(jakosc_str,jest_aktywny_avif))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
                .push(info_male("|".to_string(),false))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male(kompresja,jest_aktywny_avif))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
                .push(info_male("|".to_string(),false))
                .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        /*tekst*/.push(info_male(chrum,jest_aktywny_avif))
        )
}
