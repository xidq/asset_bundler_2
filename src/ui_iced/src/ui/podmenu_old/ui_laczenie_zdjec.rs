use crate::ui::podmenu::kolory::{
    KOLOR_CZCIONKI_SREDNI,
    WYSOKOSC_CZCIONEK_PRZYCISKI,
};
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::scroll::styl_scrollable;
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::style_fn::text_input::styl_text_input;
use crate::ui::podmenu::ui_standard::oddzielacz::ui_standard_oddzielacz;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoŁączeniaZdjęć;
use enumy::ikony::folder_icon;
use enumy::inne_ui::{ActProces, RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, scrollable, slider, space, text, text_input, Column, Row};
use iced::Element;
use iced_core::{Color, Length};
use std::mem::discriminant;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use strum::{EnumMessage, IntoEnumIterator};
use enumy::rozszerzenia::bdepth::{BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::kolor::{JpgQuant, JpgSamplingFac};
use enumy::rozszerzenia::rozszerzenia::{RozszerzeniaPojedyncze, RozszerzeniaZnacznik};
use crate::ui::podmenu::fn_ogolne::{btn_zbiorowe_bdepth, btn_zbiorowe_rozszerzenia};
use crate::ui::podmenu::metody_do_wariantow::bit_depth_laczenie::{btn_bdepth_png_laczenie, btn_bdepth_qoi_laczenie, btn_bdepth_tga_laczenie, btn_bdepth_webp_laczenie, btn_komp_ff_laczenie};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_laczenie;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage::JpgSampling;

pub fn view_laczenie<'a>(
    dane: &'a DaneDoŁączeniaZdjęć,
    jezyk: &'a WybórJęzyka,
    temat: &'a UstawieniaThemeWsio,
) -> Element<'a, Message> {

    let check_opcjonalne_ścieżki=
        dane.sciezka_out.exists() &&
        !dane.nazwa.is_empty() &&
        (
            dane.sciezka_r.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_g.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_b.clone().unwrap_or(PathBuf::from("")).exists() ||
            dane.sciezka_a.clone().unwrap_or(PathBuf::from("")).exists()
        );

    let lewa_kolumna = Column::new()
        //RED
        .push(
            Row::new()
                .push(
                    text("R:")
                        .color(Color::from_rgba(1., 0.5, 0.5, 0.7))
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    hint_btn(
                        button(text("📄").width(Length::Fill).center())
                            .padding(10)
                            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieR))
                            .style(styl_przycisków("", None, &temat.kolory.laczenie, temat,))
                            .width(Length::FillPortion(2))
                            .height(Length::Fixed(40.)),
                        jezyk.t(""),
                        temat
                    )
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_r.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieRPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki, &temat.kolory.laczenie, temat,))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //green
        .push(
            Row::new()
                .push(
                    text("G:")
                        .color(Color::from_rgba(0.5, 1., 0.5, 0.7))
                        .size(18.)
                        .font(jezyk.get_font())
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieG))
                        .style(styl_przycisków(false, false,  &temat.kolory.laczenie, temat,))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_g.as_ref().unwrap_or(&PathBuf::new()).to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieGPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki, &temat.kolory.laczenie, temat,))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //blue
        .push(
            Row::new()
                .push(
                    text("B:")
                        .color(Color::from_rgba(0.5, 0.5, 1., 0.7))
                        .font(jezyk.get_font())
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieB))
                        .style(styl_przycisków(false, false,  &temat.kolory.laczenie, temat,))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane
                                .sciezka_b
                                .as_ref()
                                .unwrap_or(&PathBuf::new())
                                .to_string_lossy(),
                        )
                        .font(jezyk.get_font())
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieBPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki, &temat.kolory.laczenie, temat,))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        //alpha
        .push(
            Row::new()
                .push(
                    text("A:")
                        .color(KOLOR_CZCIONKI_SREDNI)
                        .font(jezyk.get_font())
                        .size(18.)
                        .font(jezyk.get_font())
                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                        .width(Length::FillPortion(1))
                        .center(),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    button(text("📄").width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieA))
                        .style(styl_przycisków(false, false,  &temat.kolory.laczenie, temat,))
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane
                                .sciezka_a
                                .as_ref()
                                .unwrap_or(&PathBuf::new())
                                .to_string_lossy(),
                        )
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieAPathChanged(xx)))
                        .style(styl_text_input(check_opcjonalne_ścieżki, &temat.kolory.laczenie, temat,))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(ui_standard_oddzielacz())
        .push(
            text("Ścieżka Wyjściowa:")
                .font(jezyk.get_font())
                .width(Length::Fill)
                .color(Color::from_rgba(1., 1., 1., 0.7))
                .size(18.)
                .center(),
        )
        .push(
            Row::new()
                .push(
                    button(folder_icon(
                        !dane.sciezka_out.to_string_lossy().is_empty() ,
                        if dane.sciezka_out.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        &temat.kolory.laczenie,
                    ))
                    .padding(10)
                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzFolderOutFotoLaczenie))
                    .style(styl_przycisków(false, false,  &temat.kolory.laczenie, temat,))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(40.)),
                )
                .push(space().width(Length::FillPortion(1)))
                .push(
                    container(
                        text_input(
                            jezyk.t("input_folder_or_file"),
                            &dane.sciezka_out.to_string_lossy(),
                        )
                        .padding(10)
                        .font(jezyk.get_font())
                        .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieOutPathChanged(xx)))
                        .style(styl_text_input(dane.sciezka_out.exists(), &temat.kolory.laczenie, temat,))
                        .width(Length::Fill),
                    )
                    .height(Length::Fixed(40.))
                    .width(Length::FillPortion(10)),
                ),
        )
        .push(
            container(
                text_input(jezyk.t("input_name"), &dane.nazwa)
                    .padding(10)
                    .font(jezyk.get_font())
                    .on_input(|xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WybierzPlikInFotoLaczenieNazwaChanged(xx)))
                    .style(styl_text_input(!dane.nazwa.is_empty(), &temat.kolory.laczenie, temat,))
                    .width(Length::Fill),
            )
            .height(Length::Fixed(40.))
            .width(Length::FillPortion(10)),
        )
        .push(if check_opcjonalne_ścieżki && temat.temp.aktywny_proces == ActProces::Żodyn {
            button(
                text(jezyk.t("process_btn_start"))
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.8))
                    .width(Length::Fill)
                    .center(),
            )
            .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::WysylkaDanychDoLaczeniaZdjec))
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(false, false, &temat.kolory.laczenie, temat,))
        } else {
            button(
                text(if temat.temp.aktywny_proces == ActProces::ŁączenieZdjęć {
                    jezyk.t("btn_bussy_processing")
                } else if temat.temp.aktywny_proces != ActProces::Żodyn {
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
            .style(styl_przycisków(false, temat.temp.aktywny_proces == ActProces::ŁączenieZdjęć, &temat.kolory.laczenie, temat,))
        })
        .padding(15)
        .spacing(15)
        .width(Length::FillPortion(3));

    let prawa_kolumna = scrollable(
        Column::new()
            // match dane.out_format {
            //     OptRozszerzeniaPlikówZdjęciowych::Jpg { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Png { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Webp { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Tga { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Ff { .. } => {}
            //     OptRozszerzeniaPlikówZdjęciowych::Qoi { .. } => {}
            // } => {}
            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Jpg, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(
                if let RozszerzeniaPojedyncze::Jpg { jakosc, sampling, quant, scans, .. } = &dane.out_format {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=100,
                                        *jakosc,
                                        |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciJpg(xx)),
                                    )
                                        .style(styl_sliderów( &temat.kolory.laczenie, temat,))
                                        .width(Length::FillPortion(6)).height(50.),
                                )
                                .push(space().width(Length::FillPortion(1)))
                                .push(
                                    text(format!("Q: {}%", jakosc))
                                        .color(Color::from_rgba(1., 1., 1., 0.6))
                                        .font(jezyk.get_font())
                                        .width(Length::FillPortion(4)).height(Length::Fill).center(),
                                ).height(Length::FillPortion(1)).padding(15)
                        )
                        .push(
                            Row::new()
                                .push(
                                    pick_list(JpgSamplingFac::iter().collect::<Vec<_>>(), Some(*sampling), |hh|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgSampling(hh)))
                                        .width(Length::FillPortion(5))
                                        .padding(2)
                                        .text_line_height(1.5)
                                        .style(styl_pick_list( &temat.kolory.laczenie, temat,))
                                        .menu_style(styl_menu_pick( &temat.kolory.laczenie, temat,))
                                        .width(Length::FillPortion(4))
                                )
                                .push(
                                    pick_list(JpgQuant::iter().collect::<Vec<_>>(), Some(*quant), |hh|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgQua(hh)))
                                        .width(Length::FillPortion(5))
                                        .padding(2)
                                        .text_line_height(1.5)
                                        .style(styl_pick_list( &temat.kolory.laczenie, temat,))
                                        .menu_style(styl_menu_pick( &temat.kolory.laczenie, temat,))
                                        .width(Length::FillPortion(4))
                                ).padding(15).spacing(10)
                        )
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        2..=64,
                                        *scans,
                                        |vv|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaEdycjaZmianaJpgScans(vv))
                                    )
                                        .height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów( &temat.kolory.laczenie, temat,)),
                                )
                                .push(
                                    text(format!(
                                        "Sc: {}",
                                        scans
                                    ))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                                )
                                .padding(15)
                        )

                ).height(150.).style(styl_kontenera(true,RodzajeContainer::Góra,&temat.kolory.laczenie, temat))

            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Png, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(if let RozszerzeniaPojedyncze::Png { kompresja, bit_depth } = &dane.out_format{
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=9,
                                        *kompresja,
                                        |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiPng(xx)),
                                    )
                                        .style(styl_sliderów(&temat.kolory.laczenie, temat)).width(Length::FillPortion(6)).height(20.),
                                )
                                .push(
                                    text(format!("C: {}", kompresja))
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .height(Length::Fill)
                                        .width(Length::FillPortion(4))
                                        .center(),
                                ).padding(15)
                                .width(Length::FillPortion(12)),
                        ).height(Length::FillPortion(1))
                        .push(
                            BdepthPng::iter()
                                .filter(|wariant|{!wariant.get_message().unwrap_or("brak danych").to_lowercase().contains("luma")})
                                .into_iter()
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_png_laczenie(
                                            wariant,
                                            bit_depth,
                                            jezyk.get_font(),
                                            &temat.kolory.laczenie,
                                            temat
                                        ),
                                    )
                                })
                        ).height(Length::FillPortion(1))


                ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.laczenie, temat))
            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Webp, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(if let RozszerzeniaPojedyncze::Webp {jakosc, lossless, bit_depth,} = dane.out_format {
                container(
                    Column::new()
                        .push(
                            Row::new()

                            .push(
                                slider(
                                    0..=100,
                                    jakosc,
                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaJakosciWebp(xx)),
                                )
                                    .style(styl_sliderów(&temat.kolory.laczenie, temat)).height(20.).width(Length::FillPortion(6)),
                            )
                            .push(
                                text(format!(
                                    "Q: {}%",
                                    jakosc
                                ))
                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                            ).padding(15).height(Length::FillPortion(1))

                        )
                        .push(
                            BdepthWebp::iter()
                                .into_iter()
                                .fold(Row::new(), |row, wariant| {
                                    row.push(
                                        btn_bdepth_webp_laczenie(
                                            wariant,
                                            &bit_depth,
                                            jezyk.get_font(),
                                            &temat.kolory.laczenie,
                                            temat
                                        ),
                                    )
                                })
                                .push(
                                    button(
                                        text(if lossless {
                                            "Lossless"
                                        } else {
                                            "Lossy"
                                        })
                                        .color(Color::from_rgba(1., 1., 1., 0.6))
                                        .font(jezyk.get_font())
                                        .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
                                        .width(Length::Fill)
                                        .center(),
                                    )
                                    .width(Length::FillPortion(1))
                                    .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianalosslessWebp))
                                    .style(styl_przycisków(
                                        false,
                                        lossless,
                                        &temat.kolory.laczenie, temat
                                    )).height(Length::Fixed(50.)),
                                ).height(Length::FillPortion(1))

                        )
                ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.laczenie, temat))
            } else {
                container(Column::new())
            })

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Tga, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(
                if let RozszerzeniaPojedyncze::Tga{ bit_depth } = dane.out_format{
                    container(
                        Column::new()
                            .push(space().height(Length::FillPortion(1)))
                            .push(
                                BdepthTga::iter()
                                    .filter(|wariant|{!wariant.get_message().unwrap_or("brak danych").to_lowercase().contains("luma")})
                                    .into_iter()
                                    .fold(Row::new(), |row, wariant| {
                                        row.push(
                                            btn_bdepth_tga_laczenie(
                                                wariant,
                                                &bit_depth,
                                                jezyk.get_font(),
                                                &temat.kolory.laczenie,
                                                temat
                                            ),
                                        )
                                    })
                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.laczenie, temat))
                }else{
                    container(Column::new())
                }
            )



            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Ff, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(
                if let RozszerzeniaPojedyncze::Ff{ metoda_kompresji } = dane.out_format{
                    container(
                        Column::new()
                            .push(
                                match metoda_kompresji{
                                    OptMetodaKompresjiZdjecia::Zstd(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=22,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfZstd(xx)),
                                                )
                                                    .style(styl_sliderów(&temat.kolory.laczenie, temat)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))

                                    }
                                    OptMetodaKompresjiZdjecia::Bzip2(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=9,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfBzip2(xx)),
                                                )
                                                    .style(styl_sliderów(&temat.kolory.laczenie, temat)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))
                                    }
                                    OptMetodaKompresjiZdjecia::Xz(bb) => {
                                        Row::new()
                                            .push(
                                                slider(
                                                    1..=9,
                                                    bb,
                                                    |xx|Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaKompresjiFfXz(xx)),
                                                )
                                                    .style(styl_sliderów(&temat.kolory.laczenie, temat)).height(20.).width(Length::FillPortion(6)),
                                            )
                                            .push(
                                                text(format!(
                                                    "C: {}",
                                                    bb
                                                ))
                                                    .color(Color::from_rgba(1., 1., 1., 0.5))
                                                    .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(50.)),
                                            ).padding(15).height(Length::Fixed(50.))
                                    }
                                    OptMetodaKompresjiZdjecia::Brak => {
                                        Row::new().height(Length::Fixed(50.))
                                    }
                                }
                            )
                            .push(
                                
                                OptMetodaKompresjiZdjecia::iter()
                                    .into_iter()
                                    .fold(Row::new(), |row, wariant| {
                                        row.push(
                                            btn_komp_ff_laczenie(
                                                wariant,
                                                &metoda_kompresji,
                                                jezyk.get_font(),
                                                &temat.kolory.laczenie,
                                                temat
                                            ),
                                        )
                                    })
                                
                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.laczenie, temat))
                }else{
                    container(Column::new())
                }
            )

            .push(space().height(Length::Fixed(20.)))
            .push(ui_standard_oddzielacz())
            .push(space().height(Length::Fixed(20.)))

            .push(rozszerzenie_laczenie( dane.clone(), RozszerzeniaZnacznik::Qoi, &temat.kolory.laczenie, jezyk.get_font(),  temat))
            .push(
                if let RozszerzeniaPojedyncze::Qoi{ bit_depth } = dane.out_format{
                    container(
                        Column::new()
                            .push(space().height(Length::FillPortion(1)))
                            .push(
                                BdepthQoi::iter()
                                    .into_iter()
                                    .fold(Row::new(), |row, wariant| {
                                        row.push(
                                            btn_bdepth_qoi_laczenie(
                                                wariant,
                                                &bit_depth,
                                                jezyk.get_font(),
                                                &temat.kolory.laczenie,
                                                temat
                                            ),
                                        )
                                    })

                            ).height(Length::FillPortion(1))
                    ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,&temat.kolory.laczenie, temat,))
                }else{
                    container(Column::new())
                }
            )
            .padding(15)
            // .spacing(15)
            .width(Length::FillPortion(2)),
    ).style(styl_scrollable(&temat.kolory.laczenie, temat,));

    Row::new().push(lewa_kolumna).push(prawa_kolumna).into()
}

