use crate::ui::program_pomniejsze::style_fn::btn::styl_przycisków;
use iced::{
    font, widget::{button, column, container, row, scrollable, text, Space}, Event,
    Task,
};

use crate::ui::program_pomniejsze::kolory::{
    KOLOR_BRILIANT_CRIMSON, KOLOR_COTTON_CANDY, KOLOR_CRIMSON_GLORY, KOLOR_CZCIONKI_SREDNI,
    KOLOR_FLIRT, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE,
};
use chrono::{Local, Timelike};
use futures::channel::mpsc;
use std::path::PathBuf;

use crate::ui::program_pomniejsze::ui_dds::{view_dds, StronyDds};
use crate::ui::program_pomniejsze::ui_zdjecia_edycja::WybraneOknoEdycjiZdjęć;
use binarka::pakowanie_plikow::ogarnianie_eksportu;
use binarka::rozpakowywanie_plikow::ogarnianie_dekompresji;
use enumy::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI};
use enumy::dane_do_przetwarzania::{DaneDoBathKonwersjaZdjec, DaneDoDekompresjaPlików, DaneDoKompresjaPlików, DaneDoPakowaniaDds, DaneDoRozpakowaniaDds, DaneDoŁączeniaZdjęć};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds, FILTERFOTO};
pub(crate) use enumy::enums_structs_io::{LogPakowanie, LogPrzetwarzanieFot, LogRozpakowywanie};
// use crate::ui::program_pomniejsze::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI, KOLOR_BRILIANT_CRIMSON, KOLOR_CRIMSON_GLORY, KOLORFLIRT, KOLOR_PEACH_PUFF};
use enumy::lang::{odmiana_liczbowa, zmieniacz_ilosci_bajtow};
use enumy::opcje::{FolderCzyPlik, JpgQuant, JpgSamplingFac, OptFormatDds, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptInterpolacja, OptKompresjaDds, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych, OptRozszerzeniaPlikówZdjęciowychPojedyncze, OptRozszerzeniaPlikówZdjęciowychZnacznik, OptUIWariantPodstrony};
use enumy::statusy::{LogTxDoBathKonwersjaZdjęć, LogTxDoDekompresjiPliku, LogTxDoKompresjiPliku};
pub(crate) use enumy::wybranie_jezykowe::{DevToolsMenu, WybórJęzyka};
use iced::widget::{image, shader, stack, tooltip, Column, Row};
use iced::{Border, Color, Element, Length};
use iced_core::{Shadow, Theme, Vector};
use enumy::inne_ui::{CheckActiveProcess, CheckerDoZbiorowePrzetwarzanieZdjęć, StanKlikaczyDoLaczeniaZdjec, WybranyFormatZdjecia};
use crate::ui::wiadomosci::message_ui::Message;
use laczenie_plikow::laczenie_fot_struct_enums::{fn_do_laczenia_fot, LogTxDoŁączeniaZdjęć};
use zbiorowa_konwersja_zdjec::zmiana_fot::ogarnianie_foto;


#[allow(dead_code)]
pub struct Program {
    ui_main_wariant_podstrony: OptUIWariantPodstrony,
    pub(crate) dane_temp_do_łączenia_zdjęć: DaneDoŁączeniaZdjęć,
    dane_temp_do_kompresji_plików: DaneDoKompresjaPlików,
    dane_temp_do_dekompresji_plików: DaneDoDekompresjaPlików,

