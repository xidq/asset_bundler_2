use iced::Element;
use iced::widget::{space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::{DaneKonw, DaneMerge};
use enumy::enums_structs_io::LogPrzetwarzanieFot;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::lang::odmiana_liczbowa;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::widget::button::{przycisk, przycisk_startu};
use crate::widget::status::{status_text, status_text_bar};
use crate::widget::text_place::tekstowe_pole_wypelniane;

pub fn sciezki<'a>(dane: &'a DaneKonw, log: &'a LogPrzetwarzanieFot, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message>{
    Column::new().padding(15).spacing(15)
        .push(space().height(Length::FillPortion(1)))

        .push(

            Row::new().spacing(15)

                .push(space().width(Length::FillPortion(1)))

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
                        ButtonType::KonwPathInFile,
                        Length::Fixed(40.),
                        Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_wejściowa.is_file(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )
                .push(

                    przycisk(
                        folder_icon(
                            true,
                            if !dane.ścieżka_wejściowa.is_dir() {
                                0
                            } else {
                                2
                            },
                            kolor,
                        ),
                        ButtonType::KonwPathInFolder,
                        Length::Fixed(40.), Length::Fixed(40.),
                        kolor,
                        if dane.ścieżka_wejściowa.is_dir(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )
                .push(space().width(Length::FillPortion(1)))

        )
        .push(tekstowe_pole_wypelniane(&dane.ścieżka_wejściowa, &TextInputType::KonwPathIn, kolor, jezyk, temat, Length::Fill))
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
                        ButtonType::KonwPathOut,
                        Length::Fixed(40.), Length::Fixed(40.),

                        kolor,
                        if dane.ścieżka_wyjściowa.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )

                .push(tekstowe_pole_wypelniane(&dane.ścieżka_wyjściowa, &TextInputType::KonwPathOut, kolor, jezyk, temat, Length::Fill))

        )
        .push(space().height(50.))

        .push(przycisk_startu(&ActProces::Konw, kolor, jezyk, temat))
        .push(space().height(10.))
        
        .push(
            if log.błąd.is_empty() {
                Column::new()
                    .push(status_text(if let Some(val) = log.plik_początek{format!("Znaleziono {} plikow",val)}else{String::new()}, jezyk))
                    .push(status_text(log.msg_walidacja.clone(), jezyk))
                    .push(status_text_bar(log.plik_procent,"zmielono:",kolor,jezyk,temat))
            } else {

                Column::new()
                    .push(text(
                        log.błąd.clone()
                    )
                        .font(jezyk.get_font())
                        .color(temat.obecny_theme.err_font))
                
                    
            }
        )


        .push(space().height(Length::FillPortion(1)))

}