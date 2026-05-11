use iced::widget::{button, text, Column, Row};

use crate::ui::podmenu::binarka::pack_body::binarka_pakowanie_body;
use crate::ui::podmenu::binarka::rozpak_body::binarka_rozpakowanie_body;
use crate::ui::podmenu::binarka::status_pack::status_pakowanie;
use crate::ui::podmenu::kolory::WYSOKOSC_CZCIONEK_PRZYCISKI;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::{DaneDoDekompresjaPlików, DaneDoKompresjaPlików};
use enumy::enums_structs_io::{LogPakowanie, LogRozpakowywanie};
use enumy::inne_ui::{ActProces, BtnState, UiPodstrony, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::{Element, Length};
use crate::ui::program::Program;

pub fn view_binarka<'a>(
    dane_p: &'a DaneDoKompresjaPlików,
    dane_d: &'a DaneDoDekompresjaPlików,
    jezyk: &'a WybórJęzyka,
    proces_pakowanie: &'a LogPakowanie,
    proces_rozpakowanie: &'a LogRozpakowywanie,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {
    

    let pakowanie =
        Column::new()
            .push(binarka_pakowanie_body(dane_p,jezyk,temat))
            .push(status_pakowanie(proces_pakowanie,jezyk,temat))
            .spacing(15)
            .width(Length::FillPortion(2));


    




    let lewa =

        Column::new()
        .push(
            Program::przycisk_binarka(/* &program::Program */, /* &'static str */, /* &'static str */, /* Message */, /* &iced::Color */, /* &BtnState */)
            hint_btn(
                button(
                    text(jezyk.t("ui_menu_bin_pack"))
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::Fill)
                        .center(),
                )
                    .width(Length::Fill)
                    .on_press(Message::ZmienWariant(UiPodstrony::BinPakowanie))
                    .style(
                        styl_przycisków(
                            if temat.temp.aktywny_proces == Some(ActProces::PakowaniePliku){
                                &BtnState::Processing
                            } else if temat.temp.aktywne_okno == UiPodstrony::BinPakowanie{
                                &BtnState::Active
                            } else {
                                &BtnState::Disabled
                            },
                            &temat.kolory.binarka,
                            temat,
                        )
                    ),
                jezyk.t("ui_menu_bin_pack"),
                temat,
            )
        )
        .push(
            hint_btn(
            button(
                text(jezyk.t("ui_menu_bin_unpack"))
                    .font(jezyk.get_font())
                    .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                    .width(Length::Fill)
                    .center(),
            )
                .width(Length::Fill)
                .on_press(Message::ZmienWariant(UiPodstrony::BinRozpakowanie))
                .style(styl_przycisków(
                    if temat.temp.aktywny_proces == Some(ActProces::RozpakowaniePliku){
                        &BtnState::Processing
                    } else if temat.temp.aktywne_okno == UiPodstrony::BinRozpakowanie{
                        &BtnState::Active
                    } else {
                        &BtnState::Disabled
                    },
                    &temat.kolory.binarka,
                    temat,
                )),
            jezyk.t("ui_menu_bin_unpack"),
            temat,
            )
        )
        .height(Length::Fill)
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(1));
    
    
    
    Row::new()
        .push(lewa)
        .push(
        match temat.temp.aktywne_okno{
                UiPodstrony::BinPakowanie => {pakowanie}
                UiPodstrony::BinRozpakowanie => {binarka_rozpakowanie_body(dane_d,proces_rozpakowanie, &temat.kolory.binarka, jezyk, temat)}
                _ => {Column::new()}
            }
        ).into()
}
