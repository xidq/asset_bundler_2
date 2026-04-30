use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use iced::{
    font, widget::{button, column, container, row, scrollable, text, Space}, Event,
    Task,
};

use crate::ui::program_pomniejsze::kolory::{KOLOR_BRILIANT_CRIMSON, KOLOR_COTTON_CANDY, KOLOR_CRIMSON_GLORY, KOLOR_CZCIONKI_SREDNI, KOLOR_LIGHT_PINK, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA};
use chrono::{Local, Timelike};
use std::path::PathBuf;

use crate::ui::program_pomniejsze::ui_dds::{view_dds, StronyDds};
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::WybraneOknoEdycjiZdjęć;
use crate::ui::wiadomosci::message_ui::Message;
use enumy::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI};
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoDekompresjaPlików, DaneDoKompresjaPlików, DaneDoPakowaniaDds, DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds, FILTERFOTO};
pub(crate) use enumy::enums_structs_io::{LogPakowanie, LogPrzetwarzanieFot, LogRozpakowywanie};
use enumy::inne_ui::{ActProces, ObecnyColorCzcionkiPrzezroczystosci, ObecnyColorTheme, ObecnyColorThemePrzezroczystosci, UiPodstrony, Temp, Ustawienia, UstawieniaThemeWsio, WybranyFormatZdjecia};
use enumy::opcje::{JpgQuant, JpgSamplingFac, OptFormatDds, OptFormatyKoloruObrazOgólny, OptInterpolacja, OptKompresjaDds, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychPojedyncze, OptRozszerzeniaPlikówZdjęciowychZnacznik};
pub(crate) use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::widget::{image, stack, Column, Row};
use iced::{Border, Color, Element, Length};
use iced_core::{Shadow, Theme, Vector};


#[allow(dead_code)]
pub struct Program {
    pub(crate) dane_temp_do_łączenia_zdjęć: DaneDoŁączeniaZdjęć,
    pub(crate) dane_temp_do_kompresji_plików: DaneDoKompresjaPlików,
    pub(crate) dane_temp_do_dekompresji_plików: DaneDoDekompresjaPlików,
    ui_ustawienia: UstawieniaMenu,
    // UI state
    pub(crate) log_prawe_okno: Vec<String>,
    pub(crate) status_pakowanie_log: LogPakowanie,
    pub(crate) status_rozpakowywania_log: LogRozpakowywanie,
    pub(crate) status_zmiany_fot_log: LogPrzetwarzanieFot,
    pub(crate) status_dds_pakowanie: LogPakowaniaDds,
    pub(crate) status_dds_rozpakowywanie: LogRozpakowywanieDds,
    
    ui_main_wariant_dev: bool,
    pub(crate) ui_dds_podmenu: StronyDds,
    zdjecia_edycja_co_jest_wybrane: (bool, bool),
    pub(crate) zdjecia_edycja_co_jest_na_out: bool,
    pub(crate) dane_temp_do_zbiorowe_przetwarzanie_zdjęć: DaneDoBathKonwersjaZdjec,
    do_nothing: bool,
    startowy_jezyk: String,
    pub(crate) dane_temp_do_pakowania_dds: DaneDoPakowaniaDds,
    pub(crate) dane_temp_do_rozpakowywania_dds: DaneDoRozpakowaniaDds,
    pub(crate) dane_temp_do_rozpakowania_dds_formaty_zdjec: WybranyFormatZdjecia,
    uchwyt_szumu: image::Handle,

    pub temat:UstawieniaThemeWsio,

}
use crate::ui::program_pomniejsze::style_fn::hint_master::hint_btn;

