use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{pole_tekstowe_przycisku, przycisk, przycisk_startu};
use crate::widget::text_place::tekstowe_pole_wypelniane;
use enumy::dane_do_przetwarzania::DaneDdsUnpak;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{space, Column, Row};
use iced_core::{Color, Length};

pub fn rozpakowywanie<'a>(dane: &'a DaneDdsUnpak, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a,Message>{
    Column::new()
        .push(space().height(Length::FillPortion(1)))
        .push(

            Row::new().spacing(15)

                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_wejściowa.is_file() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::DdsRozPathIn,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_wejściowa.is_file(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )

                .push(tekstowe_pole_wypelniane(&dane.ścieżka_wejściowa, &TextInputType::DdsRozPathIn, kolor, jezyk, temat, Length::Fill))

        )

        .push(

            Row::new().spacing(15)

                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_wyjściowa.exists() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::DdsRozPathOut,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_wyjściowa.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )
                .push(tekstowe_pole_wypelniane(&dane.ścieżka_wyjściowa, &TextInputType::DdsRozPathOut, kolor, jezyk, temat, Length::Fill))

        )
        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku("mgt_file_name", jezyk, temat))
                .push(tekstowe_pole_wypelniane(&dane.nazwa, &TextInputType::DdsRozNazwa, kolor, jezyk, temat, Length::FillPortion(2)))
        )
        .push(space().height(50.))
        .push(przycisk_startu(&ActProces::DdsUnpak, kolor, jezyk, temat))

        .push(space().height(Length::FillPortion(1)))

        // .push(rozszerzenia(dane,kolor,jezyk,temat))
}