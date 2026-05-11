use iced::widget::{button, text_input, Button, Column, Row};
use iced_core::{Alignment, Color, Length};
use enumy::dane_do_przetwarzania::DaneDoDekompresjaPlików;
use enumy::enums_structs_io::LogRozpakowywanie;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::binarka::status_pack::status_rozpakowanie;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::run_btn::start_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::RozpakowanieBinarkiMessage;

pub fn binarka_rozpakowanie_body<'a>(dane_d: &DaneDoDekompresjaPlików, proces_rozpakowanie: &'a LogRozpakowywanie, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio ) -> Column<'a, Message> {

        Column::new()
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            Button::new("📄")
                                .on_press(Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::InputFile))
                                .padding(10)
                                .style(styl_przycisków(if dane_d.ścieżka_pliku.is_file() { &BtnState::Active } else { &BtnState::Disabled }, kolor,temat)),
                            jezyk.t("hint_binary_unpack_choose_in_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(jezyk.t("mgt_input_file"), &dane_d.ścieżka_pliku.to_string_lossy())
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::InputFileText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_d.ścieżka_pliku.exists(),kolor,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),


            )
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button(
                                folder_icon(
                                    true,
                                    if dane_d.ścieżka_docelowa.to_string_lossy().is_empty() {
                                        0
                                    } else {
                                        2
                                    },
                                    kolor,
                                ))
                                .on_press(Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::OutputPath))
                                .padding(10)
                                .style(styl_przycisków(if dane_d.ścieżka_docelowa.is_dir() { &BtnState::Active } else { &BtnState::Disabled }, kolor,temat)),
                            jezyk.t("hint_binary_unpack_choose_out_path_btn"),
                            temat,
                        )
                    )
                    .push(
                        text_input(
                            jezyk.t("mgt_input_folder"),
                            &dane_d.ścieżka_docelowa.to_string_lossy(),
                        )
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::RozpakowanieBinarki(RozpakowanieBinarkiMessage::OutputPathText(xx)))
                            .padding(10)
                            .style(styl_text_input(dane_d.ścieżka_docelowa.exists(),kolor,temat)),
                    )
                    .spacing(10)
                    .align_y(Alignment::Center),
            )
            .push(
                start_btn(ActProces::RozpakowaniePliku, temat.temp.start_btn_status.bin_unpak, "hint_binary_unpack_process_btn","hint_binary_unpack_process_btn", kolor, jezyk, temat)
            )
            .push(status_rozpakowanie(proces_rozpakowanie,jezyk, temat))
            .spacing(15)
            .width(Length::FillPortion(2))
}