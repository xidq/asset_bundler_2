use crate::ui::program::{LogPrzetwarzanieFot, WybórJęzyka};
use crate::ui::program_pomniejsze::kolory::KOLOR_CZCIONKI_SREDNI;
pub(crate) use crate::ui::program_pomniejsze::kolory::{KOLOR_SPANISH_ORANGE, KOLOR_TŁA};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::ui_podmenu_ff::{
    podmenu_ff_misc, podmenu_ff_wybor,
};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::ui_podmenu_qoi::{
    podmenu_qoi_misc, podmenu_qoi_wybor,
};
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::{
    ui_podmenu_jpg::{podmenu_jpg_misc, podmenu_jpg_wybor_top},
    ui_podmenu_lewe::{podmenu_lewe_rozdzielczosci, podmenu_lewe_wybor},
    ui_podmenu_png::{podmenu_png_misc, podmenu_png_wybor},
    ui_podmenu_tga::{podmenu_tga_misc, podmenu_tga_wybor},
    ui_podmenu_webp::{podmenu_webp_misc, podmenu_webp_wybor},
};
use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use crate::ui::program_pomniejsze::style_fn::checkbox::styl_checkbox;
use crate::ui::program_pomniejsze::style_fn::pick_lista::{styl_menu_pick, styl_pick_list};
use crate::ui::program_pomniejsze::style_fn::progress_bar::styl_progress_bar;
use crate::ui::program_pomniejsze::style_fn::slider::styl_sliderów;
use crate::ui::program_pomniejsze::style_fn::text_input::styl_text_input;
use crate::ui::program_pomniejsze::ui_standard::oddzielacz::ui_standard_oddzielacz;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::ikony::folder_icon;
use enumy::opcje::{OptInterpolacja, OptRozdzielczościObrazów};
use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, scrollable, slider, space, text, text_input,
    Column, Grid, Row,
};
use iced::{Border, Color, Element, Length};
use enumy::inne_ui::CheckerDoZbiorowePrzetwarzanieZdjęć;
pub(crate) use enumy::inne_ui::WybraneOknoEdycjiZdjęć;
use crate::ui::wiadomosci::message_ui::Message;

pub(crate) const ROZMIARWYBRANYCHROZSZERZEN: iced::Pixels = iced::Pixels(14.);
pub(crate) const PRZERWAWYBRANYCHROZSZERZEN: f32 = 3.;



