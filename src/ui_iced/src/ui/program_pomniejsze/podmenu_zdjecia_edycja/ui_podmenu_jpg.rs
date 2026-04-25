use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row, button, container, slider, space, text, tooltip};
use iced::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_ogolny, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn podmenu_jpg_wybor_top<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'a, Message> {
     Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg, dane, jezyk.get_font()))

        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg {
                            jakosc,
                            progresywny,
                            bit_depth,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. }))
                {
                container(
                    Column::new()
                    .push(
                        Row::new()
                            .push(
                                slider(
                                    0..=100,
                                    *jakosc,
                                    |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJakosciJpg(vv))
                                )
                                    .height(20.)
                                    .width(Length::FillPortion(6))
                                .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
                            )
                            .push(
                                text(format!(
                                    "Q: {}%",
                                    jakosc
                                ))
                                .color(KOLOR_CZCIONKI_SREDNI)
                                .font(jezyk.get_font()).width(Length::FillPortion(4)).height(Length::Fill).center(),
                            )
                            .padding(15)
                    )
                    .push(
                        Row::new()

                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,OptFormatyKoloruObrazOgólny::B8,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(
                            tooltip(
                                btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,OptFormatyKoloruObrazOgólny::L8,dane,jezyk.get_font()),
                                text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                tooltip::Position::Top,
                            )
                        )
                        .push(
                            Row::new()
                                .push(
                                    tooltip(
                                        button(
                                            text("Prog.")
                                                .font(jezyk.get_font())
                                                .width(Length::Fill)
                                                .height(Length::Fill)
                                                .center(),
                                        )
                                            .padding(10)
                                            .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaProgresJpg))
                                            .style(styl_przycisków(
                                                false,
                                                *progresywny,
                                                KOLOR_SPANISH_ORANGE,
                                            ))
                                            .width(Length::FillPortion(1)),

                                        "Zapis progresywny on/off",
                                        tooltip::Position::Top,
                                    )
                                )
                        )
                    )
                ).height(100.).style(styl_kontenera(true, KOLOR_SPANISH_ORANGE))
            } else {
                    container(Row::new())
            }
        )
        // MENU Z WYBORAMI
        // DRUGI ROW
        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_jpg_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
) -> Row<'a, Message> {
    let jpg_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Jpg { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazOgólny| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { bit_depth, .. }) = jpg_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };
    let prog_bool = || {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Jpg { progresywny, .. }) = jpg_data {
            *progresywny // zwracamy wartość bool
        } else {
            false
        }
    };

    // 3. Przygotowanie jakości
    let jakosc_str = jpg_data
        .and_then(|f| if let OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, .. } = f { Some(jakosc.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());

    let jest_aktywny_jpg = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg);


    Row::new()
        .push(info_male("Jpg".to_string(),jest_aktywny_jpg))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("C".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("BW".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::L8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male(jakosc_str,jest_aktywny_jpg))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Progresywny".to_string(),prog_bool()))
}
