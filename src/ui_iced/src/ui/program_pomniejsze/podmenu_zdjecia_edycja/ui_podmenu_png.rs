use crate::ui::program_pomniejsze::kolory::{KOLOR_CZCIONKI_SREDNI, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, slider, space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_ogolny, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub fn podmenu_png_wybor<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'a, Message> {
    Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png, dane, jezyk.get_font()))
        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Png {
                            kompresja,
                            bit_depth,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Png { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(
                                        0..=9,
                                        *kompresja,
                                        |xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKompresjiPng(xx)),
                                    ).height(20.)
                                        .width(Length::FillPortion(6))
                                        .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
                                )
                                .push(
                                    text(format!(
                                        "C: {}",
                                        kompresja
                                    ))
                                        .font(jezyk.get_font())
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .height(Length::Fill)
                                        .width(Length::FillPortion(4))
                                        .center(),
                                ).padding(15)
                        )
                        .push(
                            Row::new()
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::B8,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::B8a,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::L8,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::L8a,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::B16,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::B16a,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::L16,dane,jezyk.get_font()))
                                .push(btn_zbiorowe_kolor_ogolny(OptRozszerzeniaPlikówZdjęciowychZnacznik::Png,OptFormatyKoloruObrazOgólny::L16a,dane,jezyk.get_font()))
                        )
                ).height(100.).style(styl_kontenera(true, KOLOR_SPANISH_ORANGE))
            } else {
                container(Column::new())
            }
            .width(Length::FillPortion(5)),
        )
        //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_png_misc<'a>(
    dane: &DaneDoBathKonwersjaZdjec,
) -> Row<'a, Message> {

    let png_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Png { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazOgólny| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Png { bit_depth, .. }) = png_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    // 3. Przygotowanie jakości
    let kompresja_str = png_data
        .and_then(|f| if let OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, .. } = f { Some(kompresja.to_string()) } else { None })
        .unwrap_or_else(|| "-".to_string());

    let jest_aktywny_png = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Png);


    Row::new()
        .push(info_male("Png".to_string(),jest_aktywny_png))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B8a)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::L8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::L8a)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B16)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::B16a)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::L16)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8".to_string(),ma_kolor(OptFormatyKoloruObrazOgólny::L16a)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Kompresja: ".to_string() + &*kompresja_str, jest_aktywny_png))

}
