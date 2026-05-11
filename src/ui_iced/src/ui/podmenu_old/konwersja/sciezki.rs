use iced::Element;
use iced::widget::{button, checkbox, space, text, text_input, Column, Row};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{BtnState, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::checkbox::styl_checkbox;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub(crate) fn sciezki<'a>(dane:&'a DaneDoBathKonwersjaZdjec, czy_wyjscie_te_same: &'a bool, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {

        Column::new()
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button("📄")
                                .padding(10)
                                .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathInFile))
                                .style(styl_przycisków(
                                    ZbiorowePrzetwarzanieZdjęćMessage::PathInFile.get_id(),
                                    kolor,
                                    temat,
                                )),
                            jezyk.t("hint_conversion_choose_file_btn"),
                            temat,
                        )
                    )
                    .push(space().width(Length::Fixed(15.)))
                    .push(
                        hint_btn(
                            button(
                                folder_icon(
                                    true,
                                    match temat.btn_state.get(ZbiorowePrzetwarzanieZdjęćMessage::PathInFolder.get_id()){
                                        None => {0}
                                        Some(_) => {2}
                                    },
                                    kolor,
                                ))
                                .padding(10)
                                .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathInFolder))
                                .style(
                                    styl_przycisków(
                                        ZbiorowePrzetwarzanieZdjęćMessage::PathInFolder.get_id(),
                                    kolor,
                                    temat,
                                )
                                ),
                            jezyk.t("hint_conversion_choose_folder_btn"),
                            temat,
                        )
                    ),
            )
            .push(
                Row::new()
                    .push(
                        text_input(
                            jezyk.t("mgt_input_folder_or_file"),
                            &dane.ścieżka_wejściowa.to_string_lossy(),
                        )
                            .padding(10)
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathInText(xx)))
                            .style(styl_text_input(dane.ścieżka_wejściowa.exists(),kolor,temat,)),
                    ),
            )
            .push(
                hint_btn(
                    button(
                        text(jezyk.t("proces_conversion_reset_paths"))
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                        .padding(10)
                        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathsReset))
                        .style(styl_przycisków(ZbiorowePrzetwarzanieZdjęćMessage::PathsReset.get_id(),kolor, temat,)),
                    jezyk.t("hint_conversion_reset_paths"),
                    temat
                )
            )
            .push(
                checkbox(*czy_wyjscie_te_same)
                    .label(jezyk.t("proces_conversion_checkbox"))
                    .on_toggle(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathOutPathInBool(xx)))
                    .style(styl_checkbox(
                        kolor,
                        temat,)),
            )
            .push(
                Row::new()
                    .push(
                        hint_btn(
                            button(folder_icon(
                                !*czy_wyjscie_te_same,
                                if dane.ścieżka_wyjściowa.to_string_lossy().is_empty() {
                                    0
                                } else {
                                    2
                                },
                                kolor,
                            ))
                                .padding(10)
                                .on_press_maybe(
                                    match czy_wyjscie_te_same{
                                        false => Some(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathOutFolder)),
                                        true => None,
                                    })
                                .style(styl_przycisków(
                                    ZbiorowePrzetwarzanieZdjęćMessage::PathOutFolder.get_id(),
                                    kolor,
                                    temat,
                                )),
                            jezyk.t("hint_conversion_choose_output_folder_paths_same"),
                            temat,
                        )
                    )
                    .push(match czy_wyjscie_te_same {
                        true => text_input(
                            jezyk.t("mgt_output_folder"),
                            &dane.ścieżka_wyjściowa.to_string_lossy(),
                        )
                            .padding(10)
                            .font(jezyk.get_font())
                            .style(styl_text_input(dane.ścieżka_wyjściowa.exists(),kolor,
                                                   temat,)),
                        false => text_input(
                            jezyk.t("mgt_output_folder"),
                            &dane.ścieżka_wyjściowa.to_string_lossy(),
                        )
                            .padding(10)
                            .font(jezyk.get_font())
                            .on_input(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::PathOutText(xx)))
                            .style(styl_text_input(dane.ścieżka_wyjściowa.exists(),kolor,temat,)),
                    }),
            )
            .spacing(15)
            .padding(15)
            .width(Length::FillPortion(2)).into()
}