pub fn view_foto_change(
    dane: DaneDoBathKonwersjaZdjec,
    wybrane_okno: &WybraneOknoEdycjiZdjęć,
    jezyk: WybórJęzyka,
    wejście_check: (bool, bool),
    czy_wyjscie_te_same: bool,
    stan_klikaczy: CheckerDoZbiorowePrzetwarzanieZdjęć,
    czy_jest_proces_zaczety: bool,
    log: LogPrzetwarzanieFot,
    main_process_check: bool,
) -> Element<'_, Message> {
    let czy_sie_nada_na_wyslanie = (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_32
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_64
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_128
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_256
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_512
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_1k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_2k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_4k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_6k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_8k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16k
        || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_org)
        && (dane.ścieżka_wejściowa.is_file() || dane.ścieżka_wejściowa.is_dir())
        && dane.ścieżka_wyjściowa.is_dir()
        && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany
            && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_rgb || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_bw)
            || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany
                && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bit
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bit
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bita
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bita
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bit
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bita
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bit
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bita)
            || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany
                && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_szary
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_16b
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_24b
                    || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_32b)
            || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany
                && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany_rgb || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany_alpha)
            || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany
            || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany
                && (stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_24b || stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_32b));

    let opcje_interpolacja: Vec<String> = OptInterpolacja::WSIOINTERPOLACJI
        .iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    let menu_ścieżki = Column::new()
        .push(
            Row::new()
                .push(match wejście_check.0 {
                    true => button("📄")
                        .padding(10)
                        .on_press(Message::WybierzPlikInFotoEdycjaPakowanie)
                        .style(styl_przycisków(
                            false,
                            wejście_check.1,
                            KOLOR_SPANISH_ORANGE,
                        )),
                    false => button("🖼️")
                        .padding(10)
                        .on_press(Message::WybierzPlikInFotoEdycjaPakowanie)
                        .style(styl_przycisków(
                            false,
                            wejście_check.1,
                            KOLOR_SPANISH_ORANGE,
                        )),
                })
                .push(space().width(Length::Fixed(15.)))
                .push(match wejście_check.1 {
                    true => button(folder_icon(false, 2, KOLOR_SPANISH_ORANGE))
                        .padding(10)
                        .on_press(Message::WybierzFolderInFotoEdycjaPakowanie)
                        .style(styl_przycisków(false, false, KOLOR_SPANISH_ORANGE)),
                    false => button(folder_icon(
                        true,
                        if dane.ścieżka_wejściowa.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10)
                    .on_press(Message::WybierzFolderInFotoEdycjaPakowanie)
                    .style(styl_przycisków(
                        false,
                        false,
                        KOLOR_SPANISH_ORANGE,
                    )),
                }),
        )
        .push(
            Row::new().push(match wejście_check.1 {
                true => text_input(
                    jezyk.t("input_folder_or_file"),
                    &dane.ścieżka_wejściowa.to_string_lossy(),
                )
                .font(jezyk.get_font())
                .padding(10)
                .style(styl_text_input(dane.ścieżka_wejściowa.exists(),KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
                false => text_input(
                    jezyk.t("input_folder_or_file"),
                    &dane.ścieżka_wejściowa.to_string_lossy(),
                )
                .padding(10)
                .font(jezyk.get_font())
                .on_input(Message::ZdjeciaZmienFolderInPathChanged)
                .style(styl_text_input(dane.ścieżka_wejściowa.exists(),KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
            }),
        )
        .push(
            button(
                text("Resetuj ścieżki")
                    .font(jezyk.get_font())
                    .width(Length::Fill)
                    .center(),
            )
            .padding(10)
            .on_press(Message::ResetujStanWejsciowychSciezekEdycjaFoto)
            .style(styl_przycisków(false, true, KOLOR_SPANISH_ORANGE)),
        )
        .push(
            checkbox(czy_wyjscie_te_same)
                .label("Ścieżka wejściowa będzie wyjściową")
                .on_toggle(Message::ZdjeciaZmienFolderOutPathTenSam)
                .style(styl_checkbox(KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
        )
        .push(
            Row::new()
                .push(match czy_wyjscie_te_same {
                    true => button(folder_icon(
                        false,
                        if dane.ścieżka_wyjściowa.to_string_lossy().is_empty() {
                            1
                        } else {
                            2
                        },
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10)
                    .style(styl_przycisków(
                        false,
                        false,
                        KOLOR_SPANISH_ORANGE,
                    )),
                    false => button(folder_icon(
                        true,
                        if dane.ścieżka_wyjściowa.to_string_lossy().is_empty() {
                            0
                        } else {
                            2
                        },
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10)
                    .on_press(Message::WybierzFolderOutFotoEdycjaPakowanie)
                    .style(styl_przycisków(
                        false,
                        czy_wyjscie_te_same,
                        KOLOR_SPANISH_ORANGE,
                    )),
                })
                .push(match czy_wyjscie_te_same {
                    true => text_input(
                        jezyk.t("output_folder"),
                        &dane.ścieżka_wyjściowa.to_string_lossy(),
                    )
                    .padding(10)
                    .font(jezyk.get_font())
                    .style(styl_text_input(dane.ścieżka_wyjściowa.exists(),KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
                    false => text_input(
                        jezyk.t("output_folder"),
                        &dane.ścieżka_wyjściowa.to_string_lossy(),
                    )
                    .padding(10)
                    .font(jezyk.get_font())
                    .on_input(Message::ZdjeciaZmienFolderOutPathChanged)
                    .style(styl_text_input(dane.ścieżka_wyjściowa.exists(),KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
                }),
        )
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_rozszerzenia_plików_zdjęciowych = Column::new()
        .push(
            Row::new()
                .push(
                    Column::new()
                        // Slider dla R (indeks 0)
                        .push(
                            Row::new()
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.0, |v| {
                                        Message::ZdjeciaEdycjaZmianaKolorPng(0, v)
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
                                        Message::ZdjeciaEdycjaZmianaKolorPng(1, v)
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
                                        Message::ZdjeciaEdycjaZmianaKolorPng(2, v)
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
                ),
        )
        .width(Length::FillPortion(5))
        .push(scrollable(
            Column::new()
                .push(ui_standard_oddzielacz())
                .push(podmenu_jpg_wybor_top(&dane, &stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_png_wybor(&dane, &stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_webp_wybor(&dane, &stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_tga_wybor(&stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_ff_wybor(&dane, &stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_qoi_wybor( &stan_klikaczy, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(Row::new().push(text("WIP")))
                .spacing(15) //oesu ale to długie... a tyle krwi napsuło...
                .padding(15)
                .width(Length::FillPortion(2)),
        ))
        .spacing(15) //tu sie kończy kolumna.....................................................
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_rozdzielczosci = Column::new()
        .push(scrollable(
            Grid::new()
                .push(
                    button(
                        text("16px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R16,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("32px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R32,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_32,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("64px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R64,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_64,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("128px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R128,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_128,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("256px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R256,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_256,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("512px")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R512,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_512,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("1kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R1k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_1k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("2kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R2k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_2k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("4kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R4k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_4k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("6kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R6k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_6k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("8kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R8k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_8k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text("16kpx")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::R16k,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16k,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .push(
                    button(
                        text(jezyk.t("foto_edit_resolution_oryginal"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .font(jezyk.get_font())
                            .center(),
                    )
                    .on_press(Message::DopasujRozdzielczosci(
                        OptRozdzielczościObrazów::Oryginalna,
                    ))
                    .style(styl_przycisków(
                        false,
                        stan_klikaczy.zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_org,
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10),
                )
                .spacing(10),
        ))
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_dodatkowe = Column::new()
        //OptInterpolacja
        .push(text(jezyk.t("foto_edit_interpolation")).font(jezyk.get_font()))
        .push(
            pick_list(
                opcje_interpolacja,
                Some(jezyk.t(dane.inter.klucz()).to_string()),
                Message::PoziomInterpolacjiChanged, // Wysyła String
            )
            .width(Length::Fill)
            .padding(10)
            .style(styl_pick_list(KOLOR_SPANISH_ORANGE, KOLOR_TŁA))
            .menu_style(styl_menu_pick(KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
        )
        .push(text(match dane.noising {
            Some(x) => format!("{} {}%", jezyk.t("foto_edit_noising"), x),
            None => format!(
                "{} {}",
                jezyk.t("foto_edit_noising"),
                jezyk.t("general_off")
            ),
        }))
        .push(
            slider(
                0..=100,
                dane.noising.unwrap_or(0),
                Message::ZdjeciaEdycjaZmianaZaszumiania,
            )
            .style(styl_sliderów(KOLOR_SPANISH_ORANGE)),
        )
        .spacing(15)
        .padding(15)
        .width(Length::FillPortion(2));

    let lewa_kolumna = Column::new()
        // Przycisk Ścieżki
        .push(podmenu_lewe_wybor(wybrane_okno, &jezyk))
        .push(space().height(Length::Fixed(50.)))
        .push(podmenu_jpg_misc(&dane, &stan_klikaczy))
        .push(podmenu_png_misc(&dane, &stan_klikaczy))
        .push(podmenu_webp_misc(&dane, &stan_klikaczy))
        .push(podmenu_tga_misc( &stan_klikaczy))
        .push(podmenu_ff_misc(&stan_klikaczy))
        .push(podmenu_qoi_misc(&stan_klikaczy))
        .push(podmenu_lewe_rozdzielczosci(&stan_klikaczy))
        .push(space().height(Length::Fixed(10.)))
        .push(
            if czy_sie_nada_na_wyslanie && !czy_jest_proces_zaczety && !main_process_check {
                button(
                    text(jezyk.t("process_btn_start"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .width(Length::Fill)
                        .center(),
                )
                .on_press(Message::WysylkaDanychDoObrobkiZdjec)
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(false, false, KOLOR_SPANISH_ORANGE))
            } else {
                button(
                    text(if czy_jest_proces_zaczety {
                        jezyk.t("btn_bussy_processing")
                    } else if main_process_check {
                        jezyk.t("btn_bussy_processing_other")
                    } else {
                        jezyk.t("btn_gib_data")
                    })
                    .font(jezyk.get_font())
                    .color(Color::from_rgba(1., 1., 1., 0.7))
                    .width(Length::Fill)
                    .center(),
                )
                .width(Length::Fill)
                .height(Length::Fixed(40.))
                .style(styl_przycisków(
                    false,
                    czy_jest_proces_zaczety,
                    KOLOR_SPANISH_ORANGE,
                ))
            },
        )
        .push(text(log.plik_początek))
        .push(if log.plik_procent != 0 {
            Row::new()
                .push(text("Postęp procesu:  ").font(jezyk.get_font()))
                .push(
                    progress_bar(0.0..=100., log.plik_procent as f32)
                        .girth(18.)
                        .style(styl_progress_bar(KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
                )
        } else {
            Row::new()
        })
        .push(text(log.msg_end).font(jezyk.get_font()))
        .push(text(log.błąd).font(jezyk.get_font()))
        // .spacing(15)
        .width(Length::FillPortion(1));

    let prawa_kolumna = match wybrane_okno {
        WybraneOknoEdycjiZdjęć::Ścieżki => menu_ścieżki,
        WybraneOknoEdycjiZdjęć::OptRozszerzeniaPlikówZdjęciowych => {
            menu_rozszerzenia_plików_zdjęciowych
        }
        WybraneOknoEdycjiZdjęć::MenuOptRozdzielczościObrazów => menu_rozdzielczosci,
        WybraneOknoEdycjiZdjęć::MenuReszta => menu_dodatkowe,
    };

    Row::new().push(lewa_kolumna).push(prawa_kolumna).into()
}