// pub fn generuj_szum_pro() -> image::Handle {
//     let width = 1024;
//     let height = 1024;
//
//     // 1. Pobieramy ziarno całkowicie bez użycia 'thread_rng' i słowa 'gen'
//     // rand::random() to najprostszy sposób na u32 w nowym randzie
//     let ziarno_dla_szumu: u32 = rand::random();
//
//     // 2. Inicjalizacja Fbm (Fractal Brownian Motion)
//     let fbm = Fbm::<Perlin>::new(ziarno_dla_szumu);
//
//     let mut pixels = Vec::with_capacity(width * height * 4);
//     let skala_zoom = 0.08;
//
//     for y in 0..height {
//         for x in 0..width {
//             // noise::NoiseFn wykorzystuje metodę .get() - to jest bezpieczne
//             let wartosc_szumu = fbm.get([x as f64 * skala_zoom, y as f64 * skala_zoom]);
//
//             // Mapujemy [-1.0, 1.0] na zakres [0, 255]
//             let n = (((wartosc_szumu + 1.0) / 2.0) * 255.0).clamp(0.0, 255.0) as u8;
//
//             pixels.push(n); // R
//             pixels.push(n); // G
//             pixels.push(n); // B
//
//             // Bardzo niski alpha (przezroczystość), żeby tylko "rozbić" banding
//             pixels.push(5);
//         }
//     }
//
//     // Handle::from_rgba w nowym Iced jest standardem
//     image::Handle::from_rgba(width as u32, height as u32, pixels)
// }
pub fn generuj_ziarno() -> image::Handle {
    let width = 512;
    let height = 512;
    let mut pixels = Vec::with_capacity(width * height * 4);


    let mnożnik = (u16::MAX as f64 / u8::MAX as f64).round()  ;
    for _ in 0..(width * height) {
        // let baza: u16 = rand::random();
        // let r_rand: u16 = rand::random();
        // let g_rand: u16 = rand::random();
        // let b_rand: u16 = rand::random();
        //
        // // Twoja logika: (Random_kanału + Baza/2) / 2
        // // Przesunięcie o 8 bitów w prawo (>> 8) to najszybsze dzielenie przez 256
        // let r = ((r_rand / 2 + baza / 4) >> 7) as u8;
        // let g = ((g_rand / 2 + baza / 4) >> 7) as u8;
        // let b = ((b_rand / 2 + baza / 4) >> 7) as u8;

        let ziarno: u16 = rand::random();
        let r: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(0., u8::MAX as f64)  as u8 ;
        let g: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(0., u8::MAX as f64)  as u8 ;
        let b: u8 = ((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(0., u8::MAX as f64)  as u8 ;
        let a: u8 = (((rand::random::<u16>() as f64 + (ziarno as f64 / 2.)) / (2. * mnożnik)).round().clamp(1., u8::MAX as f64) / 15. ).round()  as u8 ;


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
                dane_temp_do_łączenia_zdjęć: DaneDoŁączeniaZdjęć {
                    sciezka_r: None,
                    sciezka_g: None,
                    sciezka_b: None,
                    sciezka_a: None,
                    sciezka_out: PathBuf::new(),
                    out_format: OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg {
                        jakosc: 90,
                        bit_depth: OptFormatyKoloruObrazOgólny::B8,
                        sampling: JpgSamplingFac::R444,
                        progresywny: false,
                        quant: JpgQuant::Default,
                        scans:4,
                    },
                    tag: OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,
                    nazwa: String::new(),
                },
                dane_temp_do_kompresji_plików: DaneDoKompresjaPlików {
                    ścieżka_in: PathBuf::new(),
                    ścieżka_out: PathBuf::new(),
                    kompresja: OptKompresjaPlikówPoziomKompresjiZstd::Standard,
                    nazwa: String::new(),
                    foldery: true,
                    filtracja: OptKompresjaPlikówFiltracjaPlików::Wszystkie,
                },
                dane_temp_do_dekompresji_plików: DaneDoDekompresjaPlików {
                    ścieżka_pliku: PathBuf::new(),
                    ścieżka_docelowa: PathBuf::new(),
                },
                log_prawe_okno: Vec::new(),
                status_pakowanie_log:  Default::default(),
                status_rozpakowywania_log:  Default::default(),
                status_zmiany_fot_log: Default::default(),
                status_dds_pakowanie:  Default::default(),
                status_dds_rozpakowywanie: Default::default(),
                ui_dds_podmenu: StronyDds::ZplikuDoDds,
                zdjecia_edycja_co_jest_wybrane: (false, false),
                zdjecia_edycja_co_jest_na_out: false,

                dane_temp_do_zbiorowe_przetwarzanie_zdjęć: DaneDoBathKonwersjaZdjec {
                    ścieżka_wejściowa: PathBuf::new(),
                    ścieżka_wyjściowa: PathBuf::new(),
                    opcje_rozdzielczości: Vec::from([OptRozdzielczościObrazów::R2k]),
                    // dane_exif: DaneExif,
                    noising: None,
                    rozszerzenia_plików_zdjęciowych: Vec::from([
                        OptRozszerzeniaPlikówZdjęciowych::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: Vec::from([OptFormatyKoloruObrazOgólny::B8]),
                            sampling: JpgSamplingFac::R420,
                            quant: JpgQuant::Default,
                            scans: 4,
                        }
                    ]),
                    tag: Vec::from([OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg]),
                    inter: OptInterpolacja::Lanczos3,
                    alfa_rgb: (0, 0, 0),
                },
                do_nothing: false,
                startowy_jezyk: locale.clone(),
                
                dane_temp_do_pakowania_dds: DaneDoPakowaniaDds {
                    ścieżka_wejściowa: None,
                    ścieżka_wyjściowa: PathBuf::new(),
                    nazwa: String::new(),
                    format: OptFormatDds::DxgiFormatBc7Unorm,
                    kompresja: OptKompresjaDds::Normal,
                },
                dane_temp_do_rozpakowywania_dds: DaneDoRozpakowaniaDds {
                    ścieżka_wejściowa: PathBuf::new(),
                    ścieżka_wyjściowa: PathBuf::new(),
                    nazwa: String::new(),
                    rozszerzenie: OptRozszerzeniaPlikówZdjęciowych::Jpg {
                        jakosc: 90,
                        progresywny: false,
                        bit_depth: vec![OptFormatyKoloruObrazOgólny::B8],
                        sampling: JpgSamplingFac::R420,
                        quant: JpgQuant::Default,
                        scans: 4,
                    },
                    tag: OptRozszerzeniaPlikówZdjęciowychZnacznik::Jpg,
                },
                dane_temp_do_rozpakowania_dds_formaty_zdjec: WybranyFormatZdjecia::Jpg,
                uchwyt_szumu: generuj_ziarno(),
                temat: UstawieniaThemeWsio {
                    kolory: ObecnyColorTheme {
                        binarka: KOLOR_BRILIANT_CRIMSON,
                        konwersja: KOLOR_SPANISH_ORANGE,
                        laczenie: KOLOR_PEACH_PUFF,
                        dds: KOLOR_COTTON_CANDY,
                        ustawienia: KOLOR_CRIMSON_GLORY,
                        hint: KOLOR_LIGHT_PINK,
                    },
                    obecny_theme: ObecnyColorThemePrzezroczystosci {
                        max: 0.9,
                        hi: 0.7,
                        mid: 0.5,
                        low: 0.2,
                        min: 0.0,
                        kolor: Color::WHITE,
                        bground: KOLOR_TŁA,
                        bground_lewy: Color::from_rgb(0.1, 0.11, 0.13),
                    },
                    tekst: ObecnyColorCzcionkiPrzezroczystosci {
                        max: 0.9,
                        hi: 0.7,
                        mid: 0.5,
                        low: 0.3,
                        min: 0.0,
                        kolor: Color::WHITE,
                    },
                    ustawienia: Ustawienia { 
                        halp_menu: false 
                    },
                    temp: Temp { 
                        aktywny_proces: ActProces::Żodyn, 
                        aktywne_okno: UiPodstrony::BinPakowanie
                    },
                },
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

    // 2. POPRAWKA: Sygnatura UPDATE (Iced 0.13+ oczekuje 2 argumentów: &mut self i Message)
    // Usunąłem trzeci argument, który generował błąd E0593
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let aktualny_jezyk = match self.ui_ustawienia {
            UstawieniaMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => WybórJęzyka::EN,
        };

        match message {
            Message::InitLogStartowy => {
                let powitanie = format!(
                    "{}!!!!!\n {}: {}\n  {}: {}\n   {}, \n    {}: {}\n---------------------------------------",
                    aktualny_jezyk.t(if Local::now().hour() < 6 {
                        "log_status_welcome_msg_welcome_morning"
                    } else if Local::now().hour() > 19 {
                        "log_status_welcome_msg_welcome_evening"
                    } else {
                        "log_status_welcome_msg_welcome_day"
                    }),
                    aktualny_jezyk.t("log_status_welcome_msg_today"),
                    Local::now().format("%d.%m.%Y"),
                    aktualny_jezyk.t("log_status_welcome_msg_time"),
                    Local::now().format("%H:%M:%S"),
                    aktualny_jezyk.t("log_status_welcome_msg_sys_rdy"),
                    aktualny_jezyk.t("log_status_welcome_msg_lang_detected"),
                    self.startowy_jezyk
                );
                self.log_prawe_okno.push(powitanie);

            }

            Message::UsuńLogi => self.log_prawe_okno = Vec::new(),
            Message::DevZmienJezyk(nowy) => {
                self.ui_ustawienia = UstawieniaMenu::UstawieniaJęzyka { jezyk: nowy };
            }

            Message::ZmienWariant(w) => {
                self.temat.temp.aktywne_okno = w;
            }


            Message::DoNothingxD(xx) => self.do_nothing = xx,
            Message::DoNothingU8xD(_) => {}
            Message::DoNothingStringxD(_x) => self.do_nothing = false,
            
            
            Message::PakowanieBinarki(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_pakowanie_binarki(msg)
                    .map(Message::PakowanieBinarki);
            },
            Message::RozpakowanieBinarki(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_rozpakowanie_binarki(msg)
                    .map(Message::RozpakowanieBinarki);
            },
            Message::ZbiorowePrzetwarzanieZdjęć(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_zbiorowe_przetwarzanie_zdjec(msg)
                    .map(Message::ZbiorowePrzetwarzanieZdjęć);
            },

            Message::ŁączenieZdjęć(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_łączenie_zdjęć(msg)
                                .map(Message::ŁączenieZdjęć);
            },
            Message::Dds(msg) => {return self.update_message_dds(msg)
                .map( Message::Dds) 
            },

            
            
            Message::EventOccurred(Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            })) => {
                // match event {
                //     // ... Twoja obsługa FileDropped ...
                //
                //     Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                //         // 1. Obsługa Ctrl + E (Przełączanie Dev Mode)
                //         if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                //             self.ui_main_wariant_dev = !self.ui_main_wariant_dev;
                //             // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                //             self.log_prawe_okno.push(format!("Dev Mode: {}", self.ui_main_wariant_dev));
                //         }
                //     }
                //     _ => {}
                // }

                // 1. Obsługa Ctrl + E (Przełączanie Dev Mode)
                if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                    self.ui_main_wariant_dev = !self.ui_main_wariant_dev;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno
                        .push(format!("Dev Mode: {}", self.ui_main_wariant_dev));
                }
                if modifiers.control() && key == iced::keyboard::Key::Character("h".into()) {
                    self.temat.ustawienia.halp_menu = !self.temat.ustawienia.halp_menu;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno.push(format!("Halp Mode: {}", self.temat.ustawienia.halp_menu));
                }
            }

            _ => {}
        }
        Task::none()
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(Vec::from([
            // 1. Nasłuchiwanie na zmianę rozmiaru okna

            // 2. Nasłuchiwanie na ogólne zdarzenia (klawiatura, mysz itp.)
            iced::event::listen().map(Message::EventOccurred),
        ]))
    }

    // 3. POPRAWKA: Sygnatura VIEW (Iced 0.13+ oczekuje TYLKO &self)
    // Błąd E0593 brał się stąd, że w poprzednim przykładzie sygnatura mogła sugerować trait-object lub stare API
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
                .push(hint_btn(
                    button(
                        text(aktualny_jezyk.t("ui_main_btn_binary"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(UiPodstrony::BinPakowanie))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        matches!(self.temat.temp.aktywny_proces, ActProces::PakowaniePliku | ActProces::RozpakowaniePliku),
                        matches!(self.temat.temp.aktywne_okno, UiPodstrony::BinPakowanie | UiPodstrony::BinRozpakowanie),
                        &self.temat.kolory.binarka, &self.temat
                    )),
                    aktualny_jezyk.t("hint_ui_main_btn_binary"),
                    &self.temat
                ))
                .push(hint_btn(
                    button(
                        text(aktualny_jezyk.t("ui_main_btn_conversion"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(UiPodstrony::KonwersjaFoto))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.temat.temp.aktywny_proces == ActProces::KonwersjaZdjęć,
                        matches!(self.temat.temp.aktywne_okno, UiPodstrony::KonwersjaFoto | UiPodstrony::KonwersjaFotoRozdzielczości | UiPodstrony::KonwersjaFotoRozszerzenia | UiPodstrony::KonwersjaFotoŚcieżki | UiPodstrony::KonwersjaFotoMenuReszta),
                        &self.temat.kolory.konwersja, &self.temat
                    )),
                    aktualny_jezyk.t("hint_ui_main_btn_conversion"),
                    &self.temat
                ))
                .push(
                    hint_btn(
                    button(
                        text(aktualny_jezyk.t("ui_main_btn_merge"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(UiPodstrony::DaneDoŁączeniaZdjęćo))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.temat.temp.aktywny_proces == ActProces::ŁączenieZdjęć,
                        self.temat.temp.aktywne_okno == UiPodstrony::DaneDoŁączeniaZdjęćo,
                        &self.temat.kolory.laczenie,
                        &self.temat
                    )),
                    aktualny_jezyk.t("hint_ui_main_btn_merge"),&self.temat
                ))
                .push(hint_btn(
                    button(
                        text(aktualny_jezyk.t("ui_main_btn_dds"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(UiPodstrony::ObslugaDds))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.temat.temp.aktywny_proces == ActProces::DdsPakowanie || self.temat.temp.aktywny_proces == ActProces::DdsRozpakowanie,
                        self.temat.temp.aktywne_okno == UiPodstrony::ObslugaDds,
                        &self.temat.kolory.dds, &self.temat
                    )),
                    aktualny_jezyk.t("hint_ui_main_btn_dds"),&self.temat
                ))
                .push(
                    hint_btn(
                    button(
                        text(aktualny_jezyk.t("ui_main_btn_settings"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(UiPodstrony::Dev))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        false,
                        self.temat.temp.aktywne_okno == UiPodstrony::Dev,
                        &self.temat.kolory.ustawienia, &self.temat
                    )),
                    aktualny_jezyk.t("hint_ui_main_btn_settings"),&self.temat
                ))
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
        let content_lewy = match self.temat.temp.aktywne_okno {
            UiPodstrony::BinPakowanie | UiPodstrony::BinRozpakowanie =>
                crate::ui::program_pomniejsze::ui_binarka::view_binarka(
                    &self.dane_temp_do_kompresji_plików,
                    &self.dane_temp_do_dekompresji_plików,
                    aktualny_jezyk,
                    &self.status_pakowanie_log,
                    &self.status_rozpakowywania_log,
                    &self.temat
                ),
            UiPodstrony::KonwersjaFoto => {
                crate::ui::program_pomniejsze::ui_zdjecia_edycja::view_foto_change(
                    &self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć,
                    aktualny_jezyk,
                    &self.zdjecia_edycja_co_jest_na_out,
                    &self.status_zmiany_fot_log,
                    &self.temat
                )
            }
            UiPodstrony::DaneDoŁączeniaZdjęćo => {
                crate::ui::program_pomniejsze::ui_laczenie_zdjec::view_laczenie(
                    self.dane_temp_do_łączenia_zdjęć.clone(),
                    *aktualny_jezyk,
                    &self.temat
                )
            }
            UiPodstrony::ObslugaDds => view_dds(
                &self.dane_temp_do_pakowania_dds,
                &self.dane_temp_do_rozpakowywania_dds,
                &self.ui_dds_podmenu,
                aktualny_jezyk,
                &self.status_dds_pakowanie,
                &self.status_dds_rozpakowywanie,
                &self.temat,
            ),
            /*OptUIWariantPodstrony::Dev */ _ => crate::ui::program_pomniejsze::dev::ui_ustawienia(&self.ui_ustawienia,&self.temat),
            //_ => column![text("Opcja jest, lecz UI jeszcze nie").size(50)].into(),
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
                // Tło: r: 0.11, g: 0.11, b: 0.1, alpha: 1.0 (zakładam pełne krycie)
                background: Some(Color::from_rgb(0.07, 0.07, 0.06).into()),

                // Cień o tym samym kolorze
                shadow: Shadow {
                    color: Color::from_rgb(0.1, 0.15, 0.2),
                    offset: Vector::new(0.0, 0.0), // Przesunięcie cienia w dół
                    blur_radius: 0.0,              // Rozmycie cienia
                },

                border: Border {
                    // radius: 8.0.into(), // Zaokrąglone rogi, żeby cień ładniej wyglądał
                    ..Border::default()
                },
                ..container::Style::default()
            }
        }), nakladka_szum];
        // self.log_prawe_okno.push(format!("{}!!!!!\n {}: {}\n  {}: {}\n   {}, \n    {}: {}\n---------------------------------------",aktualny_jezyk.t("log_status_welcome_msg_welcome"),aktualny_jezyk.t("log_status_welcome_msg_today"),Local::now().format("%d.%m.%Y"),aktualny_jezyk.t("log_status_welcome_msg_today"), Local::now().format("%H:%M:%S"),aktualny_jezyk.t("log_status_welcome_msg_sys_rdy"),aktualny_jezyk.t("log_status_welcome_msg_lang_detected"),self.startowy_jezyk));

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
                            .style(styl_przycisków(
                                false,
                                false,
                                &self.temat.obecny_theme.bground_lewy,
                                &self.temat
                            )),
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
                // Tło: r: 0.1, g: 0.15, b: 0.2, alpha: 1.0 (zakładam pełne krycie)
                background: Some(Color::from_rgb(0.1, 0.11, 0.13).into()),

                // Cień o tym samym kolorze
                shadow: Shadow {
                    color: Color::from_rgb(0.1, 0.11, 0.11),
                    offset: Vector::new(0.0, 0.0), // Przesunięcie cienia w dół
                    blur_radius: 10.0,             // Rozmycie cienia
                },

                border: Border {
                    // radius: 8.0.into(), // Zaokrąglone rogi, żeby cień ładniej wyglądał
                    ..Border::default()
                },
                ..container::Style::default()
            }
        });

        row![lewa_kolumna, prawa_kolumna].into()
    }
}
