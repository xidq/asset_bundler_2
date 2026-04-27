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
use enumy::opcje::{FolderCzyPlik, OptInterpolacja, OptRozdzielczościObrazów};
use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, scrollable, slider, space, text, text_input,
    Column, Grid, Row,
};
use iced::{Border, Color, Element, Length};
use enumy::inne_ui::{CheckActiveProcess, CheckerDoZbiorowePrzetwarzanieZdjęć};
pub(crate) use enumy::inne_ui::WybraneOknoEdycjiZdjęć;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::inne::btn_rozdzielczosci;
use crate::ui::program_pomniejsze::podmenu_zdjecia_edycja::ui_podmenu_avif::podmenu_avif_wybor;
use crate::ui::wiadomosci::message_ui::Message;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::ZbiorowePrzetwarzanieZdjęćMessage;

pub(crate) const ROZMIARWYBRANYCHROZSZERZEN: iced::Pixels = iced::Pixels(14.);
pub(crate) const PRZERWAWYBRANYCHROZSZERZEN: f32 = 3.;



pub fn view_foto_change<'a>(
    dane: &'a DaneDoBathKonwersjaZdjec,
    wybrane_okno: &WybraneOknoEdycjiZdjęć,
    jezyk: &WybórJęzyka,
    czy_wyjscie_te_same: bool,
    log: LogPrzetwarzanieFot,
    main_process_check: &CheckActiveProcess,
) -> Element<'a, Message> {
    let czy_sie_nada_na_wyslanie =
        *main_process_check == CheckActiveProcess::ProcessŻodyn &&
            (dane.ścieżka_wejściowa.is_dir() || dane.ścieżka_wejściowa.is_file()) &&
            dane.ścieżka_wyjściowa.is_dir() &&
            dane.opcje_rozdzielczości.len() > 0 &&
            dane.rozszerzenia_plików_zdjęciowych.len() > 0;

    let opcje_interpolacja: Vec<String> = OptInterpolacja::WSIOINTERPOLACJI
        .iter()
        .map(|p| jezyk.t(p.klucz()).to_string())
        .collect();

    let menu_ścieżki =
        Column::new()
        .push(
            Row::new()
                .push(
                    button(
                        if !dane.ścieżka_wejściowa.is_file() { "📄" } else { "🖼️" })
                        .padding(10)
                        .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WybierzPlikInFotoEdycjaPakowanie))
                        .style(styl_przycisków(
                            false,
                            dane.ścieżka_wejściowa.is_file(),
                            KOLOR_SPANISH_ORANGE,
                        ))

                )
                .push(space().width(Length::Fixed(15.)))
                .push(
                    button(
                        folder_icon(
                        true,
                        match (!dane.ścieżka_wejściowa.to_string_lossy().is_empty(),dane.ścieżka_wejściowa.is_file()){
                            (false, false) => {2},
                            (false,true) => {1},
                            _ => {0}
                        },
                        KOLOR_SPANISH_ORANGE,
                    ))
                    .padding(10)
                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WybierzFolderInFotoEdycjaPakowanie))
                    .style(styl_przycisków(
                        false,
                        dane.ścieżka_wejściowa.is_dir(),
                        KOLOR_SPANISH_ORANGE,
                    )),
                ),
        )
        .push(
            Row::new()
                .push(
                    text_input(
                    jezyk.t("input_folder_or_file"),
                    &dane.ścieżka_wejściowa.to_string_lossy(),
                )
                .padding(10)
                .font(jezyk.get_font())
                .on_input(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderInPathChanged(xx)))
                .style(styl_text_input(dane.ścieżka_wejściowa.exists(),KOLOR_SPANISH_ORANGE, KOLOR_TŁA)),
            ),
        )
        .push(
            button(
                text("Resetuj ścieżki")
                    .font(jezyk.get_font())
                    .width(Length::Fill)
                    .center(),
            )
            .padding(10)
            .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ResetujStanWejsciowychSciezekEdycjaFoto))
            .style(styl_przycisków(false, true, KOLOR_SPANISH_ORANGE)),
        )
        .push(
            checkbox(czy_wyjscie_te_same)
                .label("Ścieżka wejściowa będzie wyjściową")
                .on_toggle(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderOutPathTenSam(xx)))
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
                    .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WybierzFolderOutFotoEdycjaPakowanie))
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
                    .on_input(|xx|Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaZmienFolderOutPathChanged(xx)))
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
                        .push(
                            Row::new()
                                .push(
                                    slider(0..=65535, dane.alfa_rgb.0, |v| {
                                        Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKolorAlpha(0, v))
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
                                        Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKolorAlpha(1, v))
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
                                        Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::ZdjeciaEdycjaZmianaKolorAlpha(2, v))
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
                .push(podmenu_avif_wybor(&dane, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_jpg_wybor_top(&dane, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_png_wybor(&dane,  &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_webp_wybor(&dane,  &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_tga_wybor(&dane, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_ff_wybor(&dane, &jezyk))
                .push(ui_standard_oddzielacz())
                .push(podmenu_qoi_wybor( &dane, &jezyk))
                .push(ui_standard_oddzielacz())
                // .push(Row::new().push(text("WIP")))
                .spacing(15) //oesu ale to długie... a tyle krwi napsuło...
                .padding(15)
                .width(Length::FillPortion(2)),
        ))
        .spacing(15) //tu sie kończy kolumna.....................................................
        .padding(15)
        .width(Length::FillPortion(2));

    let menu_rozdzielczosci = Column::new()
        .push(scrollable(
            Column::new()
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R16, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R32, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R64, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R128, jezyk.get_font())).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R256, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R512, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R1k, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R2k, jezyk.get_font())).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R4k, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R6k, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R8k, jezyk.get_font()))
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::R16k, jezyk.get_font())).spacing(10)
                )
                .push(
                    Row::new()
                        .push(btn_rozdzielczosci(dane, OptRozdzielczościObrazów::Oryginalna, jezyk.get_font())).spacing(10)
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
        .push(podmenu_jpg_misc(&dane))
        .push(podmenu_png_misc(&dane))
        .push(podmenu_webp_misc(&dane))
        .push(podmenu_tga_misc( &dane))
        .push(podmenu_ff_misc(&dane))
        .push(podmenu_qoi_misc(&dane))
        .push(podmenu_lewe_rozdzielczosci(&dane))
        .push(space().height(Length::Fixed(10.)))
        .push(
            if czy_sie_nada_na_wyslanie  {
                button(
                    text(jezyk.t("process_btn_start"))
                        .font(jezyk.get_font())
                        .color(Color::from_rgba(1., 1., 1., 0.8))
                        .width(Length::Fill)
                        .center(),
                )
                .on_press(Message::ZbiorowePrzetwarzanieZdjęć(ZbiorowePrzetwarzanieZdjęćMessage::WysylkaDanychDoObrobkiZdjec))
                .height(Length::Fixed(40.))
                .width(Length::Fill)
                .style(styl_przycisków(false, *main_process_check == CheckActiveProcess::ProcessŻodyn, KOLOR_SPANISH_ORANGE))
            } else {
                button(
                    text(if *main_process_check == CheckActiveProcess::ProcessKonwersjaZdjęć {
                        jezyk.t("btn_bussy_processing")
                    } else if *main_process_check != CheckActiveProcess::ProcessŻodyn {
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
                    *main_process_check == CheckActiveProcess::ProcessKonwersjaZdjęć,
                    false,
                    KOLOR_SPANISH_ORANGE,
                ))
            },
        )
        .push(text(log.plik_początek))
        .push(text(log.msg_walidacja))
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
