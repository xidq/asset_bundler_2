use iced::{
    font, widget::{button, column, container, row, scrollable, text, Space},
    Task,
};

// use crate::ui::podmenu_old::ui_dds::{view_dds, StronyDds};
use crate::ui::wiadomosci::message_enum::Message;
use enumy::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI};
use enumy::dane_do_przetwarzania::{DaneBinPak, DaneBinUnpak, DaneDdsPak, DaneDdsUnpak, DaneKonw, DaneMerge};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
pub(crate) use enumy::enums_structs_io::{LogPakowanie, LogPrzetwarzanieFot, LogRozpakowywanie};
use enumy::inne_ui::{BtnState, PrzyciskiGlowneMenu, UiPods, UstawieniaThemeWsio};
pub(crate) use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::widget::{image, stack, Column, Row};
use iced::{Border, Color, Element, Length};
use iced_core::{Shadow, Theme, Vector};

#[allow(dead_code)]
pub struct Program {
    pub(crate) dane_merge: DaneMerge,
    pub(crate) dane_bin_pak: DaneBinPak,
    pub(crate) dane_bin_unpak: DaneBinUnpak,
    pub(crate) ui_ustawienia: UstawieniaMenu,
    // UI state
    pub(crate) log_prawe_okno: Vec<String>,
    pub(crate) status_pakowanie_log: LogPakowanie,
    pub(crate) status_rozpakowywania_log: LogRozpakowywanie,
    pub(crate) status_zmiany_fot_log: LogPrzetwarzanieFot,
    pub(crate) status_dds_pakowanie: LogPakowaniaDds,
    pub(crate) status_dds_rozpakowywanie: LogRozpakowywanieDds,

    ui_main_wariant_dev: bool,
    // pub(crate) ui_dds_podmenu: StronyDds,
    zdjecia_edycja_co_jest_wybrane: (bool, bool),
    pub(crate) zdjecia_edycja_co_jest_na_out: bool,
    pub(crate) dane_konw: DaneKonw,
    do_nothing: bool,
    pub(crate) startowy_jezyk: String,
    pub(crate) dane_dds_pak: DaneDdsPak,
    pub(crate) dane_dds_rozpak: DaneDdsUnpak,
    uchwyt_szumu: image::Handle,

    pub temat:UstawieniaThemeWsio,

}

use crate::ui::podstrony::binarka::main::binarka_view;
use crate::ui::podstrony::dds::main::dds_view;
use crate::ui::podstrony::konwersja::main::konwersja_view;
use crate::ui::podstrony::merging::main::merge_view;
use crate::ui::podstrony::settings::main::ustawienia_view;
use crate::widget::button::przycisk_glowne_menu;
use crate::widget::colors_n_stuff::KOLOR_CZCIONKI_SREDNI;
use crate::widget::styles::styl_przycisków;


