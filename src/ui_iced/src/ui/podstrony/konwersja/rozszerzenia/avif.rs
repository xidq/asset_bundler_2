use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_konwersja, pole_tekstowe_przycisku, przycisk, przycisk_rozszerzenia};
use crate::widget::dropdown::dropdown;
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::kolor::ForAvifChroma;
use enumy::rozszerzenia::kompresje::ForAvifKompresja;
use enumy::rozszerzenia::rozszenienia_zdjec::ImgExtAvif;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn avif<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Avif, ButtonType::KonwRozszerzenia, kolor, if dane.rozszerzenia.avif.is_some() { &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()

                    .push(
                        if let Some(
                            ImgExtAvif {
                                chroma: _,
                                speed,
                                metoda_kompresji: _,
                                lossy,
                                bit_depth
                            }) = &dane.rozszerzenia.avif
                        {
                            Column::new()
                                .push(
                                    Row::new().spacing(15).height(50.)
                                        .push(pole_tekstowe_przycisku(format!("S: {}", speed), jezyk, temat))
                                        .push(slajderr(*speed, (0,100), &SliderType::KonwersjaAvifSpeed, kolor, temat, Length::FillPortion(2)  ))
                                        .push(space().width(15.))
                                )
                                .push(
                                    BdepthAvif::iter()
                                        
                                        .fold(
                                            Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_konwersja(
                                                        wariant,
                                                        if bit_depth.contains(&wariant) { &BtnState::Active } else { &BtnState::Disabled },
                                                        jezyk,
                                                        kolor,
                                                        temat
                                                    ),
                                                )
                                            }
                                        )
                                )
                                .push(przycisk("conversion_avif_lossless", ButtonType::KonwAvifLoss,Length::FillPortion(1), Length::Fixed(50.),kolor,if lossy.is_none() { &BtnState::Processing } else { &BtnState::Disabled },jezyk,temat))
                                .push(
                                    match lossy {
                                        Some(ggg) => {
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("Q: {}", ggg), jezyk, temat))
                                                .push(slajderr(*ggg, (0, 100), &SliderType::KonwersjaAvifQuality, kolor, temat, Length::FillPortion(2)))
                                                .push(space().width(15.))
                                        }
                                        None => { Row::new().spacing(15).height(50.) }
                                    }
                                )
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_avif_chroma", jezyk, temat))
                                        .push(dropdown::<ForAvifChroma, _>(dane, kolor, temat))
                                        .push(space().width(15.))
                                )
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_avif_compression", jezyk, temat))
                                        .push(dropdown::<ForAvifKompresja, _>(dane, kolor, temat))
                                        .push(space().width(15.))
                                )
                                .push(space().height(25.))
                        } else{Column::new().push(space())}
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Oba, kolor, temat))
        )
        .into()
}