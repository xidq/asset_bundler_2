use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_qoi, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::opcje::{OptFormatyKoloruObrazuQoi, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, Column, Row};
use iced_core::Length;

pub fn podmenu_qoi_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi, dane, jezyk.get_font(),kolor,temat))

        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Qoi {
                            bit_depth:_,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Qoi { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(space()).height(Length::FillPortion(1))
                        )
                        .push(
                        Row::new()
                            .push(btn_zbiorowe_kolor_qoi(OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi,OptFormatyKoloruObrazuQoi::Color24,dane,jezyk.get_font(),kolor,temat))
                            .push(btn_zbiorowe_kolor_qoi(OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi,OptFormatyKoloruObrazuQoi::ColorA32,dane,jezyk.get_font(),kolor,temat))
                        )
                    
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra, kolor,temat))
            }else{container(Column::new())}
        ).padding(15).width(Length::FillPortion(2))
    
}

pub fn podmenu_qoi_misc(
    dane: &DaneDoBathKonwersjaZdjec,
) -> Row<'static, Message> {
    let qoi_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Qoi { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazuQoi| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth, .. }) = qoi_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    let jest_aktywny_jpg = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Qoi {});


    Row::new()
        .push(info_male("Qoi".to_string(),jest_aktywny_jpg))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Color24".to_string(),ma_kolor(OptFormatyKoloruObrazuQoi::Color24)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Color32".to_string(),ma_kolor(OptFormatyKoloruObrazuQoi::ColorA32)))
}
