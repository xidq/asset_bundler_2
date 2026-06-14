use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_konwersja, pole_tekstowe_przycisku, przycisk_rozszerzenia};
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::BdepthPng;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::rozszenienia_zdjec::ImgExtPng;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn png<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Png, ButtonType::KonwRozszerzenia, kolor, if dane.rozszerzenia.png.is_some() { &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExtPng {
                                kompresja, bit_depth
                            }) = &dane.rozszerzenia.png
                        {
                            Column::new()
                                .push(
                                    Row::new().spacing(15).height(50.)
                                        .push(pole_tekstowe_przycisku(format!("C: {}", kompresja), jezyk, temat))
                                        .push(slajderr(*kompresja as i32, (0, 4), &SliderType::KonwersjaPngKompresja, kolor, temat, Length::FillPortion(2)))
                                        .push(space().width(15.))
                                )
                                .push(
                                    BdepthPng::iter()
                                        
                                        .step_by(2)
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
                                    BdepthPng::iter()
                                        
                                        .skip(1)
                                        .step_by(2)
                                        .fold(
                                            Row::new(), |row, wariant| {
                                                row.push(
                                                    btn_bdepth_konwersja(
                                                        wariant,
                                                        if bit_depth.contains(&wariant) { &BtnState::Processing } else { &BtnState::Disabled },
                                                        jezyk,
                                                        kolor,
                                                        temat
                                                    ),
                                                )
                                            }
                                        )
                                )
                                .push(space().height(25.))

                        } else {Column::new().push(space())}
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
        ).into()
}