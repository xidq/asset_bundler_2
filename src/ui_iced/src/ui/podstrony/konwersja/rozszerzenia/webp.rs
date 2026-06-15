use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_konwersja, pole_tekstowe_przycisku, przycisk, przycisk_rozszerzenia};
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::BdepthWebp;
use enumy::rozszerzenia::ext::ImgExtTag;
use enumy::rozszerzenia::rozszenienia_zdjec::ImgExtWebp;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn webp<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Webp, ButtonType::KonwRozszerzenia, kolor, if dane.rozszerzenia.webp.is_some() { &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExtWebp {
                                jakosc, lossless, bit_depth
                            }) = &dane.rozszerzenia.webp
                        {
                            Column::new()
                                .push(
                                    match lossless  {
                                        false =>
                                            {
                                                Row::new().spacing(15).height(50.)
                                                    .push(pole_tekstowe_przycisku(format!("Q: {}", jakosc), jezyk, temat))
                                                    .push(slajderr(*jakosc, (0, 100), &SliderType::KonwersjaWebpJakosc, kolor, temat, Length::FillPortion(2)))
                                                    .push(space().width(15.))
                                            }
                                        true => {Row::new().height(50.)}
                                    }
                                )
                                .push(
                                    BdepthWebp::iter()
                                        
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
                                .push(przycisk("conversion_webp_losless", ButtonType::KonwWebpLoss,Length::FillPortion(1), Length::Fixed(50.),kolor,if *lossless { &BtnState::Processing } else { &BtnState::Disabled },jezyk,temat))
                                .push(space().height(25.))

                        } else {Column::new().push(space())}
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
        ).into()
}