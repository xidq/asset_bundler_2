use crate::ui::wiadomosci::message_enum::Message;
use crate::widget::button::{pole_tekstowe_przycisku, przycisk, przycisk_startu};
use crate::widget::colors_n_stuff::KOLOR_CZCIONKI_SREDNI;
use crate::widget::dropdown::dropdown;
use crate::widget::text_place::tekstowe_pole_wypelniane;
use enumy::dane_do_przetwarzania::DaneDdsPak;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, BtnState, ButtonType, TextInputType, UstawieniaThemeWsio};
use enumy::rozszerzenia::kompresje::{ForDds, ForDdsKompresja};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{checkbox, space, Column};
use iced::widget::{text, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::path::PathBuf;
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;

fn tekst_sciezek<'a>(
    sciezki: &'a Option<Vec<PathBuf>>,
    yy: usize,
    jezyk: &'a WybórJęzyka,
) -> Element<'a, Message> {
    text(match sciezki {
        Some(xx) if yy < xx.len() => jezyk.format_sciezek(&xx[yy]),
        _ => "".to_string(),
    })
        .color(KOLOR_CZCIONKI_SREDNI)
        .font(jezyk.get_font())
        .height(20.)
        .into()
}

pub fn pakowanie<'a>(dane: &'a DaneDdsPak, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a,Message>{
    Column::new().padding(15)
        .push(space().height(Length::FillPortion(1)))

        .push(

            Row::new().spacing(15).height(130.)

                .push(
                    Column::new().spacing(15)
                        .push(
                            przycisk(
                                folder_icon(
                                    true,
                                    if !dane.ścieżka_wejściowa.as_ref().is_some_and(|xx| xx[0].exists())  {
                                        0
                                    } else {
                                        2
                                    },
                                    kolor,
                                ),
                                ButtonType::DdsPathInFiles,
                                Length::Fixed(40.), Length::Fixed(40.),

                                kolor,
                                if dane.ścieżka_wejściowa.as_ref().is_some_and(|xx| xx[0].is_file()){ &BtnState::Active } else { &BtnState::Disabled },
                                jezyk,
                                temat
                            )
                        )
                        .push(
                            przycisk(
                                folder_icon(
                                    true,
                                    if !dane.ścieżka_wejściowa.as_ref().is_some_and(|xx| xx[0].exists()){
                                        0
                                    } else {
                                        2
                                    },
                                    kolor,
                                ),
                                ButtonType::DdsPathInFolders,
                                Length::Fixed(40.), Length::Fixed(40.),

                                kolor,
                                if dane.ścieżka_wejściowa.as_ref().is_some_and(|xx| xx[0].is_dir()){ &BtnState::Active } else { &BtnState::Disabled },
                                jezyk,
                                temat
                            )
                        )

                )
                .push(
                    match dane.ścieżka_wejściowa.clone() {
                        None => {
                            Column::new().spacing(15)
                                .push(space().height(Length::Fixed(30.)))
                                .push(
                                    text(
                                        jezyk.t("msg_no_folder_nor_file")
                                    )
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .width(Length::Fill)
                                        .height(Length::Fixed(20.))
                                        .center()
                                )
                                .push(space().height(Length::Fixed(30.)))
                        },

                        Some(sciezki) => {
                            let len = sciezki.len();
                            let mut col = Column::new().spacing(5);

                            for i in 0..len.min(3) {
                                col = col.push(tekst_sciezek(&dane.ścieżka_wejściowa, i, jezyk));
                            }

                            if len == 4 {
                                col = col.push(tekst_sciezek(&dane.ścieżka_wejściowa, 3, jezyk));
                            } else if len > 4 {
                                col = col.push(
                                    text(format!("{} {} {}", jezyk.t("mgt_and"), len - 3, jezyk.t("mgt_more")))
                                        .font(jezyk.get_font())
                                        .height(20.)
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                );
                            }
                            let space_height = match len {
                                1 => 60.,
                                2 => 40.,
                                3 => 20.,
                                _ => 0.,
                            };

                            col.push(space().height(space_height))
                        }
                    }
                )

        )
        .push(

            Row::new().spacing(15).height(50.)

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
                        ButtonType::DdsPathOut,
                        Length::Fixed(40.), Length::Fixed(40.),

                        kolor,
                        if dane.ścieżka_wyjściowa.exists(){ &BtnState::Active } else { &BtnState::Disabled },
                        jezyk,
                        temat
                    )

                )

                .push(tekstowe_pole_wypelniane(&dane.ścieżka_wyjściowa, &TextInputType::DdsPathOut, kolor, jezyk, temat, Length::Fill))

        )
        .push(space().height(25.))
        .push(
            Row::new().spacing(15).height(50.)
                .push(pole_tekstowe_przycisku("mgt_file_name", jezyk, temat))
                .push(tekstowe_pole_wypelniane(&dane.nazwa, &TextInputType::DdsNazwa, kolor, jezyk, temat, Length::FillPortion(2)))
        )
        .push(space().height(25.))

        .push(
            Row::new().spacing(15).height(50.)
                .push(pole_tekstowe_przycisku("dds_compression", jezyk, temat))
                .push(dropdown::<ForDdsKompresja, _>(dane, kolor, temat))
                .push(space().width(15.))
        )
        .push(checkbox(dane.dx9).on_toggle(|_bool|Message::Dds(DdsMsg::Dx9)).label("Dx9? (tylko bc1/2/3, reszta dx10)"))
        .push(
            Row::new().spacing(15).height(50.)
                .push(pole_tekstowe_przycisku("dds_format", jezyk, temat))
                .push(dropdown::<ForDds, _>(dane, kolor, temat))
                .push(space().width(15.))
        )
        .push(space().height(50.))
        .push(przycisk_startu(&ActProces::DdsPak, kolor, jezyk, temat))
        .push(space().height(Length::FillPortion(1)))


}