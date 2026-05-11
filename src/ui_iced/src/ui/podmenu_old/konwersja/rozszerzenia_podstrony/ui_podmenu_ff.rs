use crate::ui::podmenu::fn_ogolne::{ info_male};
use crate::ui::podmenu::style_fn::kontener::styl_kontenera;
use crate::ui::podmenu::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::podmenu::style_fn::slider::styl_sliderów;
use crate::ui::podmenu::konwersja::main_konwersja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, pick_list, slider, space, text, Column, Row};
use iced::{Color, Length};
use strum::{EnumMessage, IntoEnumIterator};
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::kompresje::OptMetodaKompresjiZdjecia;
use enumy::rozszerzenia::rozszerzenia::{Rozszerzenia, RozszerzeniaZnacznik};
use crate::ui::podmenu::metody_do_wariantow::typy_rozszerzenia::rozszerzenie_konwersja;

pub fn podmenu_ff_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {


    let opcje = vec![
        OptMetodaKompresjiZdjecia::Brak,
        OptMetodaKompresjiZdjecia::Zstd(3),
        OptMetodaKompresjiZdjecia::Bzip2(6),
        OptMetodaKompresjiZdjecia::Xz(6),
    ];
    // let wybrana= Some(OptMetodaKompresjiZdjecia::Brak);

    Column::new()
        .push(rozszerzenie_konwersja( RozszerzeniaZnacznik::Ff, kolor, jezyk.get_font(), temat))
        .push(
            if let Some(Rozszerzenia::Ff {
                            metoda_kompresji
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, Rozszerzenia::Ff { .. }))
        {
            container(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                pick_list(OptMetodaKompresjiZdjecia::iter().collect::<Vec<_>>(), Some(*metoda_kompresji), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::FfKompresja(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(kolor,temat))
                                    .menu_style(styl_menu_pick(kolor,temat))
                                    .width(Length::FillPortion(4))
                            )
                            .push(space().width(Length::Fixed(15.)))
                            .push(
                            match metoda_kompresji{
                                OptMetodaKompresjiZdjecia::Zstd(bb) => {
                                    Row::new()
                                        .push(
                                            slider(
                                                1..=22,
                                                *bb,
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::FfKompresjaVal(xx)),
                                            )
                                                .style(styl_sliderów(kolor,temat)).height(20.).width(Length::FillPortion(6)),
                                        )
                                        .push(
                                            text(format!(
                                                "C: {}",
                                                bb
                                            ))
                                                .color(Color::from_rgba(1., 1., 1., 0.5))
                                                .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(20.)),
                                        ).height(Length::Fixed(20.)).width(Length::FillPortion(6))
                                }
                                OptMetodaKompresjiZdjecia::Bzip2(bb) => {
                                    Row::new()
                                        .push(
                                            slider(
                                                1..=9,
                                                *bb,
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::FfKompresjaVal(xx)),
                                            )
                                                .style(styl_sliderów(kolor,temat)).height(20.).width(Length::FillPortion(6)),
                                        )
                                        .push(
                                            text(format!(
                                                "C: {}",
                                                bb
                                            ))
                                                .color(Color::from_rgba(1., 1., 1., 0.5))
                                                .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(20.)),
                                        ).height(Length::Fixed(20.)).width(Length::FillPortion(6))
                                }
                                OptMetodaKompresjiZdjecia::Xz(bb) => {
                                    Row::new()
                                        .push(
                                            slider(
                                                1..=9,
                                                *bb,
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::FfKompresjaVal(xx)),                                            )
                                                .style(styl_sliderów(kolor,temat)).height(20.).width(Length::FillPortion(6)),
                                        )
                                        .push(
                                            text(format!(
                                                "C: {}",
                                                bb
                                            ))
                                                .color(Color::from_rgba(1., 1., 1., 0.5))
                                                .font(jezyk.get_font()).center().width(Length::FillPortion(4)).height(Length::Fixed(20.)),
                                        ).height(Length::Fixed(20.)).width(Length::FillPortion(6))
                                }
                                OptMetodaKompresjiZdjecia::Brak => {
                                    Row::new().width(Length::FillPortion(6))
                                }
                            }
                        ).padding(15)

                    )
                    .push(
                        Row::new().height(Length::Fixed(50.))
                    )
            ).height(100.).style(styl_kontenera(true, RodzajeContainer::Góra,kolor,temat))

        }else {container(Column::new())}).padding(15).width(Length::FillPortion(2))

    // MENU Z WYBORAMI
    // DRUGI ROW
}

pub fn podmenu_ff_misc<'a>(
    dane:&DaneDoBathKonwersjaZdjec,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    let ff_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, Rozszerzenia::Ff { .. })
    });

    // 2. Wyciągnięcie nazwy metody i jej poziomu (wartości)
    let (wartosc_metody) = match ff_data {
        Some(Rozszerzenia::Ff { metoda_kompresji }) => {
            match metoda_kompresji {
                OptMetodaKompresjiZdjecia::Zstd(v)  => v.to_string(),
                OptMetodaKompresjiZdjecia::Bzip2(v) => v.to_string(),
                OptMetodaKompresjiZdjecia::Xz(v)    => v.to_string(),
                OptMetodaKompresjiZdjecia::Brak     => "-".to_string(),
            }
        }
        _ => "-".to_string(),
    };


    let mut row_rozszerzen = Row::new().spacing(3.);

    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_ff_ext",temat));

    for wariant in OptMetodaKompresjiZdjecia::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.get_message().unwrap_or("brak danych").to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    // 4. Dodajemy elementy PO iteratorze (te dodatkowe info o jakości, kompresji itd.)
    row_rozszerzen = row_rozszerzen
        .push(info_male("|".to_string(), "btn_id_batch_ff_ext",temat))
        .push(info_male(wartosc_metody, "btn_id_batch_ff_ext",temat));

    Column::new()
        /*tekst*/.push(info_male("Avif".to_string(),"btn_id_batch_avif_ext",temat))
        .push(row_rozszerzen)
}
