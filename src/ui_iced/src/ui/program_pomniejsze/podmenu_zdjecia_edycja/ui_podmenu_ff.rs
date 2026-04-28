use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptMetodaKompresjiZdjecia, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, pick_list, slider, space, text, Column, Row};
use iced::{Color, Length};
use enumy::inne_ui::{CheckerDoZbiorowePrzetwarzanieZdjęć, RodzajeContainer};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_ogolny, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::ŁączenieZdjęćMessage;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn podmenu_ff_wybor(
    dane: &DaneDoBathKonwersjaZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {


    let opcje = vec![
        OptMetodaKompresjiZdjecia::Brak,
        OptMetodaKompresjiZdjecia::Zstd(3),
        OptMetodaKompresjiZdjecia::Bzip2(6),
        OptMetodaKompresjiZdjecia::Xz(6),
    ];
    let wybrana= Some(OptMetodaKompresjiZdjecia::Brak);

    Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff, dane, jezyk.get_font()))
        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Ff {
                            metoda_kompresji
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Ff { .. }))
        {
            container(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                pick_list(opcje, Some(*metoda_kompresji), |hh|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaFF(hh)))
                                    .width(Length::FillPortion(5))
                                    .padding(2)
                                    .text_line_height(1.5)
                                    .style(styl_pick_list(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
                                    .menu_style(styl_menu_pick(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
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
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaWartoscFF(xx)),
                                            )
                                                .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
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
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaWartoscFF(xx)),
                                            )
                                                .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
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
                                                |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjaWartoscFF(xx)),                                            )
                                                .style(styl_sliderów(KOLOR_PEACH_PUFF)).height(20.).width(Length::FillPortion(6)),
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
            ).height(100.).style(styl_kontenera(true, KOLOR_SPANISH_ORANGE,RodzajeContainer::Góra))

        }else {container(Column::new())}).padding(15).width(Length::FillPortion(2))

    // MENU Z WYBORAMI
    // DRUGI ROW
}

pub fn podmenu_ff_misc<'a>(
    dane:&DaneDoBathKonwersjaZdjec,
) -> Row<'a, Message> {
    let ff_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Ff { .. })
    });

    // 2. Wyciągnięcie nazwy metody i jej poziomu (wartości)
    let (nazwa_metody, wartosc_metody) = match ff_data {
        Some(OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji }) => {
            match metoda_kompresji {
                OptMetodaKompresjiZdjecia::Zstd(v)  => ("Zstd".to_string(), v.to_string()),
                OptMetodaKompresjiZdjecia::Bzip2(v) => ("Bzip2".to_string(), v.to_string()),
                OptMetodaKompresjiZdjecia::Xz(v)    => ("Xz".to_string(), v.to_string()),
                OptMetodaKompresjiZdjecia::Brak     => ("Brak".to_string(), "-".to_string()),
            }
        }
        _ => ("-".to_string(), "-".to_string()),
    };
    let jest_aktywny_ff = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Ff);

    Row::new()
        .push(info_male("Ff".to_string(),jest_aktywny_ff))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male(nazwa_metody + ": " + &*wartosc_metody,jest_aktywny_ff))
}
