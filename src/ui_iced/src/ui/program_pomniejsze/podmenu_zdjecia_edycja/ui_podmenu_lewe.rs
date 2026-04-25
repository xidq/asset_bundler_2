use crate::ui::program_pomniejsze::kolory::KOLOR_SPANISH_ORANGE;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN, WybraneOknoEdycjiZdjęć,
};
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{Column, Row, button, space, text};
use iced_core::{Color, Length};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use enumy::opcje::OptRozdzielczościObrazów;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::info_male;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_lewe_wybor(
    wybrane_okno: &WybraneOknoEdycjiZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    Column::new()
        .push(
            button(
                text(jezyk.t("foto_edit_menu_paths"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::Ścieżki,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::Ścieżki),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk rozszerzenia_plików_zdjęciowych
        .push(
            button(
                text(jezyk.t("foto_edit_menu_extensions"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(
                    wybrane_okno,
                    WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych
                ),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk OptRozdzielczościObrazów
        .push(
            button(
                text(jezyk.t("foto_edit_menu_resolution"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::MenuOptRozdzielczościObrazów,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(
                    wybrane_okno,
                    WybraneOknoEdycjiZdjęć::MenuOptRozdzielczościObrazów
                ),
                KOLOR_SPANISH_ORANGE,
            )),
        )
        // Przycisk inne
        .push(
            button(
                text(jezyk.t("foto_edit_menu_rest"))
                    .font(jezyk.get_font())
                    .center(),
            )
            .on_press(Message::ZmienMenuEdycjiNaSciezki(
                WybraneOknoEdycjiZdjęć::MenuReszta,
            ))
            .padding(12)
            .height(Length::Fixed(40.))
            .width(Length::Fill)
            .style(styl_przycisków(
                false,
                matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::MenuReszta),
                KOLOR_SPANISH_ORANGE,
            )),
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