    // UI state
    pub(crate) log_prawe_okno: Vec<String>,
    status_pakowanie_log: LogPakowanie,
    status_rozpakowywania_log: LogRozpakowywanie,
    pub(crate) status_zmiany_fot_log: LogPrzetwarzanieFot,
    pub(crate) status_dds_pakowanie: LogPakowaniaDds,
    status_dds_rozpakowywanie: LogRozpakowywanieDds,
    pub(crate) checker_bool_status_procesow:CheckActiveProcess,
    checker_bool_status_kompresja: bool,
    checker_bool_status_dekompresja: bool,
    checker_bool_status_zbiorowe_przetwarzanie_zdjęć: bool,
    pub(crate) checker_bool_status_łączenie_zdjęć: bool,
    pub(crate) checker_bool_status_dds: (bool, bool),
    ui_main_wariant_dev: bool,
    ui_ustawienia: DevToolsMenu,
    ui_zbiorowe_przetwarzanie_zdjęć_podmenu: WybraneOknoEdycjiZdjęć,
    pub(crate) ui_dds_podmenu: StronyDds,
    zdjecia_edycja_co_jest_wybrane: (bool, bool),
    pub(crate) zdjecia_edycja_co_jest_na_out: bool,
    pub(crate) dane_temp_do_zbiorowe_przetwarzanie_zdjęć: DaneDoBathKonwersjaZdjec,
    do_nothing: bool,
    checker_do_zbiorowe_przetwarzanie_zdjęć: CheckerDoZbiorowePrzetwarzanieZdjęć,
    main_process_check: bool,
    startowy_jezyk: String,
    pub(crate) stan_boolean_do_laczenia_zdjec: StanKlikaczyDoLaczeniaZdjec,
    pub(crate) stan_boolean_do_dds: StanKlikaczyDoLaczeniaZdjec,
    pub(crate) dane_temp_do_pakowania_dds: DaneDoPakowaniaDds,
    pub(crate) dane_temp_do_rozpakowywania_dds: DaneDoRozpakowaniaDds,
    pub(crate) dane_temp_do_rozpakowania_dds_formaty_zdjec: WybranyFormatZdjecia,
    halp_menu:bool,
    uchwyt_szumu: image::Handle,

}
// pub fn generuj_szum() -> image::Handle {
//     let width = 1024;
//     let height = 1024;
//     let total_pixels = width * height;
//     let mut pixels = Vec::with_capacity((total_pixels * 4) as usize);
//
//     for i in 0..total_pixels {
//         // Prosty generator pseudo-losowy, żeby nie dodawać crate'a `rand`
//         let seed = i as f32 * 12.9898;
//         let noise_val = ((seed.sin() * 43758.5453).fract() * 255.0) as u8;
//
//         // Kanały R, G, B
//         pixels.push(noise_val);
//         pixels.push(noise_val);
//         pixels.push(noise_val);
//
//         // Kanał Alpha - ustawiamy na bardzo niski (np. 10 na 255)
//         // To jest klucz! Tu sterujesz przezroczystością szumu.
//         pixels.push(8);
//     }
//
//     // Tworzymy uchwyt z wygenerowanych pikseli
//     image::Handle::from_rgba(width, height, pixels)
// }
use noise::{NoiseFn, Fbm, Perlin};
use rand::Rng;


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
                ui_main_wariant_podstrony: OptUIWariantPodstrony::Pakowanie,
                ui_main_wariant_dev: false,
                ui_ustawienia: DevToolsMenu::UstawieniaJęzyka {
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
                status_pakowanie_log: LogPakowanie::default(),
                status_rozpakowywania_log: LogRozpakowywanie::default(),
                status_zmiany_fot_log: Default::default(),
                status_dds_pakowanie: LogPakowaniaDds{w_trakcie: 0, koniec:"".to_string(), err:"".to_string()},
                status_dds_rozpakowywanie: Default::default(),
                checker_bool_status_procesow:CheckActiveProcess::ProcessŻodyn,
                checker_bool_status_kompresja: false,
                checker_bool_status_dekompresja: false,
                checker_bool_status_zbiorowe_przetwarzanie_zdjęć: false,
                checker_bool_status_łączenie_zdjęć: false,
                ui_zbiorowe_przetwarzanie_zdjęć_podmenu: WybraneOknoEdycjiZdjęć::Ścieżki,
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
                checker_do_zbiorowe_przetwarzanie_zdjęć: CheckerDoZbiorowePrzetwarzanieZdjęć {
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrany: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_progres: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_kolor: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_alpha: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_alpha_kolor_16b: (0, 0, 0),
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_rgb: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_jpg_wybrany_bw: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_kompresja: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bit: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bit: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_8bita: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_16bita: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bit: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bit: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l8bita: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_png_wybrane_l16bita: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_szary: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_16b: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_24b: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_tga_wybrany_32b: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana: (true, false, false, false, false, false),
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_32: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_64: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_128: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_256: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_512: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_1k: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_2k: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_4k: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_6k: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_8k: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16k: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_org: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_lossless: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany_rgb: true,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_webp_wybrany_alpha: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_brak: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_zstd: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_bzip2: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_wybrany: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_ff_kompresja_xz: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_32b: false,
                    zbiorowe_przetwarzanie_zdjec_rozszerzenie_qoi_wybrany_24b: false,
                },
                main_process_check: false,
                startowy_jezyk: locale.clone(),

                stan_boolean_do_laczenia_zdjec: StanKlikaczyDoLaczeniaZdjec {
                    obraz_r_wybrany: false,
                    obraz_g_wybrany: false,
                    obraz_b_wybrany: false,
                    obraz_a_wybrany: false,
                    sciezka_out_wybrana: false,
                    jpg_wybrany: true,
                    jpg_jakosc: 90,
                    png_wybrany: false,
                    png_wybrane_8bit: true,
                    png_wybrane_16bit: false,
                    png_wybrane_8bita: false,
                    png_wybrane_16bita: false,
                    png_kompresja: 3,
                    tga_wybrany: false,
                    tga_wybrany_32b: false,
                    tga_wybrany_24b: false,
                    tga_wybrany_16b: false,
                    webp_wybrany: false,
                    webp_lossless: false,
                    webp_wybrany_rgb: true,
                    webp_wybrany_alpha: false,
                    webp_jakosc: 90,
                    ff_wybrany: false,
                    ff_kompresja_brak: true,
                    ff_kompresja_zstd: false,
                    ff_kompresja_bzip2: false,
                    ff_kompresja_xz: false,
                    qoi_wybrany: false,
                    qoi_wybrany_32b: false,
                    qoi_wybrany_24b: true,
                },
                stan_boolean_do_dds: StanKlikaczyDoLaczeniaZdjec {
                    obraz_r_wybrany: false,
                    obraz_g_wybrany: false,
                    obraz_b_wybrany: false,
                    obraz_a_wybrany: false,
                    sciezka_out_wybrana: false,
                    jpg_wybrany: true,
                    jpg_jakosc: 90,
                    png_wybrany: false,
                    png_wybrane_8bit: false,
                    png_wybrane_16bit: false,
                    png_wybrane_8bita: false,
                    png_wybrane_16bita: false,
                    png_kompresja: 3,
                    tga_wybrany: false,
                    tga_wybrany_32b: false,
                    tga_wybrany_24b: false,
                    tga_wybrany_16b: false,
                    webp_wybrany: false,
                    webp_lossless: false,
                    webp_wybrany_rgb: false,
                    webp_wybrany_alpha: false,
                    webp_jakosc: 0,
                    ff_wybrany: false,
                    ff_kompresja_brak: false,
                    ff_kompresja_zstd: false,
                    ff_kompresja_bzip2: false,
                    ff_kompresja_xz: false,
                    qoi_wybrany: false,
                    qoi_wybrany_32b: false,
                    qoi_wybrany_24b: false,
                },
                dane_temp_do_pakowania_dds: DaneDoPakowaniaDds {
                    ścieżka_wejściowa: PathBuf::new(),
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
                },
                checker_bool_status_dds: (false, false),
                dane_temp_do_rozpakowania_dds_formaty_zdjec: WybranyFormatZdjecia::Jpg,
                halp_menu: false,
                uchwyt_szumu: generuj_ziarno(),
            },
            Task::batch(Vec::from([
                Task::done(Message::InitLogStartowy),
                font::load(FONT_DEFAULT).map(|_| Message::None),
                font::load(FONT_KOREAN).map(|_| Message::None),
                font::load(FONT_JAPANESE).map(|_| Message::None),
                font::load(FONT_THAI).map(|_| Message::None),
            ])),
        )
    }

    // 2. POPRAWKA: Sygnatura UPDATE (Iced 0.13+ oczekuje 2 argumentów: &mut self i Message)
    // Usunąłem trzeci argument, który generował błąd E0593
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let aktualny_jezyk = match self.ui_ustawienia {
            DevToolsMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => WybórJęzyka::EN,
        };
        self.main_process_check = self.checker_bool_status_kompresja
            || self.checker_bool_status_dekompresja
            || self.checker_bool_status_zbiorowe_przetwarzanie_zdjęć
            || self.checker_bool_status_łączenie_zdjęć;

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
                    self.startowy_jezyk // tutaj masz już dostęp przez self
                );
                self.log_prawe_okno.push(powitanie);

            }

            Message::UsuńLogi => self.log_prawe_okno = Vec::new(),
            Message::DevZmienJezyk(nowy) => {
                self.ui_ustawienia = DevToolsMenu::UstawieniaJęzyka { jezyk: nowy };
                // KLUCZOWE: Po zmianie języka odświeżamy listę w ComboBoxie
                // self.dane_temp_do_kompresji_plików.odswiez_tlumaczenia_kompresja(&nowy);
            }

            Message::OptKompresjaPlikówPoziomKompresjiZstdChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom = OptKompresjaPlikówPoziomKompresjiZstd::WSIOKOMPRESJI
                    .iter()
                    .find(|p| {
                        // Używamy tego samego tłumacza (jezyk), którego używamy w view
                        aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                    });

                if let Some(p) = znaleziony_poziom {
                    self.dane_temp_do_kompresji_plików.kompresja = *p;
                    self.log_prawe_okno
                        .push(format!("[System] Zmieniono kompresję na: {:?}", p));
                }
            }
            Message::PoziomInterpolacjiChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom = OptInterpolacja::WSIOINTERPOLACJI.iter().find(|p| {
                    // Używamy tego samego tłumacza (jezyk), którego używamy w view
                    aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                });

                if let Some(p) = znaleziony_poziom {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.inter = *p;
                    self.log_prawe_okno
                        .push(format!("[System] Zmieniono interpolację na: {:?}", p));
                }
            }
            Message::FilterChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom =
                    OptKompresjaPlikówFiltracjaPlików::WSIOPLIKOW
                        .iter()
                        .find(|p| {
                            // Używamy tego samego tłumacza (jezyk), którego używamy w view
                            aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                        });

                if let Some(p) = znaleziony_poziom {
                    self.dane_temp_do_kompresji_plików.filtracja = *p;
                    self.log_prawe_okno
                        .push(format!("[System] Zmieniono filtrację na: {:?}", p));
                }
            }

            Message::OptKompresjaPlikówPoziomKompresjiZstdSearch(_) => {} // Wymagane przez ComboBox
            Message::ZmienWariant(w) => {
                self.ui_main_wariant_podstrony = w;

                match w {
                    OptUIWariantPodstrony::Pakowanie => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (true, false, false, false, false, false)
                    }
                    OptUIWariantPodstrony::Rozpakowanie => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (false, true, false, false, false, false)
                    }
                    OptUIWariantPodstrony::KonwersjaFoto => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (false, false, true, false, false, false)
                    }
                    OptUIWariantPodstrony::DaneDoŁączeniaZdjęćo => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (false, false, false, true, false, false)
                    }
                    OptUIWariantPodstrony::ObslugaDds => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (false, false, false, false, true, false)
                    }
                    // OptUIWariantPodstrony::Ustawienia=>self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana = (false, false, false, false, false),
                    // OptUIWariantPodstrony::Logi=>self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana = (false, false, false, false, false),
                    OptUIWariantPodstrony::Dev => {
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana =
                            (false, false, false, false, false, true)
                    }
                };
            }
            Message::PakowaniePathChanged(s) => {
                self.dane_temp_do_kompresji_plików.ścieżka_in = PathBuf::from(s)
            }
            Message::PakowanieOutPathChanged(s) => {
                self.dane_temp_do_kompresji_plików.ścieżka_out = PathBuf::from(s)
            }
            Message::StatusDekompresjaPlikówDekompresjaPlikPathChanged(s) => {
                self.dane_temp_do_dekompresji_plików.ścieżka_pliku = PathBuf::from(s)
            }
            Message::StatusDekompresjaPlikówDekompresjaOutPathChanged(s) => {
                self.dane_temp_do_dekompresji_plików.ścieżka_docelowa = PathBuf::from(s)
            }
            Message::DoNothingxD(xx) => self.do_nothing = xx,
            Message::DoNothingU8xD(_) => {}
            Message::DoNothingStringxD(_x) => self.do_nothing = false,
            Message::ZdjeciaZmienPlikInPathChanged(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::from(s);
                self.zdjecia_edycja_co_jest_wybrane = (false, true)
            }
            Message::ZdjeciaZmienFolderInPathChanged(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::from(s);
                self.zdjecia_edycja_co_jest_wybrane = (true, false)
            }
            Message::ZdjeciaZmienFolderOutPathTenSam(zdjecia_edycja_co_jest_na_out) => {
                self.zdjecia_edycja_co_jest_na_out = zdjecia_edycja_co_jest_na_out;
                let xoxo = if self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.is_file() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                        .ścieżka_wejściowa
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.clone())
                } else {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa.clone()
                };
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = xoxo
            }
            Message::ZdjeciaZmienFolderOutPathChanged(s) => {
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = PathBuf::from(s)
            }

            Message::NazwaPaczkiChanged(s) => self.dane_temp_do_kompresji_plików.nazwa = s,
            Message::LogDodaj(txt) => self.log_prawe_okno.push(txt),
            Message::ResetLogPakowanie => self.status_pakowanie_log = LogPakowanie::default(),
            Message::ResetLogRozpakowania => self.status_pakowanie_log = LogPakowanie::default(),
            Message::ZmienMenuEdycjiNaSciezki(nowy_stan_menu) => {
                self.ui_zbiorowe_przetwarzanie_zdjęć_podmenu = nowy_stan_menu
            }
            Message::FiltracjaChanged(f) => {
                self.dane_temp_do_kompresji_plików.filtracja = f;
            }
            Message::WybierzFolderInPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_kompresji_plików.ścieżka_in = path;
                }
            }
            Message::ResetujStanWejsciowychSciezekEdycjaFoto => {
                self.zdjecia_edycja_co_jest_wybrane = (false, false);
                self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = PathBuf::new();
            }
            Message::WybierzFolderOutPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_kompresji_plików.ścieżka_out = path;
                }
            }
            Message::WybierzPlikExPakowanie => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Plik .jrzs", &["jrzs"])
                    .pick_file()
                {
                    self.dane_temp_do_dekompresji_plików.ścieżka_pliku = path;
                }
            }
            Message::WybierzFolderOutExPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_dekompresji_plików.ścieżka_docelowa = path;
                }
            }
            Message::WybierzPlikInFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy", &FILTERFOTO)
                    .pick_file()
                {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = path;
                    self.zdjecia_edycja_co_jest_wybrane = (false, true)
                }
            }
            Message::WybierzFolderInFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wejściowa = path;
                    self.zdjecia_edycja_co_jest_wybrane = (true, false)
                }
            }
            Message::WybierzFolderOutFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć.ścieżka_wyjściowa = path;
                }
            }
            Message::UruchomProcesPakowania => {
                self.status_pakowanie_log = LogPakowanie::default();
                self.checker_bool_status_kompresja = true;
                let zestaw = self.dane_temp_do_kompresji_plików.clone();

                let (tx, rx) = mpsc::channel::<LogTxDoKompresjiPliku>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = ogarnianie_eksportu(zestaw, tx).await;
                            })
                            .await
                    },
                    |_| Message::Nic,
                );

                let nasluchiwanie = Task::run(rx, Message::PostepPakowania);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }
            Message::UruchomProcesRozpakowania => {
                self.checker_bool_status_dekompresja = true;
                let zestaw = self.dane_temp_do_dekompresji_plików.clone();

                let (tx, rx) = mpsc::channel::<LogTxDoDekompresjiPliku>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle
                            .spawn(async move {
                                let _ = ogarnianie_dekompresji(zestaw, tx).await;
                            })
                            .await
                    },
                    |_| Message::Nic,
                );

                let nasluchiwanie = Task::run(rx, Message::PostepRozpakowania);

                return Task::batch(Vec::from([operacja, nasluchiwanie]));
            }

            Message::PostepPakowania(progres) => {
                match progres {
                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZnalezionePliki { pliki } => {
                        // self.status_pakowanie_log.zbieranie_plików_max = pliki;
                        // self.log_prawe_okno.push(format!("[{:?}] Znaleziono {} plików do przetworzenia.", etap, pliki));
                        if self.status_pakowanie_log.zbieranie_plików_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [Zbieranie plików] Rozpoczęte",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_pakowanie_log.zbieranie_plików_licznik += 1;
                        }

                        // 1. Aktualizuj tekst o plikach TYLKO jeśli przyszły (Some)
                        if let Some(p) = pliki {
                            self.status_pakowanie_log.zbieranie_plików = format!(
                                "[Zbieranie plików] Znaleziono {} {} do przetworzenia.",
                                p,
                                aktualny_jezyk
                                    .t(&format!("files_counting_{}", odmiana_liczbowa(p)))
                            );
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self
                            .status_pakowanie_log
                            .zbieranie_plików
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_pakowanie_log.zbieranie_plików =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Znajdowanie Plików"));
                        // self.status_pakowanie_log.zbieranie_plików = format!("[Zbieranie plików] Znaleziono {} plików do przetworzenia. --{:?}", pliki.unwrap(), status);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówPakowanie { aktualny, suma } => {
                        // Obliczamy procent dla etapu pakowania
                        // self.procent_postepu = (aktualny as f32 / suma as f32) * 100.0;
                        // Logujemy tylko co jakiś czas, żeby nie zapchać UI tysiącami linii
                        if self.status_pakowanie_log.pakowanie_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [Pakowanie] Rozpoczęte",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_pakowanie_log.pakowanie_licznik += 1;
                        }

                        // self.log_prawe_okno.push(format!("[{:?}] Pakowanie: {}/{}", etap, aktualny, suma));
                        // 1. Aktualizujemy bazę napisu TYLKO jeśli przyszły obie liczby (Some)
                        if let (Some(akt), Some(sum)) = (aktualny, suma) {
                            self.status_pakowanie_log.pakowanie = format!(
                                "[Pakowanie] Pakowanie: {} z {} {}",
                                akt,
                                sum,
                                aktualny_jezyk
                                    .t(&format!("files_counting_{}", odmiana_liczbowa(sum)))
                            );
                        }

                        // 2. Czyścimy stary status i doklejamy nowy do tego, co jest w zmiennej
                        let baza = self
                            .status_pakowanie_log
                            .pakowanie
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_pakowanie_log.pakowanie =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Pakowanie"));
                        // self.status_pakowanie_log.pakowanie = format!("[Pakowanie] Pakowanie: {}/{}   ---{:?}", aktualny.unwrap(), suma.unwrap(), status);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji { procent } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_pakowanie_log.kompresja_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [Kompresja] Rozpoczęta",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_pakowanie_log.kompresja_licznik += 1;
                        }

                        // self.log_prawe_okno.push(format!("[{:?}] Kompresja: {}%", etap, procent));
                        // self.log_prawe_okno.push(format!("{} [Kompresja] Rozpoczęta", Local::now().format("%H:%M:%S")));
                        if let Some(p) = procent {
                            self.status_pakowanie_log.kompresja =
                                format!("[Kompresja] Kompresja {}%.", p);
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self
                            .status_pakowanie_log
                            .kompresja
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_pakowanie_log.kompresja =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Kompresja"));
                        // self.status_pakowanie_log.kompresjaa = format!("[Kompresja] Kompresja: {}%   ---{:?}", procent.unwrap(), status);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesSzyfrowania { procent } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_pakowanie_log.szyfrowanie_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [Szyfrowanie] Rozpoczęte",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_pakowanie_log.szyfrowanie_licznik += 1;
                        }

                        // self.log_prawe_okno.push(format!("[{:?}] Rozpoczęto szyfrowanie XOR...", etap));
                        // self.log_prawe_okno.push(format!("{} [Szyfrowanie] Rozpoczęte", Local::now().format("%H:%M:%S")));
                        if let Some(p) = procent {
                            self.status_pakowanie_log.szyfrowanie =
                                format!("[Szyfrowanie] Szyfrowanie {}%.", p);
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self
                            .status_pakowanie_log
                            .szyfrowanie
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_pakowanie_log.szyfrowanie =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Szyfrowanie"));
                        // self.status_pakowanie_log.szyfrowanie = format!("[Szyfrowanie] Rozpoczęto szyfrowanie XOR: {}%   --{:?}", procent.unwrap(),status);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówZakonczono { czas } => {
                        self.checker_bool_status_kompresja = false;
                        // self.procent_postepu = 100.0;
                        // self.log_prawe_okno.push(format!("--- PROCES ZAKOŃCZONY [{:?}] ---", etap));
                        // self.log_prawe_okno.push(format!("Łączny czas: {}", czas));

                        self.log_prawe_okno.push(format!(
                            "{} [Pakowanie] Zakończone, minęło: {}",
                            Local::now(),
                            czas
                        ));
                        self.status_pakowanie_log.czas =
                            format!("Zakończono w łącznym czasie: {}", czas);
                    }

                    LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(err) => {
                        self.checker_bool_status_kompresja = false;
                        self.log_prawe_okno.push(format!(
                            "!!! {}: {} !!!",
                            aktualny_jezyk.t("log_status_critical_error"),
                            err
                        ));
                        self.status_pakowanie_log.błąd = format!(
                            "!!! {}: {} !!!",
                            aktualny_jezyk.t("log_status_critical_error"),
                            err
                        );
                    }
                }
            }


            Message::PostepRozpakowania(progres) => {
                match progres {
                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówZbieraniePlików {
                        procent,
                    } => {
                        if self.status_rozpakowywania_log.kontrola_pliku_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [Analiza] Rozpoczęte",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_rozpakowywania_log.kontrola_pliku_licznik += 1;
                        }

                        if let Some(p) = procent {
                            self.status_rozpakowywania_log.kontrola_pliku =
                                format!("[Analiza] Przetworzono{}%", p);
                        }

                        let baza = self
                            .status_rozpakowywania_log
                            .kontrola_pliku
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_rozpakowywania_log.kontrola_pliku =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Znajdowanie plików"));
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDeszyfracja { procent } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_rozpakowywania_log.deszyfrowanie_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [StatusDekompresjaPlikówDeszyfracja] Rozpoczęta",
                                Local::now().format("%H:%M:%S")
                            ));
                            self.status_rozpakowywania_log.deszyfrowanie_licznik += 1;
                        }

                        if let Some(p) = procent {
                            self.status_rozpakowywania_log.deszyfrowanie =
                                format!("[Deszyfrowanie] postęp {}%.", p);
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self
                            .status_rozpakowywania_log
                            .deszyfrowanie
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_rozpakowywania_log.deszyfrowanie = format!(
                            "{}   ---{}",
                            baza,
                            aktualny_jezyk.t("StatusDekompresjaPlikówDeszyfracja")
                        );
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDekompresja {
                        pamięć,
                    ..
                    } => {
                        if self
                            .status_rozpakowywania_log
                            .StatusDekompresjaPlikówDekompresja_licznik
                            == 0
                        {
                            self.log_prawe_okno.push(format!(
                                "{} [{}] {}",
                                Local::now().format("%H:%M:%S"),
                                aktualny_jezyk.t("log_status_operation_decompression"),
                                aktualny_jezyk.t("log_status_started")
                            ));
                            self.status_rozpakowywania_log
                                .StatusDekompresjaPlikówDekompresja_licznik += 1;
                        }

                        if let Some(p) = pamięć {
                            self.status_rozpakowywania_log
                                .StatusDekompresjaPlikówDekompresja = format!(
                                "[{}] Zdekompresowano {}.",
                                aktualny_jezyk.t("log_status_operation_decompression"),
                                zmieniacz_ilosci_bajtow(p),
                            );
                        }

                        let baza = self
                            .status_rozpakowywania_log
                            .StatusDekompresjaPlikówDekompresja
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_rozpakowywania_log
                            .StatusDekompresjaPlikówDekompresja = format!(
                            "{}   ---{}",
                            baza,
                            aktualny_jezyk.t("StatusDekompresjaPlikówDekompresja")
                        );
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówRozpakowywanie {
                        aktualny,
                        suma,
                    } => {
                        if self.status_rozpakowywania_log.rozpakowanie_licznik == 0 {
                            self.log_prawe_okno.push(format!(
                                "{} [{}] Rozpoczęte",
                                Local::now().format("%H:%M:%S"),
                                aktualny_jezyk.t("log_status_operation_unpacking")
                            ));
                            self.status_rozpakowywania_log.rozpakowanie_licznik += 1;
                        }

                        if let (Some(akt), Some(sum)) = (aktualny, suma) {
                            self.status_rozpakowywania_log.rozpakowanie = format!(
                                "[{}] postęp: {} z {} {}",
                                aktualny_jezyk.t("log_status_operation_unpacking"),
                                akt,
                                sum,
                                aktualny_jezyk.t("files_counting_numbers_3")
                            );
                        }

                        let baza = self
                            .status_rozpakowywania_log
                            .rozpakowanie
                            .split("   ---")
                            .next()
                            .unwrap_or("");
                        self.status_rozpakowywania_log.rozpakowanie =
                            format!("{}   ---{}", baza, aktualny_jezyk.t("Rozpakowywanie"));
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówZakończenie { czas } => {
                        self.checker_bool_status_dekompresja = false;
                        // self.procent_postepu = 100.0;
                        // self.log_prawe_okno.push(format!("--- PROCES ZAKOŃCZONY [{:?}] ---", etap));
                        // self.log_prawe_okno.push(format!("Łączny czas: {}", czas));

                        self.log_prawe_okno.push(format!(
                            "{} [{}] {}, {}: {}",
                            Local::now().format("%H:%M:%S"),
                            aktualny_jezyk.t("log_status_operation_unpacking"),
                            aktualny_jezyk.t("log_status_finished"),
                            aktualny_jezyk.t("log_status_in_time").to_lowercase(),
                            czas
                        ));
                        self.status_rozpakowywania_log.czas =
                            format!("Zakończono w łącznym czasie: {}", czas);
                    }

                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówBłąd(err) => {
                        self.checker_bool_status_dekompresja = false;
                        self.log_prawe_okno.push(format!(
                            "!!! {}: {} !!!",
                            aktualny_jezyk.t("log_status_critical_error"),
                            err
                        ));
                        self.status_rozpakowywania_log.błąd = format!(
                            "!!! {}: {} !!!",
                            aktualny_jezyk.t("log_status_critical_error"),
                            err
                        );
                    }
                }
            }
            Message::DopasujRozdzielczosci(rozdzielczosc) => {
                // 1. Wyciągamy referencję do booleana w stanie, który odpowiada temu enumowi
                // Musimy wiedzieć, czy przycisk właśnie włączamy, czy wyłączamy.
                let stan_bool = match rozdzielczosc {
                    OptRozdzielczościObrazów::R16 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16
                    }
                    OptRozdzielczościObrazów::R32 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_32
                    }
                    OptRozdzielczościObrazów::R64 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_64
                    }
                    OptRozdzielczościObrazów::R128 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_128
                    }
                    OptRozdzielczościObrazów::R256 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_256
                    }
                    OptRozdzielczościObrazów::R512 => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_512
                    }
                    OptRozdzielczościObrazów::R1k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_1k
                    }
                    OptRozdzielczościObrazów::R2k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_2k
                    }
                    OptRozdzielczościObrazów::R4k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_4k
                    }
                    OptRozdzielczościObrazów::R6k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_6k
                    }
                    OptRozdzielczościObrazów::R8k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_8k
                    }
                    OptRozdzielczościObrazów::R16k => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_16k
                    }
                    OptRozdzielczościObrazów::Oryginalna => {
                        &mut self
                            .checker_do_zbiorowe_przetwarzanie_zdjęć
                            .zbiorowe_przetwarzanie_zdjec_rozszerzenie_rozdzielczości_wybrane_org
                    }
                };

                // 2. Odwracamy stan (kliknięcie przełącza on/off)
                *stan_bool = !*stan_bool;

                // 3. Aktualizujemy wektor z opcjami (synchronizacja)
                if *stan_bool {
                    // Jeśli włączyliśmy przycisk -> dodaj do listy, jeśli jeszcze go tam nie ma
                    if !self
                        .dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                        .opcje_rozdzielczości
                        .contains(&rozdzielczosc)
                    {
                        self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                            .opcje_rozdzielczości
                            .push(rozdzielczosc);
                    }
                } else {
                    // Jeśli wyłączyliśmy przycisk -> usuń z listy
                    self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć
                        .opcje_rozdzielczości
                        .retain(|x| x != &rozdzielczosc);
                }
            }
            Message::ZbiorowePrzetwarzanieZdjęć(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_zbiorowe_przetwarzanie_zdjec(msg)
                    .map(|m| Message::ZbiorowePrzetwarzanieZdjęć(m));
            },

            Message::ŁączenieZdjęć(msg) => {
                // let _ =self.update_message_łączenie_zdjęć(msg).map(Message::ŁączenieZdjęć);
                return self.update_message_łączenie_zdjęć(msg)
                                .map(|m| Message::ŁączenieZdjęć(m));
            },
            Message::Dds(msg) => {return self.update_message_dds(msg)
                .map(|m| Message::Dds(m)); },

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
                    self.halp_menu = !self.halp_menu;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno.push(format!("Halp Mode: {}", self.halp_menu));
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
    pub fn view(&self) -> Element<'_, Message> {
        let aktualny_jezyk = match self.ui_ustawienia {
            DevToolsMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => WybórJęzyka::EN,
        };
        let nakladka_szum = iced::widget::image(self.uchwyt_szumu.clone())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .opacity(1.)
            .content_fit(iced::ContentFit::Cover);

        // --- LEWA STRONA ---
        let przyciski_menu = container(
            Row::new()
                .push(tooltip(
                    button(
                        text(aktualny_jezyk.t("main_toggle_export"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::Pakowanie))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.checker_bool_status_procesow == CheckActiveProcess::ProcessPakowaniePliku,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.0,
                        KOLOR_BRILIANT_CRIMSON,
                    )),
                    aktualny_jezyk.t("main_toggle_export"),
                    tooltip::Position::Bottom,
                ))
                .push(tooltip(
                    button(
                        text(aktualny_jezyk.t("main_toggle_import"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::Rozpakowanie))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.checker_bool_status_procesow == CheckActiveProcess::ProcessRozpakowaniePliku,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.1,
                        KOLOR_FLIRT,
                    )),
                    aktualny_jezyk.t("main_toggle_import"),
                    tooltip::Position::Bottom,
                ))
                .push(tooltip(
                    button(
                        text(aktualny_jezyk.t("main_toggle_photo_compil"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::KonwersjaFoto))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.checker_bool_status_procesow == CheckActiveProcess::ProcessKonwersjaZdjęć,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.2,
                        KOLOR_SPANISH_ORANGE,
                    )),
                    aktualny_jezyk.t("main_toggle_photo_compil"),
                    tooltip::Position::Bottom,
                ))
                .push(
                    tooltip(
                    button(
                        text(aktualny_jezyk.t("main_toggle_photo_merge"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::DaneDoŁączeniaZdjęćo))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.checker_bool_status_procesow == CheckActiveProcess::ProcessŁączenieZdjęć,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.3,
                        KOLOR_PEACH_PUFF,
                    )),
                    aktualny_jezyk.t("main_toggle_photo_merge"),
                    tooltip::Position::Bottom,
                ))
                .push(tooltip(
                    button(
                        text(aktualny_jezyk.t("main_toggle_dds"))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::ObslugaDds))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        self.checker_bool_status_procesow == CheckActiveProcess::ProcessDdsPakowanie || self.checker_bool_status_procesow == CheckActiveProcess::ProcessDdsRozpakowanie,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.4,
                        KOLOR_COTTON_CANDY,
                    )),
                    aktualny_jezyk.t("main_toggle_dds"),
                    tooltip::Position::Bottom,
                ))
                .push(tooltip(
                    button(
                        text("Dev")
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center(),
                    )
                    .on_press(Message::ZmienWariant(OptUIWariantPodstrony::Dev))
                    .height(Length::Fill)
                    .width(Length::FillPortion(5))
                    .style(styl_przycisków(
                        false,
                        self.checker_do_zbiorowe_przetwarzanie_zdjęć.zbiorowe_przetwarzanie_zdjec_rozszerzenie_zakladka_menu_wybrana.5,
                        KOLOR_CRIMSON_GLORY,
                    )),
                    aktualny_jezyk.t("main_toggle_settings"),
                    tooltip::Position::Bottom,
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
        let content_lewy = match self.ui_main_wariant_podstrony {
            OptUIWariantPodstrony::Pakowanie => crate::ui::program_pomniejsze::ui_pakowanie::view_eksport(
                &self.dane_temp_do_kompresji_plików,
                aktualny_jezyk,
                self.status_pakowanie_log.clone(),
                self.checker_bool_status_kompresja,
                self.main_process_check,
            ),
            OptUIWariantPodstrony::Rozpakowanie => crate::ui::program_pomniejsze::ui_rozpakowanie::view_import(
                &self.dane_temp_do_dekompresji_plików,
                aktualny_jezyk,
                self.status_rozpakowywania_log.clone(),
                self.checker_bool_status_dekompresja,
                self.main_process_check,
            ),
            OptUIWariantPodstrony::KonwersjaFoto => {
                crate::ui::program_pomniejsze::ui_zdjecia_edycja::view_foto_change(
                    &self.dane_temp_do_zbiorowe_przetwarzanie_zdjęć,
                    &self.ui_zbiorowe_przetwarzanie_zdjęć_podmenu,
                    &aktualny_jezyk,
                    self.zdjecia_edycja_co_jest_na_out,
                    self.status_zmiany_fot_log.clone(),
                    &self.checker_bool_status_procesow,
                    &self.halp_menu,
                )
            }
            OptUIWariantPodstrony::DaneDoŁączeniaZdjęćo => {
                crate::ui::program_pomniejsze::ui_laczenie_zdjec::view_laczenie(
                    &self.dane_temp_do_łączenia_zdjęć.clone(),
                    &aktualny_jezyk,
                    &self.checker_bool_status_procesow,
                )
            }
            OptUIWariantPodstrony::ObslugaDds => view_dds(
                &self.dane_temp_do_pakowania_dds,
                &self.dane_temp_do_rozpakowywania_dds,
                &self.ui_dds_podmenu,
                &aktualny_jezyk,
                &self.status_dds_pakowanie,
                &self.status_dds_rozpakowywanie,
                &self.checker_bool_status_procesow,
            ),
            OptUIWariantPodstrony::Dev => crate::ui::program_pomniejsze::dev::ui_ustawienia(&self.ui_ustawienia),
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
                                (0.1, 0.11, 0.13),
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
