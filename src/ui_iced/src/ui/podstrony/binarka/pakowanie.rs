use iced::widget::{space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneBinPak;
use enumy::enums_structs_io::LogPakowanie;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podstrony::binarka::pakowanie_status::status_pakowanie;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::{pole_tekstowe_przycisku, przycisk, przycisk_startu};
use crate::widget::dropdown::dropdown;
use crate::widget::text_place::tekstowe_pole_wypelniane;

pub fn binarka_pak<'a>(
    dane:&'a DaneBinPak,
    log: &'a LogPakowanie,
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
                            if !dane.ścieżka_in.exists() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::BinKompPathIn,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_in.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )
                    
                )
            
                .push(tekstowe_pole_wypelniane(&dane.ścieżka_in, &TextInputType::BinKompPathIn, kolor, jezyk, temat, Length::Fill))
            
        )

        .push(

            Row::new().spacing(15)

                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_out.exists() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::BinKompPathOut,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_out.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )
                .push(tekstowe_pole_wypelniane(&dane.ścieżka_out, &TextInputType::BinKompPathOut, kolor, jezyk, temat, Length::Fill))

        )

        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku("mgt_filter_label", jezyk, temat))
                .push(dropdown::<OptKompresjaPlikówFiltracjaPlików, _>(dane, kolor, temat))
        )

        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku("mgt_compression_level_label", jezyk, temat))
                .push(dropdown::<OptKompresjaPlikówPoziomKompresjiZstd,_>(dane, kolor, temat))
        )
        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku("mgt_file_name", jezyk, temat))
                .push(tekstowe_pole_wypelniane(&dane.nazwa, &TextInputType::BinKompNazwa, kolor, jezyk, temat, Length::FillPortion(2)))
        )

        .push(space().height(50.))

        .push(przycisk_startu(&ActProces::BinPak, kolor, jezyk, temat))

        .push(status_pakowanie(log, kolor, jezyk, temat))
        .push(space().height(Length::FillPortion(1)))

}