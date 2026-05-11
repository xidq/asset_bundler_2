use iced::Element;
use iced::widget::{button, pick_list, text, text_input, Column, Row};
use iced_core::{Alignment, Color, Length};
use strum::IntoEnumIterator;
use enumy::dane_do_przetwarzania::DaneDoKompresjaPlików;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, UiPodstrony, UstawieniaThemeWsio};
use enumy::opcje::{OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::{hint_btn, hint_pick_list};
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::run_btn::start_btn;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::PakowanieBinarkiMessage;

pub fn binarka_pakowanie_body<'a>(
    dane_p:&'a DaneDoKompresjaPlików,
    jezyk:&'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio
) -> Element<'a,Message>{

    Column::new()
        .push(
            Row::new()
                .push(
                    hint_btn(
                        button(
                            folder_icon(
                                true,
                                if !dane_p.ścieżka_in.exists() {
                                    0
                                } else {
                                    2
                                },
                                &temat.kolory.binarka,
                            )
                        )
                            .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::InputPath))
                            .padding(10)
                            .style(
                                styl_przycisków(
                                    if dane_p.ścieżka_in.exists(){
                                        &BtnState::Active
                                    } else {
                                        &BtnState::Disabled
                                    },
                                    &temat.kolory.binarka,
                                    temat,
                                )
                            ),
                        jezyk.t("hint_binary_pack_choose_in_path_btn"),
                        temat,
                    )
                )
                .push(
                    text_input(jezyk.t("mgt_input_folder"), &dane_p.ścieżka_in.to_string_lossy())
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::InputPathText(xx)))
                        .padding(10)
                        .style(styl_text_input(dane_p.ścieżka_in.exists(), &temat.kolory.binarka,temat)),
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
                                if !dane_p.ścieżka_out.exists() {
                                    0
                                } else {
                                    2
                                },
                                &temat.kolory.binarka,
                            )
                        )
                            .on_press(Message::PakowanieBinarki(PakowanieBinarkiMessage::OutputPath))
                            .padding(10)
                            .style(
                                styl_przycisków(
                                    if dane_p.ścieżka_out.exists(){
                                        &BtnState::Active
                                    } else {
                                        &BtnState::Disabled
                                    },
                                    &temat.kolory.binarka,
                                    temat,
                                )
                            ),
                        jezyk.t("hint_binary_pack_choose_out_path_btn"),
                        temat,
                    )
                )
                .push(
                    text_input(
                        jezyk.t("mgt_output_folder"),
                        &dane_p.ścieżka_out.to_string_lossy(),
                    )
                        .font(jezyk.get_font())
                        .on_input(|path|Message::PakowanieBinarki(PakowanieBinarkiMessage::OutputPathText(path)))
                        .padding(10)
                        .style(styl_text_input(dane_p.ścieżka_out.exists(), &temat.kolory.binarka,temat)),
                )
                .spacing(10)
                .align_y(Alignment::Center),
        )
        // --- KOMPRESJA ---
        .push(
            Column::new()
                .push(
                    text(jezyk.t("mgt_compression_level_label"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .size(14),
                )
                .push(
                    hint_pick_list(
                        pick_list(
                            OptKompresjaPlikówPoziomKompresjiZstd::iter().collect::<Vec<_>>(),
                            Some(dane_p.kompresja),
                            |kompresja|Message::PakowanieBinarki(PakowanieBinarkiMessage::Kompresja(kompresja)),
                        )
                            .width(Length::Fill)
                            .padding(10)
                            .style(styl_pick_list(&temat.kolory.binarka,temat))
                            .menu_style(styl_menu_pick(&temat.kolory.binarka, temat)),
                        jezyk.t("hint_binary_pack_choose_compression"),
                        temat,
                    )
                )
                .spacing(5),
        )
        // --- FILTROWANIE ---
        .push(
            Column::new()
                .push(
                    text(jezyk.t("mgt_filter_label"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .size(14),
                )
                .push(
                    hint_pick_list(
                        pick_list(
                            OptKompresjaPlikówFiltracjaPlików::iter().collect::<Vec<_>>(),
                            Some(dane_p.filtracja),
                            |xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::Filtr(xx)),
                        )
                            .width(Length::Fill)
                            .padding(10)
                            .style(styl_pick_list(&temat.kolory.binarka,temat))
                            .menu_style(styl_menu_pick(&temat.kolory.binarka,temat)),
                        jezyk.t("mgt_filter_label"),
                        temat,
                    )
                )
                .spacing(5),
        )
        // --- NAZWA PLIKU ---
        .push(
            text_input(jezyk.t("mgt_file_name"), &dane_p.nazwa)
                .font(jezyk.get_font())
                .on_input(|xx|Message::PakowanieBinarki(PakowanieBinarkiMessage::Nazwa(xx)))
                .padding(10)
                .style(styl_text_input(!dane_p.nazwa.is_empty(), &temat.kolory.binarka,temat)),
        )
        // --- PRZYCISK START ---
        .push(
            start_btn(
                ActProces::PakowaniePliku,
                temat.temp.start_btn_status.bin_pak,
                "hint_binary_pack_process_btn",
                "hint_binary_pack_process_btn",
                &temat.kolory.binarka,
                jezyk,
                temat
            )
        )
        .spacing(15).into()
}