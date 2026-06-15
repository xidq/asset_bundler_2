use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{btn_bdepth_merge, pole_tekstowe_przycisku, przycisk, przycisk_rozszerzenia};
use crate::widget::dropdown::dropdown;
use crate::widget::slajder::slajderr;
use crate::widget::styles::styl_kontenera;
use enumy::dane_do_przetwarzania::DaneMerge;
use enumy::inne_ui::{BtnState, ButtonType, RodzajeContainer, SliderType, UstawieniaThemeWsio};
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::ext::{ImgExtSingle, ImgExtTag};
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced_core::{Color, Length};
use strum::IntoEnumIterator;

pub fn rozszerzenia<'a>(dane: &'a DaneMerge, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new()
        .push(space().height(Length::FillPortion(1)))

        .push(
            ImgExtTag::iter()
                .filter(|wariant| !wariant.to_string().to_lowercase().contains("unknown"))
                .fold(
                    Row::new(), |row, wariant| {
                        row.push(
                            przycisk_rozszerzenia(
                                wariant.clone(),
                                ButtonType::MergeRozszerzenia,
                                kolor,
                                if dane.tag == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                jezyk,
                                temat
                            ),
                        )
                    }
                )
        )
        .push(
            match dane.tag{
                ImgExtTag::Jpg => {
                    container(
                        Row::new().height(300.)
                            .push(
                                if let
                                    ImgExtSingle::Jpg {
                                        jakosc,
                                        progresywny,
                                        bit_depth:_,
                                        sampling: _,
                                        quant: _,
                                        scans,
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("Q: {}%", jakosc), jezyk, temat))
                                                .push(slajderr(jakosc, (0,100), &SliderType::MergeJpgQuality, kolor, temat, Length::FillPortion(2)  ))
                                                .push(space().width(15.))
                                        )
                                        // .push(
                                        //     BdepthJpg::iter()
                                        //         
                                        //         .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                        //         .fold(
                                        //             Row::new(), |row, wariant| {
                                        //                 row.push(
                                        //                     btn_bdepth_merge(
                                        //                         wariant,
                                        //                         if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                        //                         jezyk,
                                        //                         kolor,
                                        //                         temat
                                        //                     ),
                                        //                 )
                                        //             }
                                        //         )
                                        // )
                                        .push(przycisk("conversion_jpg_prog", ButtonType::MergeJpgProg,Length::FillPortion(1), Length::Fixed(50.),kolor,if progresywny { &BtnState::Active} else { &BtnState::Disabled },jezyk,temat))
                                        .push(
                                            Row::new().spacing(15)
                                                .push(pole_tekstowe_przycisku(format!("scans: {}", scans), jezyk, temat))
                                                .push(slajderr(scans, (2,64), &SliderType::MergeJpgScans, kolor, temat, Length::FillPortion(2)  ))
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

                }
                ImgExtTag::Png => {
                    container(
                        Row::new().height(150.)
                            .push(
                                if let
                                    ImgExtSingle::Png {
                                        kompresja, bit_depth
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("C: {}", kompresja), jezyk, temat))
                                                .push(slajderr(kompresja, (0, 9), &SliderType::MergePngKompresja, kolor, temat, Length::FillPortion(2)))
                                                .push(space().width(15.))
                                        )
                                        .push(
                                            BdepthPng::iter()
                                                
                                                .step_by(2)
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
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
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
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
                }
                ImgExtTag::Webp => {
                    container(
                        Row::new().height(150.)
                            .push(
                                if let
                                    ImgExtSingle::Webp {
                                        jakosc, lossless, bit_depth
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(
                                            match lossless  {
                                                false =>
                                                    {
                                                        Row::new().spacing(15).height(50.)
                                                            .push(pole_tekstowe_przycisku(format!("Q: {}", jakosc), jezyk, temat))
                                                            .push(slajderr(jakosc, (0, 100), &SliderType::MergeWebpJakosc, kolor, temat, Length::FillPortion(2)))
                                                            .push(space().width(15.))
                                                    }
                                                true => {Row::new().height(50.)}
                                            }
                                        )
                                        .push(
                                            BdepthWebp::iter()
                                                
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                                                jezyk,
                                                                kolor,
                                                                temat
                                                            ),
                                                        )
                                                    }
                                                )
                                        )
                                        .push(przycisk("conversion_webp_losless", ButtonType::MergeWebpLoss,Length::FillPortion(1), Length::Fixed(50.),kolor,if lossless { &BtnState::Processing } else { &BtnState::Disabled },jezyk,temat))
                                        .push(space().height(25.))

                                } else {Column::new().push(space())}
                            )
                    ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
                }
                ImgExtTag::Tga => {
                    container(
                        Row::new().height(100.)
                            .push(
                                if let
                                    ImgExtSingle::Tga {
                                        bit_depth
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(space().height(50.))
                                        .push(
                                            BdepthTga::iter()
                                                
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                                                jezyk,
                                                                kolor,
                                                                temat
                                                            ),
                                                        )
                                                    }
                                                )
                                        )
                                } else {Column::new().push(space())}
                            )
                    ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
                }
                ImgExtTag::Ff => {
                    container(
                        Row::new().height(100.)
                            .push(
                                if let
                                    ImgExtSingle::Ff {
                                        metoda_kompresji
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(
                                            match metoda_kompresji {
                                                ForFfKompresja::Zstd(kompresja) => {
                                                    Row::new().spacing(15).height(50.)
                                                        .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                        .push(slajderr(kompresja, (1,22), &SliderType::MergeFfZstd, kolor, temat, Length::FillPortion(2)  ))
                                                        .push(space().width(15.))
                                                }
                                                ForFfKompresja::Bzip2(kompresja) => {
                                                    Row::new().spacing(15).height(50.)
                                                        .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                        .push(slajderr(kompresja, (1,9), &SliderType::MergeFfBzip2, kolor, temat, Length::FillPortion(2)  ))
                                                        .push(space().width(15.))
                                                }
                                                ForFfKompresja::Xz(kompresja) => {
                                                    Row::new().spacing(15).height(50.)
                                                        .push(pole_tekstowe_przycisku(format!("Q: {}", kompresja), jezyk, temat))
                                                        .push(slajderr(kompresja, (1,9), &SliderType::MergeFfXz, kolor, temat, Length::FillPortion(2)  ))
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
                }
                ImgExtTag::Qoi => {
                    container(
                        Row::new().height(100.)
                            .push(
                                if let
                                    ImgExtSingle::Qoi {
                                        bit_depth
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(Row::new().height(50.))
                                        .push(
                                            BdepthQoi::iter()
                                                
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
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
                }
                ImgExtTag::Avif => {
                    container(
                        Row::new().height(300.)

                            .push(
                                if let
                                    ImgExtSingle::Avif {
                                        chroma: _,
                                        speed,
                                        metoda_kompresji: _,
                                        lossy,
                                        bit_depth
                                    } = dane.rozszerzenie
                                {
                                    Column::new()
                                        .push(
                                            Row::new().spacing(15).height(50.)
                                                .push(pole_tekstowe_przycisku(format!("S: {}", speed), jezyk, temat))
                                                .push(slajderr(speed, (0,100), &SliderType::MergeAvifSpeed, kolor, temat, Length::FillPortion(2)  ))
                                                .push(space().width(15.))
                                        )
                                        .push(
                                            BdepthAvif::iter()
                                                
                                                .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                                .fold(
                                                    Row::new(), |row, wariant| {
                                                        row.push(
                                                            btn_bdepth_merge(
                                                                wariant,
                                                                if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                                                jezyk,
                                                                kolor,
                                                                temat
                                                            ),
                                                        )
                                                    }
                                                )
                                        )
                                        .push(przycisk("conversion_avif_lossless", ButtonType::MergeAvifLoss,Length::FillPortion(1), Length::Fixed(50.),kolor,if lossy.is_none() { &BtnState::Processing } else { &BtnState::Disabled },jezyk,temat))
                                        .push(
                                            match lossy {
                                                Some(ggg) => {
                                                    Row::new().spacing(15).height(50.)
                                                        .push(pole_tekstowe_przycisku(format!("Q: {}", ggg), jezyk, temat))
                                                        .push(slajderr(ggg, (0, 100), &SliderType::MergeAvifQuality, kolor, temat, Length::FillPortion(2)))
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
                }
                ImgExtTag::Unknown => {container(space())}
                ImgExtTag::Exr => {
                    container(
                        Row::new().height(100.)
                            .push(
                                if let ImgExtSingle::Exr {
                                    bit_depth, kompresja: _
                                } = dane.rozszerzenie {
                                    Column::new()
                                    .push(
                                        BdepthExr::iter()
                                            // .filter(|wariant| !wariant.to_string().to_lowercase().contains("luma"))
                                            .fold(
                                                Row::new().height(50.), |row, wariant| {
                                                    row.push(
                                                        btn_bdepth_merge(
                                                            wariant,
                                                            if bit_depth == wariant { &BtnState::Active } else { &BtnState::Disabled },
                                                            jezyk,
                                                            kolor,
                                                            temat
                                                        ),
                                                    )
                                                }
                                            )
                                    )
                                    .push(
                                        Row::new().spacing(15).height(50.)
                                            .push(pole_tekstowe_przycisku("mgt_compression", jezyk, temat))
                                            .push(dropdown::<ForExrKompresja, _>(dane, kolor, temat))
                                            .push(space().width(15.))
                                    )
                                } else {Column::new().push(space())}


                            )


                    ).style(styl_kontenera(true,RodzajeContainer::Góra, kolor, temat))
                }
            }
        )
        .push(
            match dane.tag{
                ImgExtTag::Jpg => {space()}
                ImgExtTag::Png => {space().height(150.)}
                ImgExtTag::Webp => {space().height(150.)}
                ImgExtTag::Tga => {space().height(200.)}
                ImgExtTag::Ff => {space().height(200.)}
                ImgExtTag::Qoi => {space().height(200.)}
                ImgExtTag::Avif => {space()}
                ImgExtTag::Unknown => {space()}
                ImgExtTag::Exr => {space().height(200.)}
            }
        )
        .push(space().height(Length::FillPortion(1)))
}