use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, slider, space, text, tooltip, Column, Row};
use iced_core::{Color, Length};
use enumy::inne_ui::{CheckerDoZbiorowePrzetwarzanieZdjęć, RodzajeContainer};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_ogolny, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn podmenu_webp_wybor(
    dane: &DaneDoBathKonwersjaZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp, dane, jezyk.get_font()))

        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp {
                            jakosc,
                            lossless,
                            bit_depth,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Webp { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=100,
                                        *jakosc,
                                        |vv|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaJakosciWebp(vv))
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
                                        btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp,OptFormatyKoloruObrazOgólny::B8,dane,jezyk.get_font()),
                                        text("rgb".to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .push(
                                    tooltip(
                                        btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp,OptFormatyKoloruObrazOgólny::B8a,dane,jezyk.get_font()),
                                        text("rgba".to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            tooltip(
                                                button(
                                                    text("Lossless")
                                                        .font(jezyk.get_font())
                                                        .width(Length::Fill)
                                                        .height(Length::Fill)
                                                        .center(),
                                                )
                                                    .padding(10)
                                                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaLosslessWebp))
                                                    .style(styl_przycisków(
                                                        false,
                                                        *lossless,
                                                        KOLOR_SPANISH_ORANGE,
                                                    ))
                                                    .width(Length::FillPortion(1)),

                                                "Zapis lossless on/off",
                                                tooltip::Position::Top,
                                            )
                                        )
                                )
                        )
                ).height(100.).style(styl_kontenera(true, KOLOR_SPANISH_ORANGE,RodzajeContainer::Góra))
            } else {
                container(Row::new())
            }
        )
        .push(
            Row::new()

        ) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_webp_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
) -> Row<'a, Message> {
    let webp_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Webp{ .. })
    });

    let lossless_bool = || {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp { lossless, .. }) = webp_data {
            *lossless // zwracamy wartość bool
        } else {
            false
        }
    };
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazOgólny| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Webp { bit_depth, .. }) = webp_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    // 3. Przygotowanie jakości
    let jakosc_str = webp_data
        .and_then(|f| if let OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc, .. } = f { Some(jakosc.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());

    let jest_aktywny_webp = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Webp);
    
    Row::new()
        .push(info_male("Webp".to_string(),jest_aktywny_webp))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("RGB".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("RGBa".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B8a)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male(jakosc_str,jest_aktywny_webp))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Lossless".to_string(),lossless_bool()))
}
