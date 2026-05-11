use iced::widget::{container, slider, space, text, Column, Row};
use iced_core::{Border, Color, Length};
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::inne_ui::{SliderType, UstawieniaThemeWsio};
use enumy::opcje::OptInterpolacja;
use enumy::wybranie_jezykowe::WybórJęzyka;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use crate::widget::button::pole_tekstowe_przycisku;
use crate::widget::colors_n_stuff::KOLOR_CZCIONKI_SREDNI;
use crate::widget::dropdown::dropdown;
use crate::widget::slajder::slajderr;

pub fn reszta<'a>(dane: &'a DaneKonw, kolor: &'a Color, jezyk: &'a WybórJęzyka, temat: &'a UstawieniaThemeWsio) -> Column<'a, Message> {
    Column::new().padding(15).spacing(15)
        .push(pole_tekstowe_przycisku("conversion_alpha_color", jezyk, temat))
        .push(
            Row::new()
                .push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.0, |v| {
                                        Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::WypełnienieAlpha(0, v))
                                    })
                                        .style(
                                            move |_theme, _status| slider::Style {
                                                rail: slider::Rail {
                                                    backgrounds: (
                                                        Color::from_rgb(
                                                            dane.alfa_rgb.0 as f32 / 65535. / 2.,
                                                            dane.alfa_rgb.0 as f32 / 65535. / 5.,
                                                            dane.alfa_rgb.0 as f32 / 65535. / 5.,
                                                        )
                                                            .into(), // Aktywne (lewo) - Czerwony
                                                        Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                    ),
                                                    width: 4.,
                                                    border: Border {
                                                        color: Color::BLACK,
                                                        width: 1.0,
                                                        radius: 5.0.into(),
                                                    },
                                                },
                                                handle: slider::Handle {
                                                    shape: slider::HandleShape::Circle { radius: 8.0 },
                                                    background: Color::from_rgb(0.75, 0.2, 0.2).into(), // Czerwona kropka
                                                    border_width: 1.0,
                                                    border_color: Color::BLACK,
                                                },
                                            },
                                        ),
                                )
                                .push(
                                    text(format!(
                                        "R: {} | {}",
                                        dane.alfa_rgb.0,
                                        (dane.alfa_rgb.0 as f32 / 65535. * 255.).round() as u8
                                    ))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font())
                                        .width(Length::Fixed(130.))
                                        .center(),
                                ),
                        )
                        .push(
                            Row::new()
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.1, |v| {
                                        Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::WypełnienieAlpha(1, v))
                                    })
                                        .style(
                                            move |_theme, _status| slider::Style {
                                                rail: slider::Rail {
                                                    backgrounds: (
                                                        Color::from_rgb(
                                                            dane.alfa_rgb.1 as f32 / 65535. / 5.,
                                                            dane.alfa_rgb.1 as f32 / 65535. / 2.,
                                                            dane.alfa_rgb.1 as f32 / 65535. / 5.,
                                                        )
                                                            .into(), // Aktywne (lewo) - Czerwony
                                                        Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                    ),
                                                    width: 4.,
                                                    border: Border {
                                                        color: Color::BLACK,
                                                        width: 1.0,
                                                        radius: 5.0.into(),
                                                    },
                                                },
                                                handle: slider::Handle {
                                                    shape: slider::HandleShape::Circle { radius: 8.0 },
                                                    background: Color::from_rgb(0.2, 0.75, 0.2).into(), // Czerwona kropka
                                                    border_width: 1.0,
                                                    border_color: Color::BLACK,
                                                },
                                            },
                                        ),
                                )
                                .push(
                                    text(format!(
                                        "G: {} | {}",
                                        dane.alfa_rgb.1,
                                        (dane.alfa_rgb.1 as f32 / 65535. * 255.).round() as u8
                                    ))
                                        .font(jezyk.get_font())
                                        .width(Length::Fixed(130.))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .center(),
                                ),
                        )
                        .push(
                            Row::new()
                                // Slider dla B (indeks 2)
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.2, |v| {
                                        Message::ZbiorowePrzetwarzanieZdjęć(KonwMsg::WypełnienieAlpha(2, v))
                                    })
                                        .style(
                                            move |_theme, _status| slider::Style {
                                                rail: slider::Rail {
                                                    backgrounds: (
                                                        Color::from_rgb(
                                                            dane.alfa_rgb.2 as f32 / 65535. / 5.,
                                                            dane.alfa_rgb.2 as f32 / 65535. / 5.,
                                                            dane.alfa_rgb.2 as f32 / 65535. / 2.,
                                                        )
                                                            .into(), // Aktywne (lewo) - Czerwony
                                                        Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                    ),
                                                    width: 4.,
                                                    border: Border {
                                                        color: Color::BLACK,
                                                        width: 1.0,
                                                        radius: 5.0.into(),
                                                    },
                                                },
                                                handle: slider::Handle {
                                                    shape: slider::HandleShape::Circle { radius: 8.0 },
                                                    background: Color::from_rgb(0.2, 0.2, 0.75).into(), // Czerwona kropka
                                                    border_width: 1.0,
                                                    border_color: Color::BLACK,
                                                },
                                            },
                                        ),
                                )
                                .push(
                                    text(format!(
                                        "B: {} | {}",
                                        dane.alfa_rgb.2,
                                        (dane.alfa_rgb.2 as f32 / 65535. * 255.).round() as u8
                                    ))
                                        .color(KOLOR_CZCIONKI_SREDNI)
                                        .font(jezyk.get_font())
                                        .width(Length::Fixed(130.))
                                        .center(),
                                ),
                        ),
                )
                .push(space().width(Length::Fixed(15.)))
                .push(
                    container("")
                        .width(Length::Fixed(15.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style {
                            text_color: None,
                            background: Some(
                                Color::from_rgb(
                                    dane.alfa_rgb.0 as f32 / 65535.,
                                    dane.alfa_rgb.1 as f32 / 65535.,
                                    dane.alfa_rgb.2 as f32 / 65535.,
                                )
                                    .into(),
                            ),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: false,
                        }),
                )
                // let kolor_szary = (dane.alfa_rgb.0 as f32 / 65535.) * (dane.alfa_rgb.1 as f32 / 65535.) *(dane.alfa_rgb.2 as f32 / 65535.) / 3.;
                .push(
                    container("")
                        .width(Length::Fixed(15.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style {
                            text_color: None,
                            background: Some(
                                Color::from_rgb(
                                    (dane.alfa_rgb.0 as f32 / 65535.
                                        + dane.alfa_rgb.1 as f32 / 65535.
                                        + dane.alfa_rgb.2 as f32 / 65535.)
                                        / 3_f32,
                                    (dane.alfa_rgb.0 as f32 / 65535.
                                        + dane.alfa_rgb.1 as f32 / 65535.
                                        + dane.alfa_rgb.2 as f32 / 65535.)
                                        / 3_f32,
                                    (dane.alfa_rgb.0 as f32 / 65535.
                                        + dane.alfa_rgb.1 as f32 / 65535.
                                        + dane.alfa_rgb.2 as f32 / 65535.)
                                        / 3_f32,
                                )
                                    .into(),
                            ),
                            border: Default::default(),
                            shadow: Default::default(),
                            snap: false,
                        }),
                )
        )
        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku("mgt_interpolation", jezyk, temat))
                .push(dropdown::<OptInterpolacja, _>(dane, kolor, temat))
                .push(space().width(15.))
        )
        .push(
            Row::new().spacing(15)
                .push(pole_tekstowe_przycisku(match dane.noising {
                    Some(x) => format!("{}\n{}%", jezyk.t("conversion_noising"), x),
                    None => format!(
                        "{}\n{}",
                        jezyk.t("conversion_noising"),
                        jezyk.t("mgt_gen_off")
                    ),
                }, jezyk, temat))
                .push(slajderr(dane.noising.unwrap_or(0) as i32, (0,100), &SliderType::KonwersjaNoising, kolor, temat, Length::FillPortion(2)  ))
                .push(space().width(15.))
        )


}
