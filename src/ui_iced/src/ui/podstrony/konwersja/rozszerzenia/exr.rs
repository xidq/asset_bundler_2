use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_konwersja, pole_tekstowe_przycisku, przycisk_rozszerzenia};
use crate::widget::dropdown::dropdown;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::BdepthExr;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::kompresje::ForExrKompresja;
use enumy::rozszerzenia::rozszenienia_zdjec::ImgExtExr;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::Color;
use strum::IntoEnumIterator;

pub fn exr<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Exr, ButtonType::KonwRozszerzenia, kolor, if dane.rozszerzenia.exr.is_some(){ &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExtExr {
                                bit_depth,
                                kompresja:_
                            }) = &dane.rozszerzenia.exr
                        {
                            Column::new()
                                .push(
                                    // match kompresja{
                                        // ForExrKompresja::Dwaa(Some(pp)) => {
                                        //     Row::new().spacing(15).height(50.)
                                        //         .push(pole_tekstowe_przycisku(format!("Q: {}", pp), jezyk, temat))
                                        //         .push(slajderr(*pp as i32, (0,100), &SliderType::KonwExrCompDwaa, kolor, temat, Length::FillPortion(2)))
                                        //         .push(space().width(15.))
                                        // }
                                        // ForExrKompresja::Dwab(Some(pp)) => {
                                        //     Row::new().spacing(15).height(50.)
                                        //         .push(pole_tekstowe_przycisku(format!("Q: {}", pp), jezyk, temat))
                                        //         .push(slajderr(*pp as i32, (0,100), &SliderType::KonwExrCompDwab, kolor, temat, Length::FillPortion(2)))
                                        //         .push(space().width(15.))
                                        // }
                                        // Wszystkie warianty z None oraz każdy inny rodzaj kompresji wpadną tutaj:
                                        // _ =>
                                            Row::new().height(50.)
                                    // }
                                )
                                .push(
                                    BdepthExr::iter()
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
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_jpg_sampling", jezyk, temat))
                                        .push(dropdown::<ForExrKompresja, _>(dane, kolor, temat))
                                        .push(space().width(15.))
                                )

                                .push(space().height(25.))
                        } else {
                            Column::new().push(space())
                        }
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Oba, kolor, temat))
        ).into()
}