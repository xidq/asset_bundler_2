use crate::ui::program_pomniejsze::kolory::KOLOR_CZCIONKI_SREDNI;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::{btn_zbiorowe_kolor_tga, btn_zbiorowe_rozszerzenia, info_male};
use crate::ui::program_pomniejsze::style_fn::kontener::styl_kontenera;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::PRZERWAWYBRANYCHROZSZERZEN;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{RodzajeContainer, UstawieniaThemeWsio};
use enumy::opcje::{OptFormatyKoloruObrazuTga, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychZnacznik};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{container, space, text, tooltip, Column, Row};
use iced_core::Length;

pub fn podmenu_tga_wybor<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(btn_zbiorowe_rozszerzenia(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga, dane, jezyk.get_font(),kolor,temat))

        .push(
            if let Some(OptRozszerzeniaPlikówZdjęciowych::Tga {
                            bit_depth: _,
                        }) = dane.rozszerzenia_plików_zdjęciowych
                .iter()
                .find(|f| matches!(f, OptRozszerzeniaPlikówZdjęciowych::Tga { .. }))
            {
                container(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    text("bez kompresji")
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font()).width(Length::Fill).height(Length::Fill).center(),
                                )
                                .height(Length::FillPortion(1)).padding(15)
                        )
                        .push(
                            Row::new()
                                .push(
                                    tooltip(
                                        btn_zbiorowe_kolor_tga(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga,OptFormatyKoloruObrazuTga::Szary8,dane,jezyk.get_font(),kolor,temat),
                                        text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .push(
                                    tooltip(
                                        btn_zbiorowe_kolor_tga(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga,OptFormatyKoloruObrazuTga::HighColor16,dane,jezyk.get_font(),kolor,temat),
                                        text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .push(
                                    tooltip(
                                        btn_zbiorowe_kolor_tga(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga,OptFormatyKoloruObrazuTga::TrueColor24,dane,jezyk.get_font(),kolor,temat),
                                        text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .push(
                                    tooltip(
                                        btn_zbiorowe_kolor_tga(OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga,OptFormatyKoloruObrazuTga::TrueColorA32,dane,jezyk.get_font(),kolor,temat),
                                        text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                                        tooltip::Position::Top,
                                    )
                                )
                                .height(Length::FillPortion(1))
                        )
                ).height(100.).style(styl_kontenera(true,RodzajeContainer::Góra,kolor,temat))
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

pub fn podmenu_tga_misc(
    dane: &DaneDoBathKonwersjaZdjec,
    // stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    let tga_data = dane.rozszerzenia_plików_zdjęciowych.iter().find(|f| {
        matches!(f, OptRozszerzeniaPlikówZdjęciowych::Tga { .. })
    });
    // 2. Pomocnicze sprawdzenie koloru
    let ma_kolor = |target_bit: OptFormatyKoloruObrazuTga| {
        if let Some(OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth, .. }) = tga_data {
            bit_depth.contains(&target_bit)
        } else {
            false
        }
    };

    let jest_aktywny_jpg = dane.tag.contains(&OptRozszerzeniaPlikówZdjęciowychZnacznik::Tga);


    Row::new()
        .push(info_male("Jpg".to_string(),jest_aktywny_jpg))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("|".to_string(),false))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("G8".to_string(),ma_kolor(OptFormatyKoloruObrazuTga::Szary8)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("HC16".to_string(),ma_kolor(OptFormatyKoloruObrazuTga::HighColor16)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("TC24".to_string(),ma_kolor(OptFormatyKoloruObrazuTga::TrueColor24)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("TC32".to_string(),ma_kolor(OptFormatyKoloruObrazuTga::TrueColorA32)))
}
