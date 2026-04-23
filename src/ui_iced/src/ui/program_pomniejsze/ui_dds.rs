use crate::ui::program_pomniejsze::kolory::{KOLOR_COTTON_CANDY, KOLOR_CZCIONKI_SREDNI, KOLOR_SPANISH_ORANGE, KOLOR_TŁA, WYSOKOSC_CZCIONEK_PRZYCISKI};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMessage;
use enumy::dane_do_przetwarzania::{DaneDoPakowaniaDds, DaneDoRozpakowaniaDds};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
pub(crate) use enumy::inne_ui::StronyDds;
use enumy::opcje::{OptFormatDds, OptKompresjaDds};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, row, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use strum::IntoEnumIterator;
use enumy::ikony::folder_icon;

pub fn view_dds(
    dane_pakowanie: DaneDoPakowaniaDds,
    dane_rozpakowanie: DaneDoRozpakowaniaDds,
    wybrane_okno: &StronyDds,
    jezyk: WybórJęzyka,
    czy_jest_proces_zaczety: (bool,bool),
    log_pakowanie: LogPakowaniaDds,
    log_rozpakowywanie: LogRozpakowywanieDds,
    main_process_check: bool,

) -> Element<'_, Message> {

    let czy_sie_nada_na_wyslanie_pakowanie=dane_pakowanie.ścieżka_wejściowa.exists() && dane_pakowanie.ścieżka_wyjściowa.exists() && !dane_pakowanie.nazwa.is_empty();
    let lewa = Column::new()
        // .push(space().height(Length::FillPortion(10)))
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZplikuDoDds)))
            .style(styl_przycisków(
                czy_jest_proces_zaczety.0,
                matches!(wybrane_okno, StronyDds::ZplikuDoDds),
                KOLOR_COTTON_CANDY,
            )),
        )
        .push(
            button(
                text(jezyk.t(""))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
            .width(Length::Fill)
            .on_press(Message::Dds(DdsMessage::ZmienMenuDds(StronyDds::ZddsDoPliku)))
            .style(styl_przycisków(
                czy_jest_proces_zaczety.1,
                matches!(wybrane_okno, StronyDds::ZddsDoPliku),
                KOLOR_COTTON_CANDY,
            )),
        )
        // .push(space().height(Length::FillPortion(10)))
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));

    let pakowanie = Column::new()
        .push(
            text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane_pakowanie.ścieżka_wejściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(10))


                ).spacing(15)
        )
        .push(
            text(jezyk.t("ŚcieżkaWyjściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane_pakowanie.ścieżka_wyjściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xx|Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaŚcieżkiWyjściowej(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(10))


                ).spacing(15)
        )
        .push(
            Row::new()
                .push(text("wybierz nazwę").font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI).height(40.).width(Length::FillPortion(3)).center())
                .push(
                    container(
                        text_input(
                            jezyk.t("input_name"),
                            &dane_pakowanie.nazwa
                        ).font(jezyk.get_font())
                            .padding(10)
                            .style(styl_text_input(!dane_pakowanie.nazwa.is_empty(),KOLOR_COTTON_CANDY, KOLOR_TŁA))
                            .on_input(|xx|Message::Dds(DdsMessage::DdsPakowanieZmianaNazwy(xx))).style(styl_text_input(dane_pakowanie.ścieżka_wyjściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(7))


                ).spacing(15)
        )
        .push(ui_standard_oddzielacz())
        .push(
            text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            pick_list(
                OptFormatDds::iter().collect::<Vec<_>>(),
                Some(dane_pakowanie.format),
                |format| Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaWybranegoFormatu(format))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(KOLOR_COTTON_CANDY, KOLOR_TŁA))
                .menu_style(styl_menu_pick(KOLOR_COTTON_CANDY, KOLOR_TŁA)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            text(jezyk.t("Filter:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            pick_list(
                OptKompresjaDds::iter().collect::<Vec<_>>(),
                Some(dane_pakowanie.kompresja),
                |kompresja| Message::Dds(DdsMessage::DDS_Pakowanie_ZmianaWybranejKompresji(kompresja))
            )
                .text_line_height(2.)
                .width(Length::Fill)
                .style(styl_pick_list(KOLOR_COTTON_CANDY, KOLOR_TŁA))
                .menu_style(styl_menu_pick(KOLOR_COTTON_CANDY, KOLOR_TŁA)),
        )
        .push(ui_standard_oddzielacz())
        .push(
            if czy_sie_nada_na_wyslanie_pakowanie && !czy_jest_proces_zaczety.0 && !main_process_check {
                button(
                    text(jezyk.t("process_btn_start"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .width(Length::Fill)
                        .center(),
                )
                    .on_press(Message::Dds(DdsMessage::DdsPakowanieWysylanieDanych))
                    .height(Length::Fixed(40.))
                    .width(Length::Fill)
                    .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
            } else {
                button(
                    text(if czy_jest_proces_zaczety.0 {
                        jezyk.t("btn_bussy_processing")
                    } else if main_process_check {
                        jezyk.t("btn_bussy_processing_other")
                    } else {
                        jezyk.t("btn_gib_data")
                    })
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.7))
                        .width(Length::Fill)
                        .center(),
                )
                    .width(Length::Fill)
                    .height(Length::Fixed(40.))
                    .style(styl_przycisków(
                        false,
                        czy_jest_proces_zaczety.0,
                        KOLOR_COTTON_CANDY,
                    ))
            },
        )


        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .spacing(15)
        .padding(15);

    let rozpakowanie = Column::new()
        .push(
            text(jezyk.t("ŚcieżkaWejściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wejściowa.exists() { 2 } else { 0 }, KOLOR_COTTON_CANDY))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane_rozpakowanie.ścieżka_wejściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::DdsrozpakowanieZmianaŚcieżkiWejściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(),  KOLOR_COTTON_CANDY,KOLOR_TŁA))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(10))


                ).spacing(15)
        )
        .push(
            text(jezyk.t("ŚcieżkaWyjściowa:")).font(jezyk.get_font()).color(KOLOR_CZCIONKI_SREDNI)
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(true, if dane_rozpakowanie.ścieżka_wyjściowa.exists() { 1 } else { 0 }, KOLOR_COTTON_CANDY))
                        .padding(10)
                        .on_press(Message::Dds(DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowejWybór))
                        .style(styl_przycisków(false, false, KOLOR_COTTON_CANDY))
                        .width(Length::Fixed(40.))
                        .height(Length::Fixed(40.)),
                )
                .push(
                    container(
                        text_input(
                            jezyk.t("output_folder_or_file"),
                            &dane_rozpakowanie.ścieżka_wyjściowa.to_string_lossy()
                        ).font(jezyk.get_font())
                            .padding(10)
                            .on_input(|xxx|Message::Dds(DdsMessage::DdsRozpakowanieZmianaŚcieżkiWyjściowej(xxx))).style(styl_text_input(dane_pakowanie.ścieżka_wejściowa.exists(), KOLOR_COTTON_CANDY, KOLOR_TŁA))
                            .width(Length::Fill)
                    ).height(Length::Fixed(40.))
                        .width(Length::FillPortion(10))


                ).spacing(15)
        )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .spacing(15)
        .padding(15);

    let prawa = match wybrane_okno {
        StronyDds::ZplikuDoDds => pakowanie,
        StronyDds::ZddsDoPliku => rozpakowanie,
    };

    row![lewa, prawa].into()
}
