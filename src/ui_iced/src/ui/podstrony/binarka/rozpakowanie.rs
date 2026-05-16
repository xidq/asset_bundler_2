use crate::ui::podstrony::binarka::rozpakowanie_status::status_rozpakowanie;
use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{przycisk, przycisk_startu};
use crate::widget::text_place::tekstowe_pole_wypelniane;
use enumy::dane_do_przetwarzania::DaneBinUnpak;
use enumy::enums_structs_io::LogRozpakowywanie;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{space, Column, Row};
use iced_core::{Color, Length};

pub fn binarka_rozpakowanie<'a>(
    dane:&'a DaneBinUnpak,
    log: &'a LogRozpakowywanie,
    kolor: &'a Color,
    jezyk:&'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio
) -> Column<'a, Message> {
    Column::new()

        .push(space().height(Length::FillPortion(1)))

        .push(

            Row::new().spacing(15)

                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_pliku.is_file() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::BinDekompPathIn,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_pliku.is_file(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )

                .push(tekstowe_pole_wypelniane(&dane.ścieżka_pliku, &TextInputType::BinDekompPathIn, kolor, jezyk, temat, Length::Fill))

        )
        .push(

            Row::new().spacing(15)

                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_docelowa.exists() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::BinDekompPathOut,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_docelowa.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )

                .push(tekstowe_pole_wypelniane(&dane.ścieżka_docelowa, &TextInputType::BinDekompPathOut, kolor, jezyk, temat, Length::Fill))

        )
        .push(space().height(50.))

        .push(przycisk_startu(&ActProces::BinUnpak, kolor, jezyk, temat))

        .push(status_rozpakowanie(log, kolor, jezyk, temat))
        .push(space().height(Length::FillPortion(1)))

}