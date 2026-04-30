use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::info_male;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::hint_master::hint_btn;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN,
};
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{UiPodstrony, UstawieniaThemeWsio};
use enumy::opcje::OptRozdzielczościObrazów;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, space, text, Column, Row};
use iced_core::Length;

pub fn podmenu_lewe_wybor<'a>(
    jezyk: &'a WybórJęzyka,
    kolor: &'a iced::Color,
    temat: &'a UstawieniaThemeWsio,
) -> Column<'a, Message> {
    Column::new()
        .push(
            hint_btn(
                button(
                    text(jezyk.t("ui_conversion_paths"))
                        .font(jezyk.get_font())
                        .center(),
                )
                .on_press(Message::ZmienWariant(
                    UiPodstrony::KonwersjaFotoŚcieżki,
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    false,
                    matches!(temat.temp.aktywne_okno, UiPodstrony::KonwersjaFotoŚcieżki),
                    kolor,temat,
                )),
                jezyk.t("hint_ui_conversion_paths"),
                
                temat
            )
        )
        // Przycisk rozszerzenia_plików_zdjęciowych
        .push(
            hint_btn(
                button(
                    text(jezyk.t("ui_conversion_extensions"))
                        .font(jezyk.get_font())
                        .center(),
                )
                .on_press(Message::ZmienWariant(
                    UiPodstrony::KonwersjaFotoRozszerzenia,
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    false,
                    matches!(
                        temat.temp.aktywne_okno,
                        UiPodstrony::KonwersjaFotoRozszerzenia
                    ),
                    kolor,temat,
                )),
                jezyk.t("hint_ui_conversion_extensions"),
                
                temat
            )
        )
        // Przycisk OptRozdzielczościObrazów
        .push(
            hint_btn(
                button(
                    text(jezyk.t("ui_conversion_resolutions"))
                        .font(jezyk.get_font())
                        .center(),
                )
                .on_press(Message::ZmienWariant(
                    UiPodstrony::KonwersjaFotoRozdzielczości,
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    false,
                    matches!(
                        temat.temp.aktywne_okno,
                        UiPodstrony::KonwersjaFotoRozdzielczości
                    ),
                    kolor,temat,
                )),
                jezyk.t("hint_ui_conversion_resolutions"),
                
                temat
            )
        )
        // Przycisk inne
        .push(
            hint_btn(
                button(
                    text(jezyk.t("ui_conversion_rest"))
                        .font(jezyk.get_font())
                        .center(),
                )
                .on_press(Message::ZmienWariant(
                    UiPodstrony::KonwersjaFotoMenuReszta,
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    false,
                    matches!(temat.temp.aktywne_okno, UiPodstrony::KonwersjaFotoMenuReszta),
                    kolor,temat,
                )),
                jezyk.t("hint_ui_conversion_rest"),
                
                temat
            )
        )
        .spacing(15)
        .width(Length::FillPortion(1))
}

pub fn podmenu_lewe_rozdzielczosci<'a>(
    dane:&DaneDoBathKonwersjaZdjec,
) -> Row<'a, Message> {
    let bool_wybrane = |bb|{ dane.opcje_rozdzielczości.contains(&bb) };
    Row::new()
        .push(info_male("16".to_string(),bool_wybrane(OptRozdzielczościObrazów::R16)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("32".to_string(),bool_wybrane(OptRozdzielczościObrazów::R32)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("64".to_string(),bool_wybrane(OptRozdzielczościObrazów::R64)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("128".to_string(),bool_wybrane(OptRozdzielczościObrazów::R128)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("256".to_string(),bool_wybrane(OptRozdzielczościObrazów::R256)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("512".to_string(),bool_wybrane(OptRozdzielczościObrazów::R512)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("1k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R1k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("2k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R2k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("4k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R4k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("6k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R6k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("8k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R8k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("16k".to_string(),bool_wybrane(OptRozdzielczościObrazów::R16k)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(info_male("Org".to_string(),bool_wybrane(OptRozdzielczościObrazów::Oryginalna)))
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
}