pub fn generuj_ziarno() -> image::Handle {
    let width = 512;
    let height = 512;
    let mut pixels = Vec::with_capacity(width * height * 4);


    let mnożnik = (u16::MAX as f64 / u8::MAX as f64).round()  ;
    for _ in 0..(width * height) {

        let ziarno: u16 = rand::random();
        let r: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(1., u8::MAX as f64)  as u8 ;
        let g: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(0., u8::MAX as f64)  as u8 ;
        let b: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(0., u8::MAX as f64)  as u8 ;
        let a: u8 = (((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(1., u8::MAX as f64) / 10. ).round()  as u8 ;


        pixels.push(r);
        pixels.push(g);
        pixels.push(b);

        pixels.push(a);
    }

    image::Handle::from_rgba(width as u32, height as u32, pixels)
}




impl Program {
    pub fn new() -> (Self, Task<Message>) {
        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

        // Mapujemy go na Twój Enum
        let startowy_jezyk = WybórJęzyka::z_systemu(locale.clone());
        (
            Self {
                ui_main_wariant_dev: false,
                ui_ustawienia: UstawieniaMenu::UstawieniaJęzyka {
                    jezyk: startowy_jezyk,
                },
                dane_merge: DaneMerge::default(),
                dane_bin_pak: DaneBinPak::default(),
                dane_bin_unpak: DaneBinUnpak::default(),
                log_prawe_okno: Vec::new(),
                status_pakowanie_log:  Default::default(),
                status_rozpakowywania_log:  Default::default(),
                status_zmiany_fot_log: Default::default(),
                status_dds_pakowanie:  Default::default(),
                status_dds_rozpakowywanie: Default::default(),
                zdjecia_edycja_co_jest_wybrane: (false, false),
                zdjecia_edycja_co_jest_na_out: false,

                dane_konw:  DaneKonw::default(),
                do_nothing: false,
                startowy_jezyk: locale.clone(),

                dane_dds_pak: DaneDdsPak::default(),
                dane_dds_rozpak: DaneDdsUnpak::default(),
                uchwyt_szumu: generuj_ziarno(),
                temat: UstawieniaThemeWsio::default(),
            },
            Task::batch(Vec::from([
                Task::done(Message::InitLogStartowy),
                font::load(FONT_DEFAULT).map(|_| Message::Nic),
                font::load(FONT_KOREAN).map(|_| Message::Nic),
                font::load(FONT_JAPANESE).map(|_| Message::Nic),
                font::load(FONT_THAI).map(|_| Message::Nic),
            ])),
        )
    }



    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(Vec::from([

            iced::event::listen().map(Message::EventOccurred),
        ]))
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let aktualny_jezyk = match &self.ui_ustawienia {
            UstawieniaMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => &WybórJęzyka::EN,
        };
        let nakladka_szum = iced::widget::image(self.uchwyt_szumu.clone())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .opacity(1.)
            .content_fit(iced::ContentFit::Cover);

        // --- LEWA STRONA ---
        let przyciski_menu = container(
            Row::new()
                .push(przycisk_glowne_menu(PrzyciskiGlowneMenu::Binarka, aktualny_jezyk, &self.temat))
                .push(przycisk_glowne_menu(PrzyciskiGlowneMenu::Konwersja, aktualny_jezyk, &self.temat))
                .push(przycisk_glowne_menu(PrzyciskiGlowneMenu::Łączenie, aktualny_jezyk, &self.temat))
                .push(przycisk_glowne_menu(PrzyciskiGlowneMenu::Dds, aktualny_jezyk, &self.temat))
                .push(przycisk_glowne_menu(PrzyciskiGlowneMenu::Ustawienia, aktualny_jezyk, &self.temat))

                .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fixed(50.))
        .style(move |_theme| {
            container::Style {
                background: Some(Color::from_rgb(0.1, 0.11, 0.11).into()),
                shadow: Shadow {
                    color: Color::from_rgb(0.1, 0.11, 0.11),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 20.0,
                },
                border: Border {
                    radius: 8.0.into(), // Warto dodać radius, żeby cień nie "kantował"
                    ..Border::default()
                },
                ..Default::default()
            }
        });

        // WYWOŁANIE WYDZIELONYCH MODUŁÓW
        let content_lewy = match self.temat.temp.act_window {
            UiPods::BinPak | UiPods::BinUnpak =>
                binarka_view(
                    &self.dane_bin_pak,
                    &self.dane_bin_unpak,
                    &self.status_pakowanie_log,
                    &self.status_rozpakowywania_log,
                    aktualny_jezyk,
                    &self.temat
                ),
            UiPods::KonwPath | UiPods::KonwRes | UiPods::KonwEtc | UiPods::KonwExt => {
                konwersja_view(
                    &self.dane_konw,
                    &self.status_zmiany_fot_log,
                    aktualny_jezyk,
                    &self.temat
                )
            }
            UiPods::Merge | UiPods::MergeExt => {
                merge_view(
                    &self.dane_merge,
                    aktualny_jezyk,
                    &self.temat
                )
            }
            UiPods::DdsPak | UiPods::DdsUnpak | UiPods::DdsExt => dds_view(
                &self.dane_dds_pak,
                &self.dane_dds_rozpak,
                aktualny_jezyk,
                &self.temat,
                // &self.status_dds_pakowanie,
                // &self.status_dds_rozpakowywanie,
            ),
            UiPods::Ustawienia => ustawienia_view(&self.ui_ustawienia, &self.temat),
            // _ => column![text("Opcja jest, lecz UI jeszcze nie").size(50)].into(),
        };

        let lewa_kolumna =
            stack![
            container(
                Column::new()
                    .push(przyciski_menu)
                    .push(Space::new().height(30))
                    .push(content_lewy)
                    .padding(20),
            )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgb(0.07, 0.07, 0.06).into()),

                // Cień o tym samym kolorze
                shadow: Shadow {
                    color: Color::from_rgb(0.1, 0.15, 0.2),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 0.0,
                },

                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            }
        }), nakladka_szum];

        // --- PRAWA STRONA (Logi zostawiamy tutaj, bo są proste) ---
        let logi_column = column(
            self.log_prawe_okno
                .iter()
                .map(|l| {
                    text(l)
                        .size(19)
                        .color(Color::from_rgba(1., 1., 1., 0.7))
                        .font(iced::Font {
                            family: font::Family::Name("VT323"),
                            ..Default::default()
                        })
                        .into()
                })
                .collect::<Vec<Element<'_, Message>>>(),
        )
        .spacing(5);

        let prawa_kolumna = container(
            Column::new()
                .push(
                    Row::new()
                        .push(
                            text(aktualny_jezyk.t("console_menu_status"))
                                .size(19)
                                .color(Color::from_rgba(1., 1., 1., 0.8))
                                .font(iced::Font {
                                    family: font::Family::Name("VT323"),
                                    ..Default::default()
                                })
                                .width(Length::FillPortion(2))
                                .center(),
                        )
                        .push(
                            button(
                                text(aktualny_jezyk.t("console_menu_reset"))
                                    .size(19)
                                    .width(Length::FillPortion(2))
                                    .height(Length::Fill)
                                    .font(iced::Font {
                                        family: font::Family::Name("VT323"),
                                        ..Default::default()
                                    })
                                    .center(),
                            )
                            .height(Length::Fixed(25.))
                            .on_press(Message::UsuńLogi)
                            .style(
                                styl_przycisków(
                                    &BtnState::Disabled,
                                    &self.temat.obecny_theme.bground_lewy,
                                    &self.temat
                                )
                            ),
                        )
                        .height(Length::Fixed(30.)),
                )
                .push(
                    text("---------------------------------------")
                        .size(19)
                        .color(KOLOR_CZCIONKI_SREDNI)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .font(iced::Font {
                            family: font::Family::Name("VT323"),
                            ..Default::default()
                        })
                        .center()
                        .height(Length::Fixed(25.)),
                )
                .push(Space::new().height(5))
                .push(scrollable(logi_column))
                .padding(15),
        )
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgb(0.1, 0.11, 0.13).into()),

                shadow: Shadow {
                    color: Color::from_rgb(0.1, 0.11, 0.11),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 10.0,
                },

                border: Border {
                    ..Border::default()
                },
                ..container::Style::default()
            }
        });

        row![lewa_kolumna, prawa_kolumna].into()
    }
}
