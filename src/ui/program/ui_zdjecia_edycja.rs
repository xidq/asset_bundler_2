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
use crate::ui::program::podmenu_zdjecia_edycja::ui_podmenu_jpg::{ podmenu_jpg_wybor_top};

use crate::io::enums_structs_io::PoziomKompresji;

const ROZMIARWYBRANYCHROZSZERZEN: iced::Pixels = iced::Pixels(14.);
const PRZERWAWYBRANYCHROZSZERZEN:f32 = 3.;
pub const KOLORSPANISHORANGE: (f32, f32, f32) = (232. / 255., 97. / 255., 0.);
#[derive(Clone, Debug)]
pub enum WybraneOknoEdycjiZdjęć{
    Ścieżki,
    Rozszerzenia,
    MenuRozdzielczości,
    MenuReszta,
}

pub fn view_foto_change(
    dane:  EdycjaZdjęć,
    wybrane_okno:&WybraneOknoEdycjiZdjęć,
    jezyk: WybórJęzyka,
    wejście_check: (bool,bool),
    czy_wyjscie_te_same: bool,
    stan_klikaczy:StanKlikaczaDoEdycjiZdjec,
    czy_jest_proces_zaczety: bool,
    log:LogPrzetwarzanieFot,
    main_process_check:bool,
) -> Element<'_, Message> {

    let czy_sie_nada_na_wyslanie =
        (
            stan_klikaczy.rozdzielczości_wybrane_16 ||
            stan_klikaczy.rozdzielczości_wybrane_32 ||
            stan_klikaczy.rozdzielczości_wybrane_64 ||
            stan_klikaczy.rozdzielczości_wybrane_128 ||
            stan_klikaczy.rozdzielczości_wybrane_256 ||
            stan_klikaczy.rozdzielczości_wybrane_512 ||
            stan_klikaczy.rozdzielczości_wybrane_1k ||
            stan_klikaczy.rozdzielczości_wybrane_2k ||
            stan_klikaczy.rozdzielczości_wybrane_4k ||
            stan_klikaczy.rozdzielczości_wybrane_6k ||
            stan_klikaczy.rozdzielczości_wybrane_8k ||
            stan_klikaczy.rozdzielczości_wybrane_16k ||
            stan_klikaczy.rozdzielczości_wybrane_org
        ) && (
            dane.ścieżka_wejściowa.is_file() ||
            dane.ścieżka_wejściowa.is_dir()
        ) &&
            dane.ścieżka_wyjściowa.is_dir()
        && (
            stan_klikaczy.jpg_wybrany ||
            stan_klikaczy.png_wybrany ||
            stan_klikaczy.tga_wybrany
            )
        &&(
            stan_klikaczy.jpg_wybrany &&(stan_klikaczy.jpg_wybrany_rgb || stan_klikaczy.jpg_wybrany_bw) ||
                stan_klikaczy.png_wybrany &&(
                    stan_klikaczy.png_wybrane_8bit ||
                    stan_klikaczy.png_wybrane_16bit ||
                    stan_klikaczy.png_wybrane_8bita ||
                    stan_klikaczy.png_wybrane_16bita ||
                    stan_klikaczy.png_wybrane_l8bit ||
                    stan_klikaczy.png_wybrane_l8bita ||
                    stan_klikaczy.png_wybrane_l16bit ||
                    stan_klikaczy.png_wybrane_l16bita
                ) ||
                stan_klikaczy.tga_wybrany &&(stan_klikaczy.tga_wybrany_szary || stan_klikaczy.tga_wybrany_16b || stan_klikaczy.tga_wybrany_24b || stan_klikaczy.tga_wybrany_32b)

        );


    let opcje_interpolacja: Vec<String> = Interpolacja::WSIOINTERPOLACJI.iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();


    let menu_ścieżki=
        Column::new()


        .push(
            Row::new()
                .push(
                    match wejście_check.0 {
                        true => {
                            button("📄")
                                .padding(10)
                                .on_press(Message::WybierzPlikInFotoEdycjaPakowanie)
                                .style(styl_przycisków!(false,wejście_check.1,KOLORSPANISHORANGE))

                        },
                        false => {
                            button("🖼️")
                                .padding(10)
                                .on_press(Message::WybierzPlikInFotoEdycjaPakowanie)
                                .style(styl_przycisków!(false,wejście_check.1,KOLORSPANISHORANGE))
                        },

                    },
                )
                .push(space().width(Length::Fixed(15.)),)
                .push(
                    match wejście_check.1 {
                        true => {
                            button("📁")
                            .padding(10)
                            .on_press(Message::WybierzFolderInFotoEdycjaPakowanie)
                            .style(styl_przycisków!(false,wejście_check.0,KOLORSPANISHORANGE))
                        },
                        false => {
                            button("📂")
                            .padding(10)
                            .on_press(Message::WybierzFolderInFotoEdycjaPakowanie)
                            .style(styl_przycisków!(false,wejście_check.0,KOLORSPANISHORANGE))
                        }
                    },
                ),
        )
        .push(
            Row::new()
                .push(match wejście_check.1 {
                    true => {
                        text_input(
                            jezyk.t(""),
                            &dane.ścieżka_wejściowa.to_string_lossy()

                        ).font(jezyk.get_font()).padding(10).style(styl_text_input!(KOLORSPANISHORANGE,KOLORTŁA))
                    },
                    false => {
                        text_input(
                            jezyk.t(""),
                            &dane.ścieżka_wejściowa.to_string_lossy()
                        )
                        .padding(10).font(jezyk.get_font())
                        .on_input(Message::ZdjeciaZmienFolderInPathChanged).style(styl_text_input!(KOLORSPANISHORANGE,KOLORTŁA))
                    }
                    },
                ),
        )

        .push(
            button(
                text("Resetuj ścieżki")
                    .font(jezyk.get_font())
                    .width(Length::Fill)
                    .center()
            )
                .padding(10)
                .on_press(Message::ResetujStanWejsciowychSciezekEdycjaFoto)
                .style(styl_przycisków!(false,true,KOLORSPANISHORANGE)),
        )

        .push(
            checkbox(czy_wyjscie_te_same)
                .label("Ścieżka wejściowa będzie wyjściową")
                .on_toggle(Message::ZdjeciaZmienFolderOutPathTenSam)
                .style(styl_checkbox!(KOLORSPANISHORANGE,KOLORTŁA)),
        )

        .push(
            Row::new()
                .push(
                    match czy_wyjscie_te_same {
                        true => {
                            button("📁")
                            .padding(10)
                            .style(styl_przycisków!(false,czy_wyjscie_te_same,KOLORSPANISHORANGE))
                        },
                        false => {
                            button("📂")
                            .padding(10)
                            .on_press(Message::WybierzFolderOutFotoEdycjaPakowanie)
                            .style(styl_przycisków!(false,czy_wyjscie_te_same,KOLORSPANISHORANGE))
                        }
                    },
                )
                .push(
                    match czy_wyjscie_te_same {
                        true => {
                            text_input(
                                jezyk.t(""),
                                &dane.ścieżka_wyjściowa.to_string_lossy()

                            ).padding(10).font(jezyk.get_font()).style(styl_text_input!(KOLORSPANISHORANGE,KOLORTŁA))
                        },
                        false => {
                            text_input(
                                jezyk.t(""),
                                &dane.ścieżka_wyjściowa.to_string_lossy()
                            )
                            .padding(10).font(jezyk.get_font())
                            .on_input(Message::ZdjeciaZmienFolderOutPathChanged).style(styl_text_input!(KOLORSPANISHORANGE,KOLORTŁA))
                        }
                    }
                ),
        )
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));


    let menu_rozszerzenia =
        Column::new()
            .push(Row::new()
                .push(
                    Column::new()
                        // Slider dla R (indeks 0)
                        .push(
                            Row::new()
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.0, |v| Message::ZdjeciaEdycjaZmianaKolorPng(0, v))
                                                .style(move |_theme, _status| slider::Style {
                                            rail: slider::Rail {
                                                backgrounds: (
                                                        Color::from_rgb(dane.alfa_rgb.0 as f32 / 65535., 0.0, 0.0).into(), // Aktywne (lewo) - Czerwony
                                                        Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                    ),
                                                width: 4.,
                                                border: Border{
                                                color: Color::BLACK,width: 1.0 , radius: 5.0.into(),},
                                            },
                                            handle: slider::Handle {
                                                shape: slider::HandleShape::Circle { radius: 8.0 },
                                                background: Color::from_rgb(0.75, 0.0, 0.0).into(), // Czerwona kropka
                                                border_width: 1.0,
                                                border_color: Color::BLACK,
                                            },
                                        }
                                    ),
                                )
                                .push(text(format!("R: {} | {}",dane.alfa_rgb.0, (dane.alfa_rgb.0 as f32 / 65535. * 255.).round() as u8))
                                    .color(Color::from_rgba(1.,1.,1.,0.5))
                                    .font(jezyk.get_font()).width(Length::Fixed(130.)).center()),
                        )


                        .push(
                            Row::new()
                                .push(slider(0..=65535, dane.alfa_rgb.1, |v| Message::ZdjeciaEdycjaZmianaKolorPng(1, v))
                                            .style(move |_theme, _status| slider::Style {
                                                rail: slider::Rail {
                                                    backgrounds: (
                                                            Color::from_rgb(0.0, dane.alfa_rgb.1 as f32 / 65535., 0.0).into(), // Aktywne (lewo) - Czerwony
                                                            Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                        ),
                                                    width: 4.,
                                                    border: Border{
                                                    color: Color::BLACK,width: 1.0 , radius: 5.0.into(),},
                                                },
                                                handle: slider::Handle {
                                                    shape: slider::HandleShape::Circle { radius: 8.0 },
                                                    background: Color::from_rgb(0.0, 0.75, 0.0).into(), // Czerwona kropka
                                                    border_width: 1.0,
                                                    border_color: Color::BLACK,
                                                },
                                            }),
                                )

                                .push(text(format!("G: {} | {}",dane.alfa_rgb.1, (dane.alfa_rgb.1 as f32 / 65535. * 255.).round() as u8))
                                    .font(jezyk.get_font()).width(Length::Fixed(130.))
                                    .color(Color::from_rgba(1.,1.,1.,0.5)).center()),
                        )


                        .push(
                            Row::new()
                            // Slider dla B (indeks 2)
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.2, |v| Message::ZdjeciaEdycjaZmianaKolorPng(2, v))
                                                .style(move |_theme, _status| slider::Style {
                                            rail: slider::Rail {
                                                backgrounds: (
                                                        Color::from_rgb(0.0, 0.0, dane.alfa_rgb.2 as f32 / 65535.).into(), // Aktywne (lewo) - Czerwony
                                                        Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                    ),
                                                width: 4.,
                                                border: Border{
                                                color: Color::BLACK,width: 1.0 , radius: 5.0.into(),},
                                            },
                                            handle: slider::Handle {
                                                shape: slider::HandleShape::Circle { radius: 8.0 },
                                                background: Color::from_rgb(0.0, 0.0, 0.75).into(), // Czerwona kropka
                                                border_width: 1.0,
                                                border_color: Color::BLACK,
                                            },
                                        }
                                    ),
                                )
                                .push(
                                    text(
                                        format!(
                                            "B: {} | {}",
                                            dane.alfa_rgb.2,
                                            (dane.alfa_rgb.2 as f32 / 65535. * 255.).round() as u8)
                                        )
                                        .color(Color::from_rgba(1.,1.,1.,0.5))
                                        .font(jezyk.get_font())
                                        .width(Length::Fixed(130.))
                                        .center()
                                ),
                        ))


                    .push(space().width(Length::Fixed(15.)),)
                    .push(container("")
                        .width(Length::Fixed(15.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style{
                            text_color: None,
                            background: Some(Color::from_rgb(dane.alfa_rgb.0 as f32 / 65535.,dane.alfa_rgb.1 as f32 / 65535.,dane.alfa_rgb.2 as f32 / 65535.).into()),
                            border: Default::default()
                            ,shadow: Default::default()
                            ,snap: false,}
                    ),)
                    // let kolor_szary = (dane.alfa_rgb.0 as f32 / 65535.) * (dane.alfa_rgb.1 as f32 / 65535.) *(dane.alfa_rgb.2 as f32 / 65535.) / 3.;
                    .push(container("")
                        .width(Length::Fixed(15.))
                        .height(Length::Fixed(50.))
                        .style(move |_theme| container::Style{
                            text_color: None,
                            background: Some(Color::from_rgb((dane.alfa_rgb.0 as f32 / 65535. + dane.alfa_rgb.1 as f32 / 65535. + dane.alfa_rgb.2 as f32 / 65535.) / 3_f32,(dane.alfa_rgb.0 as f32 / 65535. + dane.alfa_rgb.1 as f32 / 65535. + dane.alfa_rgb.2 as f32 / 65535.) / 3_f32,(dane.alfa_rgb.0 as f32 / 65535. + dane.alfa_rgb.1 as f32 / 65535. + dane.alfa_rgb.2 as f32 / 65535.) / 3_f32).into()),
                            border: Default::default()
                            ,shadow: Default::default()
                            ,snap: false,}
                    ),),
                ).width(Length::FillPortion(5))

            .push(scrollable(
            Column::new()
                    .push(container("")
                        .width(Length::Fill)
                        .height(Length::Fixed(2.))
                        .style(|_theme| container::Style{
                            background: Some(Color::from_rgba(1.,1.,1.,0.2).into()),
                            ..container::Style::default()

                    }),)


                    .push(podmenu_jpg_wybor_top(&dane, &stan_klikaczy, &jezyk))


                    // .push()



                    .push(container("")
                    .width(Length::Fill)
                    .height(Length::Fixed(2.))
                    .style(|_theme| container::Style{
                        background: Some(Color::from_rgba(1.,1.,1.,0.2).into()),
                        ..container::Style::default()

                    }),)

                    .push(if stan_klikaczy.png_wybrany{
                        Row::new()
                            .push(
                                button(
                                    text("png")
                                        .font(jezyk.get_font())
                                        .width(Length::Fill)
                                        .center()
                                )
                                    .padding(10)
                                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyPng)
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrany,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5)),
                            )

                            .push(
                                space().width(Length::FillPortion(1)),
                            )

                            .push(Column::new()
                                .push(
                                    slider(
                                        0..=9,match dane.rozszerzenia[1] {
                                            Rozszerzenia::Png { kompresja, .. } => kompresja,
                                            _ => 0
                                        },Message::ZdjeciaEdycjaZmianaKompresjiPng)
                                            .style(move |_theme, _status| slider::Style {
                                                rail: slider::Rail {
                                                    backgrounds: (
                                                            Color::from_rgba(
                                                                KOLORSPANISHORANGE.0,
                                                                KOLORSPANISHORANGE.1,
                                                                KOLORSPANISHORANGE.2,
                                                                0.4).into(), // Aktywne (lewo) - Czerwony
                                                            Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                                                        ),
                                                    width: 4.,
                                                    border: Border{
                                                    color: Color::from_rgba(
                                                        KOLORSPANISHORANGE.0,
                                                        KOLORSPANISHORANGE.1,
                                                        KOLORSPANISHORANGE.2,
                                                        0.2
                                                    ),width: 1.0 , radius: 5.0.into(),},
                                                },
                                                handle: slider::Handle {
                                                    shape: slider::HandleShape::Circle { radius: 8.0 },
                                                    background: Color::from_rgb(
                                                        KOLORSPANISHORANGE.0,
                                                        KOLORSPANISHORANGE.1,
                                                        KOLORSPANISHORANGE.2
                                                    ).into(), // Czerwona kropka
                                                    border_width: 1.0,
                                                    border_color: Color::BLACK,
                                                },
                                        }),)
                                .push(
                                    text(
                                        format!("Kompresja: {}",match dane.rozszerzenia[1] {
                                            Rozszerzenia::Png { kompresja, .. } => kompresja, _ => 0}))
                                                .font(jezyk.get_font())
                                                .color(Color::from_rgba(1.,1.,1.,0.5))
                                                .width(Length::Fill)
                                                .center(),
                                ).width(Length::FillPortion(12))

                            )

                    }else{
                        Row::new()
                            .push(
                                button(
                                    text("png")
                                    .font(jezyk.get_font())
                                    .width(Length::Fill)
                                    .center()
                                )
                                    .padding(10)
                                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyPng)
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrany,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5)),)
                            .push(
                                space().width(Length::FillPortion(13))
                            )

                    }.width(Length::FillPortion(5)),)






                    .push(if stan_klikaczy.png_wybrany{
                        Row::new()
                            .push(
                                button(text("8-bit").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::B8))
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_8bit,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )

                            .push(space().width(Length::Fixed(5.)),)
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                button(
                                    text("Luma8")
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .font(jezyk.get_font())
                                        .center()
                                )
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::L8))
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_l8bit,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )
                            .push(
                                    space().width(Length::Fixed(5.)),
                            )
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                button(
                                    text("16-bit")
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .font(jezyk.get_font())
                                        .center()
                                )
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::B16))
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_16bit,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )

                            .push(
                                button(
                                    text("Luma16")
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .font(jezyk.get_font())
                                        .center()
                                )
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::L16))
                                    .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_l16bit,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )

                    }else{
                        Row::new()
                    },)


                    .push(if stan_klikaczy.png_wybrany{
                    Row::new()
                        .push(
                            button(text("8-bit\nAlpha").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::B8a))
                                .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_8bita,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                                .height(Length::Fixed(50.)),
                        )
                        .push(
                            space().width(Length::Fixed(5.)),
                        )
                        .push(
                            container("")
                                .width(Length::Fixed(2.))
                                .height(Length::Fixed(50.))
                                .style(move |_theme| container::Style{
                                    text_color: None,
                                    background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                    border: Default::default()
                                    ,shadow: Default::default()
                                    ,snap: false,}),
                        )
                        .push(
                            space().width(Length::Fixed(5.)),
                        )
                        .push(
                            button(text("Luma8\nAlpha").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::L8a))
                                .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_l8bita,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                                .height(Length::Fixed(50.)),
                        )
                        .push(
                            space().width(Length::Fixed(5.)),
                        )
                        .push(
                            container("")
                                .width(Length::Fixed(2.))
                                .height(Length::Fixed(50.))
                                .style(move |_theme| container::Style{
                                    text_color: None,
                                    background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                    border: Default::default()
                                    ,shadow: Default::default()
                                    ,snap: false,}),
                        )
                        .push(
                            space().width(Length::Fixed(5.)),
                        )

                        .push(
                            button(text("16-bit\nAlpha").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::B16a))
                                .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_16bita,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                                .height(Length::Fixed(50.)),
                        )

                        .push(
                            space().width(Length::Fixed(5.)),
                        )
                        .push(
                            container("")
                                .width(Length::Fixed(2.))
                                .height(Length::Fixed(50.))
                                .style(move |_theme| container::Style{
                                    text_color: None,
                                    background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                    border: Default::default()
                                    ,shadow: Default::default()
                                    ,snap: false,}),
                        )
                        .push(
                            space().width(Length::Fixed(5.)),
                        )
                        .push(
                            button(text("luma16\nAlpha").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                .on_press(Message::ZdjeciaEdycjaZmianaBitDepthPng(Obraz::L16a))
                                .style(styl_przycisków!(false,stan_klikaczy.png_wybrane_l16bita,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5))
                                .height(Length::Fixed(50.)),
                        )

                    }else{
                        Row::new()
                    },)

                    .push(container("")
                          .width(Length::Fill)
                          .height(Length::Fixed(2.))
                          .style(|_theme| container::Style{
                              background: Some(Color::from_rgba(1.,1.,1.,0.2).into()),
                              ..container::Style::default()

                          }),)


                    .push(if stan_klikaczy.webp_wybrany{
                        Row::new()
                            .push(
                                button(
                                    text("webp")
                                        .font(jezyk.get_font())
                                        .width(Length::Fill)
                                        .center()
                                )
                                    .padding(10)
                                    .on_press(Message::ZdjeciaEdycjaZmianaWybranyWebp)
                                    .style(styl_przycisków!(false,stan_klikaczy.webp_wybrany,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5)),
                            )
                            .push(space().width(Length::FillPortion(1)),)
                            .push(
                                Column::new()
                                    .push(
                                        slider(0..=100,match dane.rozszerzenia[2] {
                                            Rozszerzenia::Webp { jakosc, .. } => jakosc,
                                            _ => 0
                                        },
                                               Message::ZdjeciaEdycjaZmianaJakosciWebp)
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
                                                    background: Color::from_rgb(KOLORSPANISHORANGE.0, KOLORSPANISHORANGE.1, KOLORSPANISHORANGE.2).into(),
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
                                                match dane.rozszerzenia[2] {
                                                    Rozszerzenia::Webp { jakosc, .. } => jakosc, _ => 0
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
                                    if stan_klikaczy.webp_wybrany{
                                        button(text("los").font(jezyk.get_font()).width(Length::Fill).center())
                                            .padding(10)
                                            .on_press(Message::ZdjeciaEdycjaZmianaLosslessWebp)
                                            .style(styl_przycisków!(false,stan_klikaczy.webp_lossless,KOLORSPANISHORANGE))
                                            .width(Length::FillPortion(5))
                                    }else{
                                        button(text("los").font(jezyk.get_font()).width(Length::Fill).center())
                                            .padding(10)
                                            .style(styl_przycisków!(false,stan_klikaczy.webp_lossless,KOLORSPANISHORANGE))
                                            .width(Length::FillPortion(5))
                                    },
                                    "Zapis progresywny on/off",
                                    tooltip::Position::Top
                                ),
                            )
                            .push(space().width(Length::FillPortion(1)),)

                    }else{
                        Row::new()
                            .push(button(text("webp").font(jezyk.get_font()).width(Length::Fill).center())
                                      .padding(10)
                                      .on_press(Message::ZdjeciaEdycjaZmianaWybranyWebp)
                                      .style(styl_przycisków!(false,stan_klikaczy.webp_wybrany,KOLORSPANISHORANGE))
                                      .width(Length::FillPortion(5)),)
                            .push(space().width(Length::FillPortion(13)),)

                    },)
                    .push(if stan_klikaczy.webp_wybrany{
                        Row::new()
                            .push(tooltip(
                                button(text(jezyk.t("foto_edit_color")).font(jezyk.get_font()).width(Length::Fill).center())
                                    .padding(10)
                                    .on_press(Message::ZdjeciaEdycjaZmianaKolorWebp(Obraz::B8))
                                    .style(styl_przycisków!(false,stan_klikaczy.webp_wybrany_rgb,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                                jezyk.t("foto_edit_tooltip_webp_color"),
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
                                button(text("alpha").font(jezyk.get_font()).width(Length::Fill).center())
                                    .padding(10)
                                    .on_press(Message::ZdjeciaEdycjaZmianaKolorWebp(Obraz::B8a))
                                    .style(styl_przycisków!(false,stan_klikaczy.webp_wybrany_alpha,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                                jezyk.t("foto_edit_tooltip_webp_alpha"),
                                tooltip::Position::Top
                            ),)

                    }else{
                        Row::new()
                    },)

                    .push(container("")
                          .width(Length::Fill)
                          .height(Length::Fixed(2.))
                          .style(|_theme| container::Style{
                              background: Some(Color::from_rgba(1.,1.,1.,0.2).into()),
                              ..container::Style::default()

                          }),
                    )

                    .push(Row::new()
                        .push(
                            button(text("tga").font(jezyk.get_font()).width(Length::Fill).center())
                                .padding(10)
                                .on_press(Message::ZdjeciaEdycjaZmianaWybranyTga)
                                .style(styl_przycisków!(false,stan_klikaczy.tga_wybrany,KOLORSPANISHORANGE))
                                .width(Length::FillPortion(5)),
                        )
                        .push(
                            space().width(Length::FillPortion(13))
                        ),)


                    .push(if stan_klikaczy.tga_wybrany{
                        Row::new()

                        // row![
                            .push(
                                button(text("szary8").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthTga(ObrazTGA::Szary8))
                                    .style(styl_przycisków!(false,stan_klikaczy.tga_wybrany_szary,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                button(text("HighColor16").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthTga(ObrazTGA::HighColor16))
                                    .style(styl_przycisków!(false,stan_klikaczy.tga_wybrany_16b,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                button(text("TrueColor24").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthTga(ObrazTGA::TrueColor24))
                                    .style(styl_przycisków!(false,stan_klikaczy.tga_wybrany_24b,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                container("")
                                    .width(Length::Fixed(2.))
                                    .height(Length::Fixed(50.))
                                    .style(move |_theme| container::Style{
                                        text_color: None,
                                        background: Some(Color::from_rgba(1.0,1.0,1.0,0.2).into()),
                                        border: Default::default()
                                        ,shadow: Default::default()
                                        ,snap: false,}),
                            )
                            .push(
                                space().width(Length::Fixed(5.)),
                            )
                            .push(
                                button(text("TrueColor32").width(Length::Fill).height(Length::Fill).font(jezyk.get_font()).center())
                                    .on_press(Message::ZdjeciaEdycjaZmianaBitDepthTga(ObrazTGA::TrueColorA32))
                                    .style(styl_przycisków!(false,stan_klikaczy.tga_wybrany_32b,KOLORSPANISHORANGE))
                                    .width(Length::FillPortion(5))
                                    .height(Length::Fixed(50.)),
                            )

                        }else{
                            Row::new()
                                .push(
                                    space()
                                )
                        },)

                        .push(container("")
                            .width(Length::Fill)
                            .height(Length::Fixed(2.))
                            .style(|_theme| container::Style{
                                background: Some(Color::from_rgba(1.,1.,1.,0.2).into()),
                                ..container::Style::default()

                        }),)

                        .push(Row::new()
                            .push(
                                text("WIP")
                            ),
                        )


            .spacing(15) //oesu ale to długie... a tyle krwi napsuło...
            .padding(15)
            .width(Length::FillPortion(2)))
            )            .spacing(15)//tu sie kończy kolumna.....................................................
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_rozdzielczosci = Column::new()
        .push(Grid::new()
            .push(
                button(
                    text("16px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R16))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_16,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("32px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R32))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_32,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("64px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R64))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_64,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("128px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R128))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_128,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("256px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R256))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_256,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("512px")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R512))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_512,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("1kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R1k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_1k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("2kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R2k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_2k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("4kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R4k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_4k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("6kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R6k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_6k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("8kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R8k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_8k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text("16kpx")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::R16k))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_16k,KOLORSPANISHORANGE))
                    .padding(10),
            )
            .push(
                button(
                    text(jezyk.t("foto_edit_resolution_oryginal"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(jezyk.get_font()
                )
                    .center())
                    .on_press(Message::DopasujRozdzielczosci(Rozdzielczości::Oryginalna))
                    .style(styl_przycisków!(false,stan_klikaczy.rozdzielczości_wybrane_org,KOLORSPANISHORANGE))
                    .padding(10),
            )
        .spacing(10)

        )            .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_dodatkowe = Column::new()
        //interpolacja
        .push(text(jezyk.t("foto_edit_interpolation")).font(jezyk.get_font()),)
        .push(pick_list(
            opcje_interpolacja,
            Some(jezyk.t(dane.inter.klucz()).to_string()),
            Message::PoziomInterpolacjiChanged, // Wysyła String

        )
        .width(Length::Fill)
        .padding(10)
        .style(styl_pick_list!(KOLORSPANISHORANGE, KOLORTŁA))
        .menu_style(styl_menu_pick!(KOLORSPANISHORANGE, KOLORTŁA)),)

        .push(text(match dane.noising{
            Some(x) => format!("{} {}%",jezyk.t("foto_edit_noising"),x),
            None => format!("{} {}",jezyk.t("foto_edit_noising"),jezyk.t("general_off"))
        }),)

        .push(
            slider(
                0..=100,
                dane.noising.unwrap_or(0),
                Message::ZdjeciaEdycjaZmianaZaszumiania
            )
                .style(move |_theme, _status| slider::Style {
                    rail: slider::Rail {
                        backgrounds: (
                                Color::from_rgba(
                                    KOLORSPANISHORANGE.0,
                                    KOLORSPANISHORANGE.1,
                                    KOLORSPANISHORANGE.2, 0.4)
                                    .into(), // Aktywne (lewo) - Czerwony
                                Color::from_rgb(0.2, 0.2, 0.2).into(), // Nieaktywne (prawo) - Szary
                            ),
                        width: 4.,
                        border: Border{
                        color: Color::from_rgba(
                            KOLORSPANISHORANGE.0,
                            KOLORSPANISHORANGE.1,
                            KOLORSPANISHORANGE.2,
                            0.2
                        ),width: 1.0 , radius: 5.0.into(),},
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 8.0 },
                        background: Color::from_rgb(
                            KOLORSPANISHORANGE.0,
                            KOLORSPANISHORANGE.1,
                            KOLORSPANISHORANGE.2
                        ).into(), // Czerwona kropka
                        border_width: 1.0,
                        border_color: Color::BLACK,
                    },
                }
            )
        )

        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));





    let lewa_kolumna = Column::new()
    // Przycisk Ścieżki
    .push(button(text(jezyk.t("foto_edit_menu_paths")).font(jezyk.get_font()).center())
        .on_press(Message::ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć::Ścieżki))
        .padding(12)
        .width(Length::Fill)
        .style(styl_przycisków!(false,matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::Ścieżki),KOLORSPANISHORANGE)),)


    // Przycisk Rozszerzenia
    .push(button(text(jezyk.t("foto_edit_menu_extensions")).font(jezyk.get_font()).center())
        .on_press(Message::ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć::Rozszerzenia))
        .padding(12)
        .width(Length::Fill)
        .style(styl_przycisków!(false,matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::Rozszerzenia),KOLORSPANISHORANGE)),)


    // Przycisk Rozdzielczości
    .push(button(text(jezyk.t("foto_edit_menu_resolution")).font(jezyk.get_font()).center())
        .on_press(Message::ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć::MenuRozdzielczości))
        .padding(12)
        .width(Length::Fill)
        .style(styl_przycisków!(false,matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::MenuRozdzielczości),KOLORSPANISHORANGE)),)

            // Przycisk inne
    .push(button(text(jezyk.t("foto_edit_menu_rest")).font(jezyk.get_font()).center())
        .on_press(Message::ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć::MenuReszta))
        .padding(12)
        .width(Length::Fill)
        .style(styl_przycisków!(false,matches!(wybrane_okno, WybraneOknoEdycjiZdjęć::MenuReszta),KOLORSPANISHORANGE)),)

    .push(space().height(Length::Fixed(50.)),)

    .push(Row::new()
        .push(text("Jpg").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.jpg_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("C").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.jpg_wybrany_rgb && stan_klikaczy.jpg_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("BW").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.jpg_wybrany_bw && stan_klikaczy.jpg_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

        .push(text("|").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., 0.3)).size(ROZMIARWYBRANYCHROZSZERZEN),)

        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(
                text(format!("{}%",match dane.rozszerzenia[0] {
                    Rozszerzenia::Jpg { jakosc, .. } => jakosc, _ => 0}
            )).font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.jpg_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),
        ),
    )




    .push(Row::new()

        .push(text("Png").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("8").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_8bit && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("8a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_8bita && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("l8").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_l8bit && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("l8a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_l8bita && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("16").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_16bit && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("16a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_16bita && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("l16").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_l16bit && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("l16a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.png_wybrane_l16bita && stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

        .push(text("|").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., 0.3)).size(ROZMIARWYBRANYCHROZSZERZEN),)

        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(
            text(format!("{}",match dane.rozszerzenia[1] {
                    Rozszerzenia::Png { kompresja, .. } => kompresja, _ => 0}
            )).font(iced::Font {
                family: iced::font::Family::Name("VT323"),
                ..Default::default()
            }).color(Color::from_rgba(1., 1., 1., if stan_klikaczy.png_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),
        ),
    )

    .push(Row::new()
              .push(text("Webp").font( iced::Font {family: iced::font::Family::Name("VT323"),
                  ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
              .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
              .push(text("C").font( iced::Font {family: iced::font::Family::Name("VT323"),
                  ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany_rgb && stan_klikaczy.webp_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
              .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
              .push(text("A").font( iced::Font {family: iced::font::Family::Name("VT323"),
                  ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany_alpha && stan_klikaczy.webp_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)

              .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

              .push(text("|").font( iced::Font {family: iced::font::Family::Name("VT323"),
                  ..Default::default()}).color(Color::from_rgba(1.,1.,1., 0.3)).size(ROZMIARWYBRANYCHROZSZERZEN),)

              .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

              .push(
                  text(format!("{}%",match dane.rozszerzenia[2] {
                      Rozszerzenia::Webp { jakosc, .. } => jakosc, _ => 2}
                  )).font( iced::Font {family: iced::font::Family::Name("VT323"),
                      ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany && !stan_klikaczy.webp_lossless{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),
              )

            .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

            .push(text("|").font( iced::Font {family: iced::font::Family::Name("VT323"),
                ..Default::default()}).color(Color::from_rgba(1.,1.,1., 0.3)).size(ROZMIARWYBRANYCHROZSZERZEN),)

            .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)

            .push(text( "Lossless" ).font( iced::Font {family: iced::font::Family::Name("VT323"),
                ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany && stan_klikaczy.webp_lossless{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),
            )
            .push(text( "/" ).font( iced::Font {family: iced::font::Family::Name("VT323"),
                ..Default::default()}).color(Color::from_rgba(1.,1.,1., 0.2)).size(ROZMIARWYBRANYCHROZSZERZEN),
            )
            .push(text( "Lossy" ).font( iced::Font {family: iced::font::Family::Name("VT323"),
                ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.webp_wybrany && !stan_klikaczy.webp_lossless{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),
            )

    )



        
    .push(Row::new()

        .push(text("Tga").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.tga_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("8b").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.tga_wybrany_szary && stan_klikaczy.tga_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("16a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.tga_wybrany_16b && stan_klikaczy.tga_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("24b").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.tga_wybrany_24b && stan_klikaczy.tga_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("32a").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.tga_wybrany_32b && stan_klikaczy.tga_wybrany{0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),),)
            

    
    .push(Row::new()
     
        .push(text("16").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_16 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("32").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_32 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("64").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_64 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("128").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_128 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("256").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_256 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("512").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_512 {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("1k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_1k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("2k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_2k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("4k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_4k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("6k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_6k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("8k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_8k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("16k").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_16k {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),)
        .push(text("org").font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).color(Color::from_rgba(1.,1.,1., if stan_klikaczy.rozdzielczości_wybrane_org {0.5}else{0.2})).size(ROZMIARWYBRANYCHROZSZERZEN),)
        .push(space().width(Length::Fixed(PRZERWAWYBRANYCHROZSZERZEN)),),)
            



    .push(
        if czy_sie_nada_na_wyslanie && !czy_jest_proces_zaczety && !main_process_check{
            button(
                text(
                    jezyk.t("btn_proces_avaliable")
                )
                    .font(jezyk.get_font())
                    .width(Length::Fill)
                    .center()
            )
                .on_press(Message::WysylkaDanychDoObrobkiZdjec)
                .width(Length::Fill)
                .style(styl_przycisków!(false,false,KOLORSPANISHORANGE))
        } else {
            button(
                text(
                    if czy_jest_proces_zaczety{jezyk.t("btn_proces_in_progress")}
                    else if main_process_check{jezyk.t("btn_proces_in_progress_other")}
                    else{jezyk.t("btn_proces_lack_of_data")})
                        .font(jezyk.get_font())
                        .width(Length::Fill)
                        .center()
            )
                .width(Length::Fill)
                .style(styl_przycisków!(false,czy_jest_proces_zaczety,KOLORSPANISHORANGE))
        },
    )
        
    .push(text(log.plik_początek),)
    
    .push(if log.plik_procent !=0{
        Row::new()
            .push(text("Postęp procesu:  ").font(jezyk.get_font()),)
            .push(progress_bar(0.0 ..=100.,log.plik_procent as f32),)

    } else{
        Row::new()
            .push(space())

    },)

        
    .push(text(log.msg_end).font(jezyk.get_font()),)
    .push(text(log.błąd).font(jezyk.get_font()),)
        

        .spacing(15)
        .width(Length::FillPortion(1));


    let prawa_kolumna =
        match wybrane_okno {
            WybraneOknoEdycjiZdjęć::Ścieżki => menu_ścieżki,
            WybraneOknoEdycjiZdjęć::Rozszerzenia => menu_rozszerzenia,
            WybraneOknoEdycjiZdjęć::MenuRozdzielczości => menu_rozdzielczosci,
            WybraneOknoEdycjiZdjęć::MenuReszta => menu_dodatkowe,
        }
; // Opcjonalnie, żeby UI nie rozjechało się na całą szerokość





    Row::new()
        .push(lewa_kolumna) .push(prawa_kolumna).into()

}