// fn btn_zmiany_rozszerzenia<'a>(lell: RozszerzeniaZnacznik, co_istnieje:Rc::clone(&dane)DoŁączeniaZdjęć, font:iced::Font, kolor:&'a Color, temat: &'a UstawieniaThemeWsio) -> Element<'a, Message>{
//
//     let (nazwa,msg) = match lell {
//         RozszerzeniaZnacznik::Jpg => ("Jpg",
//                                       RozszerzeniaZnacznik::Jpg
//         ),
//         RozszerzeniaZnacznik::Png=> ("Png",
//                                      RozszerzeniaZnacznik::Png
//         ),
//         RozszerzeniaZnacznik::Webp => ("Webp",
//                                        RozszerzeniaZnacznik::Webp
//         ),
//         RozszerzeniaZnacznik::Tga=> ("Tga",
//                                      RozszerzeniaZnacznik::Tga
//         ),
//         RozszerzeniaZnacznik::Ff => ("FF",
//                                      RozszerzeniaZnacznik::Ff
//         ),
//         RozszerzeniaZnacznik::Qoi=> ("Qoi",
//                                      RozszerzeniaZnacznik::Qoi
//         ),
//         RozszerzeniaZnacznik::Avif => ("Avif",
//                                        RozszerzeniaZnacznik::Avif)
//
//     };
//     button(
//         text(
//             nazwa)
//             .color(Color::from_rgba(1., 1., 1., 0.6))
//             .font(font)
//             .height(WYSOKOSC_CZCIONEK_PRZYCISKI)
//             .width(Length::Fill)
//             .center(),
//     )
//         .on_press(Message::ŁączenieZdjęć(ŁączenieZdjęćMessage::ZdjeciaLaczenieZmianaWybraneRozszerzenie(msg.clone()),
//         ))
//         .style(styl_przycisków(
//             false,
//             co_istnieje.tag == msg,
//             kolor, temat,
//         )).into()
// }