use crate::ui::podmenu::fn_ogolne::info_male;
use crate::ui::podmenu::style_fn::btn::styl_przycisków;
use crate::ui::podmenu::style_fn::hint_master::hint_btn;
use crate::ui::podmenu::konwersja::main_konwersja::{
    PRZERWAWYBRANYCHROZSZERZEN,
};
use crate::ui::wiadomosci::message_ui::Message;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::inne_ui::{UiPodstrony, UstawieniaThemeWsio};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, space, text, Column, Row};
use iced_core::Length;
use strum::IntoEnumIterator;
use enumy::rozszerzenia::bdepth::BdepthAvif;

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
                    "btn_id_batch_menu_paths"
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(
                    styl_przycisków(
                    "btn_id_batch_menu_paths",
                    None,
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
                    "btn_id_batch_menu_ext"
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    "btn_id_batch_menu_ext",
                    None,
                    kolor,
                    temat,
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
                    "btn_id_batch_menu_resolutions"
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    "btn_id_batch_menu_resolutions",
                    None,
                    kolor,
                    temat,
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
                    "btn_id_batch_menu_rest"
                ))
                .padding(12)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(
                    "btn_id_batch_menu_rest",
                    None,
                    kolor,
                    temat,
                )),
                jezyk.t("hint_ui_conversion_rest"),
                
                temat
            )
        )
        .spacing(15)
        .width(Length::FillPortion(1))
}

pub fn podmenu_lewe_rozdzielczosci(
    temat: &UstawieniaThemeWsio,
) -> Row<Message> {

    let mut row_rozszerzen = Row::new().spacing(3.);

    for wariant in Rozdzielczości::iter() {
        row_rozszerzen = row_rozszerzen.push(
            info_male(
                wariant.maly_wariant().to_string(),
                wariant.bath_konwersja_id(),
                temat
            )
        );
    }

    row_rozszerzen

}
