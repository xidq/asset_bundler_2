use crate::foty::zmiana_fot::{EdycjaZdjęć, Interpolacja, ObrazTGA, Zaszumianie};
use crate::foty::zmiana_fot::{Obraz, Rozdzielczości, Rozszerzenia};
use crate::ui::program::czcionki::KOLORTŁA;
use crate::ui::program::{LogPrzetwarzanieFot, Message, StanKlikaczaDoEdycjiZdjec, WybórJęzyka};
use crate::{styl_checkbox, styl_menu_pick, styl_pick_list, styl_przycisków, styl_text_input};
use iced::widget::{button, checkbox, column, container, grid, pick_list, progress_bar, row, scrollable, slider, space, text, text_input, tooltip, Column, Grid, Row};
use iced::{Border, Color, Element, Length, };
// use iced_core::Background::Gradient;
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians,Shadow, Vector};
use crate::ui::program::ui_zdjecia_edycja::KOLORSPANISHORANGE;
use crate::ui::program::style_fn::btn::styl_przycisków;

pub fn podmenu_jpg_wybor_top(
    dane: &EdycjaZdjęć,
    stan_klikaczy: &StanKlikaczaDoEdycjiZdjec,
    jezyk: &WybórJęzyka,
) -> Column<'static, Message>{

    Column::new()
        .push(if stan_klikaczy.jpg_wybrany{
            Row::new()
                .push(
                    button(
                        text("jpg")
                            .font(jezyk.get_font())
                            .width(Length::Fill)
                            .center()
                    )
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaWybranyJpg)
                        .style(styl_przycisków(false,stan_klikaczy.jpg_wybrany,KOLORSPANISHORANGE))
                        .width(Length::FillPortion(5)),
                )
                .push(space().width(Length::FillPortion(1)),)
                .push(
                    Column::new()
                        .push(
                            slider(0..=100,match dane.rozszerzenia[0] {
                                Rozszerzenia::Jpg { jakosc, .. } => jakosc,
                                _ => 0
                            },
                                   Message::ZdjeciaEdycjaZmianaJakosciJpg)
                                .style(move |_theme, _status| slider::Style {
                                    rail: slider::Rail {
                                        backgrounds: (
                                            Color::from_rgba(KOLORSPANISHORANGE.0, KOLORSPANISHORANGE.1, KOLORSPANISHORANGE.2, 0.4).into(), // Aktywne (lewo) - Czerwony
                                            Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                        ),
                                        width: 4.,
                                        border: Border{
                                            color: Color::from_rgba(KOLORSPANISHORANGE.0, KOLORSPANISHORANGE.1, KOLORSPANISHORANGE.2, 0.2),width: 1.0 , radius: 5.0.into(),},
                                    },
                                    handle: slider::Handle {
                                        shape: slider::HandleShape::Circle { radius: 8.0 },
                                        background: Color::from_rgb(KOLORSPANISHORANGE.0, KOLORSPANISHORANGE.1, KOLORSPANISHORANGE.2).into(), // Czerwona kropka
                                        border_width: 1.0,
                                        border_color: Color::BLACK,
                                    },
                                }
                                ),
                        )
                        .push(
                            text(
                                format!(
                                    "{} {}%",
                                    jezyk.t("foto_edit_quality"),
                                    match dane.rozszerzenia[0] {
                                        Rozszerzenia::Jpg { jakosc, .. } => jakosc, _ => 0
                                    }
                                )
                            )
                                .color(Color::from_rgba(1.,1.,1.,0.5))
                                .font(jezyk.get_font())
                        ).width(Length::FillPortion(5)),
                )//push column qniec
                .push(space().width(Length::FillPortion(1)),)
                .push(
                    tooltip(
                        if stan_klikaczy.jpg_wybrany{
                            button(text("Prog.").font(jezyk.get_font()).width(Length::Fill).center())
                                .padding(10)
                                .on_press(Message::ZdjeciaEdycjaZmianaProgresJpg)
                                .style(styl_przycisków(false,stan_klikaczy.jpg_progres,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                        }else{
                            button(text("Prog.").font(jezyk.get_font()).width(Length::Fill).center())
                                .padding(10)
                                .style(styl_przycisków(false,stan_klikaczy.jpg_progres,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                        },
                        "Zapis progresywny on/off",
                        tooltip::Position::Top
                    ),
                )
                .push(space().width(Length::FillPortion(1)),)

        }else{
            Row::new()
                .push(button(text("jpg").font(jezyk.get_font()).width(Length::Fill).center())
                          .padding(10)
                          .on_press(Message::ZdjeciaEdycjaZmianaWybranyJpg)
                          .style(styl_przycisków(false,stan_klikaczy.jpg_wybrany,KOLORSPANISHORANGE))
                          .width(Length::FillPortion(5)),)
                .push(space().width(Length::FillPortion(13)),)

        }
    )




        // MENU Z WYBORAMI
        // DRUGI ROW




    .push(
        if stan_klikaczy.jpg_wybrany{
            Row::new()
                .push(tooltip(
                    button(text(jezyk.t("foto_edit_color").to_string()).font(jezyk.get_font()).width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaKolorJpg(Obraz::B8))
                        .style(styl_przycisków(false,stan_klikaczy.jpg_wybrany_rgb,KOLORSPANISHORANGE))
                        .width(Length::FillPortion(5))
                        .height(Length::Fixed(50.)),
                    text(jezyk.t("foto_edit_tooltip_jpg_color").to_string()),
                    tooltip::Position::Top
                ),)

                .push(space().width(Length::Fixed(5.)),)
                .push(container("")
                          .width(Length::Fixed(2.))
                          .height(Length::Fixed(50.))
                          .style(move |_theme| container::Style{
                              text_color: None,
                              background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                              border: Default::default()
                              ,shadow: Default::default()
                              ,snap: false,}),)
                .push(space().width(Length::Fixed(5.)),)

                .push(tooltip(
                    button(text("bw").font(jezyk.get_font()).width(Length::Fill).center())
                        .padding(10)
                        .on_press(Message::ZdjeciaEdycjaZmianaKolorJpg(Obraz::L8))
                        .style(styl_przycisków(false,stan_klikaczy.jpg_wybrany_bw,KOLORSPANISHORANGE))
                        .width(Length::FillPortion(5))
                        .height(Length::Fixed(50.)),
                    text(jezyk.t("foto_edit_tooltip_jpg_bw").to_string()),
                    tooltip::Position::Top
                ),)

        }else{
            Row::new()
        }
    )
    .spacing(15) //oesu ale to długie... a tyle krwi napsuło...
    .padding(15)
    .width(Length::FillPortion(2))
}

