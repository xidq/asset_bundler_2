use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{pole_tekstowe_przycisku, przycisk, przycisk_startu};
use crate::widget::text_place::tekstowe_pole_wypelniane;
use enumy::dane_do_przetwarzania::DaneMerge;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{space, text, Column, Row};
use iced_core::{Color, Length};

pub fn sciezki<'a>(dane: &'a DaneMerge, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    lazy_static::lazy_static! {
    static ref PUSTA_SCIEZKA: std::path::PathBuf = std::path::PathBuf::new();
}
        Column::new()
            .push(space().height(Length::FillPortion(1)))

            .push(

                Row::new().spacing(15).height(50.)
                    .push(text(String::from("R:")).font(jezyk.get_font()).size(25.).height(Length::Fill).width(30.).color(Color{r: 1., g: 0.2, b: 0.2, a: 0.6}).center())

                    .push(

                        przycisk(
                            folder_icon(
                                true,
                                if !dane.sciezka_r.as_ref().is_some_and(|xx| xx.is_file()) {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ),
                            ButtonType::MergPathInR,
                            Length::Fixed(40.), Length::Fixed(40.),

                            kolor,
                            if dane.sciezka_r.as_ref().is_some_and(|xx| xx.is_file()){ &BtnState::Active } else { &BtnState::Disabled },
                            jezyk,
                            temat
                        )

                    )

                    .push(tekstowe_pole_wypelniane(dane.sciezka_r.as_ref().unwrap_or(&PUSTA_SCIEZKA), &TextInputType::MergPathInR, kolor, jezyk, temat, Length::FillPortion(1)))

            )
            .push(

                Row::new().spacing(15).height(50.)
                    .push(text(String::from("G:")).font(jezyk.get_font()).size(25.).height(Length::Fill).width(30.).color(Color{r: 0.2, g: 1., b: 0.2, a: 0.6}).center())

                    .push(

                        przycisk(
                            folder_icon(
                                true,
                                if !dane.sciezka_g.as_ref().is_some_and(|xx| xx.is_file()) {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ),
                            ButtonType::MergPathInG,
                            Length::Fixed(40.), Length::Fixed(40.),

                            kolor,
                            if dane.sciezka_g.as_ref().is_some_and(|xx| xx.is_file()){ &BtnState::Active } else { &BtnState::Disabled },
                            jezyk,
                            temat
                        )

                    )

                    .push(tekstowe_pole_wypelniane(dane.sciezka_g.as_ref().unwrap_or(&PUSTA_SCIEZKA), &TextInputType::MergPathInG, kolor, jezyk, temat, Length::FillPortion(1)))

            )
            .push(

                Row::new().spacing(15).height(50.)
                    .push(text(String::from("B:")).font(jezyk.get_font()).size(25.).height(Length::Fill).width(30.).color(Color{r: 0.2, g: 0.2, b: 1., a: 0.6}).center())

                    .push(

                        przycisk(
                            folder_icon(
                                true,
                                if !dane.sciezka_b.as_ref().is_some_and(|xx| xx.is_file()) {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ),
                            ButtonType::MergPathInB,
                            Length::Fixed(40.), Length::Fixed(40.),

                            kolor,
                            if dane.sciezka_b.as_ref().is_some_and(|xx| xx.is_file()){ &BtnState::Active } else { &BtnState::Disabled },
                            jezyk,
                            temat
                        )

                    )

                    .push(tekstowe_pole_wypelniane(dane.sciezka_b.as_ref().unwrap_or(&PUSTA_SCIEZKA), &TextInputType::MergPathInB, kolor, jezyk, temat, Length::FillPortion(1)))

            )
            .push(

                Row::new().spacing(15).height(50.)
                    .push(text(String::from("A:")).font(jezyk.get_font()).size(25.).height(Length::Fill).width(30.).color(Color{r: 1., g: 1., b: 1., a: 0.5}).center())

                    .push(

                        przycisk(
                            folder_icon(
                                true,
                                if !dane.sciezka_a.as_ref().is_some_and(|xx| xx.is_file()) {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ),
                            ButtonType::MergPathInA,
                            Length::Fixed(40.), Length::Fixed(40.),

                            kolor,
                            if dane.sciezka_a.as_ref().is_some_and(|xx| xx.is_file()){ &BtnState::Active } else { &BtnState::Disabled },
                            jezyk,
                            temat
                        )

                    )

                    .push(tekstowe_pole_wypelniane(dane.sciezka_a.as_ref().unwrap_or(&PUSTA_SCIEZKA), &TextInputType::MergPathInA, kolor, jezyk, temat, Length::FillPortion(1)))

            )
            .push(

                Row::new().spacing(15).height(50.)

                    .push(

                        przycisk(
                            folder_icon(
                                true,
                                if !dane.sciezka_out.exists() {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ),
                            ButtonType::MergePathOut,
                            Length::Fixed(40.),
                            Length::Fixed(40.),
                            kolor,
                            if dane.sciezka_out.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                            jezyk,
                            temat
                        )

                    )
                    .push(tekstowe_pole_wypelniane(&dane.sciezka_out, &TextInputType::MergePathOut, kolor, jezyk, temat, Length::Fill))

            )
            .push(
                Row::new().spacing(15).height(50.)
                    .push(pole_tekstowe_przycisku("mgt_file_name", jezyk, temat))
                    .push(tekstowe_pole_wypelniane(&dane.nazwa, &TextInputType::MergeNazwa, kolor, jezyk, temat, Length::FillPortion(2)))
            )
            .push(space().height(25.))
            .push(przycisk_startu(&ActProces::Merge, &temat.kolory.laczenie, jezyk, temat))
            .push(space().height(Length::FillPortion(1)))

}