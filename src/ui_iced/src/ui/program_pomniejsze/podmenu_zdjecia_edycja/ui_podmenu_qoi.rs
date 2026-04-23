use crate::ui::program_pomniejsze::kolory::KOLOR_SPANISH_ORANGE;
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::{
    PRZERWAWYBRANYCHROZSZERZEN, ROZMIARWYBRANYCHROZSZERZEN,
};
use enumy::opcje::OptFormatyKoloruObrazuQoi;
use enumy::wybranie_jezykowe::WybórJęzyka;
use iced::widget::{button, container, space, text, Column, Row};
use iced_core::{Color, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub fn podmenu_qoi_wybor(
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message> {
    let space_val = if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany {15} else {0};
    Column::new()
        .push(
            Row::new()
                .push(
                    button(
                        text("qoi")
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center(),
                    )
                    .padding(10)
                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyQoi)
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5)),
                )
                .push(space().width(Length::FillPortion(13))),
        )
        .push(if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany {
            Row::new()
                // row![
                .push(
                    button(
                        text("Color")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthQoi(
                        OptFormatyKoloruObrazuQoi::Color24,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_24b,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
                .push(space().width(Length::Fixed(5.)))
                .push(
                    container("")
                        .width(Length::Fixed(2.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style {
                            text_color: None,
                            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.2).into()),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: false,
                        }),
                )
                .push(space().width(Length::Fixed(5.)))
                .push(
                    button(
                        text("ColorA")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthQoi(
                        OptFormatyKoloruObrazuQoi::ColorA32,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_32b,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .width(Length::FillPortion(5))
                    .height(Length::Fixed(50.)),
                )
        } else {
            Row::new().push(space())
        })
        .spacing(space_val) //oesu ale to długie... a tyle krwi napsuło...
        .padding(15)
        .width(Length::FillPortion(2))
}

pub fn podmenu_qoi_misc(
    stan_klikaczy: &CheckerDoZbiorowePrzetwarzanieZdjęć,
) -> Row<'static, Message> {
    Row::new()
        .push(
            text("Qoi")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany { 0.5 } else { 0.2 },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("|")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(1., 1., 1., 0.3))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("24b")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_24b && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
        .push(
            text("32a")
                .font(iced::Font {
                    family: iced::font::Family::Name("VT323"),
                    ..Default::default()
                })
                .color(Color::from_rgba(
                    1.,
                    1.,
                    1.,
                    if stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_32b && stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany {
                        0.5
                    } else {
                        0.2
                    },
                ))
                .size(ROZMIARWYBRANYCHROZSZERZEN),
        )
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)))
}
