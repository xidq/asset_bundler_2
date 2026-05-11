use iced::Element;
use iced::widget::{container, scrollable, space, Column, Row};
use iced_core::{Color, Length};
use strum::IntoEnumIterator;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::rozszerzenia::{ImgExt, ImgExtTag};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::{btn_bdepth_konwersja, pole_tekstowe_przycisku, przycisk, przycisk_rozszerzenia};
use crate::widget::dropdown::dropdown;
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;

pub fn jpg<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Jpg, ButtonType::KonwRozszerzenia, kolor, if dane.tag.contains(&ImgExtTag::Jpg){ &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExt::Jpg {
                                jakosc,
                                progresywny,
                                bit_depth,
                                sampling: _,
                                quant: _,
                                scans,
                            }) = dane.rozszerzenia
                            .iter()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            Column::new()
                                .push(
                                    Row::new().spacing(15).height(50.)
                                        .push(pole_tekstowe_przycisku(format!("Q: {}%", jakosc), jezyk, temat))
                                        .push(slajderr(*jakosc as i32, (0,100), &SliderType::KonwJpgQuality, kolor, temat, Length::FillPortion(2)  ))
                                        .push(space().width(15.))
                                )
                                .push(
                                    BdepthJpg::iter()
                                        .into_iter()
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
                                .push(przycisk("conversion_jpg_prog", ButtonType::KonwJpgProg,Length::FillPortion(1), Length::Fixed(50.),kolor,if *progresywny { &BtnState::Processing } else { &BtnState::Disabled },jezyk,temat))
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku(format!("scans: {}", scans), jezyk, temat))
                                        .push(slajderr(*scans as i32, (2,64), &SliderType::KonwersjaJpgScans, kolor, temat, Length::FillPortion(2)  ))
                                        .push(space().width(15.))
                                )
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_jpg_sampling", jezyk, temat))
                                        .push(dropdown::<ForJpgSamplingFac, _>(dane, kolor, temat))
                                        .push(space().width(15.))
                                )
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_jpg_qua", jezyk, temat))
                                        .push(dropdown::<ForJpgQuant, _>(dane, kolor, temat))
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