use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::{pole_tekstowe_przycisku, przycisk_rozszerzenia};
use crate::widget::dropdown::dropdown;
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::ext::{ImgExt, ImgExtTag};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced::Element;
use iced_core::{Color, Length};

pub fn ff<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message> {
    Column::new()
        .push(przycisk_rozszerzenia(ImgExtTag::Ff, ButtonType::KonwRozszerzenia, kolor, if dane.tag.contains(&ImgExtTag::Ff) { &BtnState::Active } else { &BtnState::Disabled }, jezyk, temat))
        .push(
            container(
                Row::new()
                    .push(
                        if let Some(
                            ImgExt::Ff {
                                metoda_kompresji
                            }) = dane.rozszerzenia
                            .iter()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            Column::new()
                                .push(
                                    match metoda_kompresji {
                                        ForFfKompresja::Zstd(kompresja) => {
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                .push(slajderr(*kompresja as i32, (1,22), &SliderType::KonwersjaFfZstd, kolor, temat, Length::FillPortion(2)  ))
                                                .push(space().width(15.))
                                        }
                                        ForFfKompresja::Bzip2(kompresja) => {
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                .push(slajderr(*kompresja as i32, (1,9), &SliderType::KonwersjaFfBzip2, kolor, temat, Length::FillPortion(2)  ))
                                                .push(space().width(15.))
                                        }
                                        ForFfKompresja::Xz(kompresja) => {
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                .push(slajderr(*kompresja as i32, (1,9), &SliderType::KonwersjaFfXz, kolor, temat, Length::FillPortion(2)  ))
                                                .push(space().width(15.))
                                        }
                                        ForFfKompresja::Brak => {
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku("mgt_compression_none", jezyk, temat))

                                        }
                                    }
                                )
                                .push(
                                    Row::new().spacing(15)
                                        .push(pole_tekstowe_przycisku("conversion_jpg_qua", jezyk, temat))
                                        .push(dropdown::<ForFfKompresja, _>(dane, kolor, temat))
                                        .push(space().width(15.))
                                )
                        } else {Column::new().push(space())}
                    )
            ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
        ).into()
}