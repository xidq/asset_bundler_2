use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_konwersja, przycisk_rozszerzenia};
use crate::widget::styles::styl_kontenera;
use encodery::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::BdepthQoi;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::rozszenienia_zdjec::ImgExtQoi;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::Color;
use strum::IntoEnumIterator;

pub fn qoi<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Qoi, ButtonType::KonwRozszerzenia, kolor, if dane.rozszerzenia.qoi.is_some() { &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExtQoi {
                                bit_depth
                            }) = &dane.rozszerzenia.qoi
                        {
                            Column::new()
                                .push(Row::new().height(50.))
                                .push(
                                    BdepthQoi::iter()
                                        
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
                                .push(space().height(25.))
                        } else {Column::new().push(space())}
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
        ).into()
}