use iced::{
    font, widget::{button, column, container, row, scrollable, text, Space}, Event,
    Task,
};
use std::collections::HashMap;

// use crate::ui::podmenu_old::ui_dds::{view_dds, StronyDds};
use crate::ui::wiadomosci::message_ui::Message;
use chrono::{Local, Timelike};
use enumy::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI};
use enumy::dane_do_przetwarzania::{DaneKonw, DaneBinUnpak, DaneBinPak, DaneDdsPak, DaneDdsUnpak, DaneMerge};
use enumy::enums_structs_io::{LogPakowaniaDds, LogRozpakowywanieDds};
pub(crate) use enumy::enums_structs_io::{LogPakowanie, LogPrzetwarzanieFot, LogRozpakowywanie};
use enumy::inne_ui::{ActProces, BtnState, ButtonType, DropdownType, ObecnyColorCzcionkiPrzezroczystosci, ObecnyColorTheme, ObecnyColorThemePrzezroczystosci, PrzyciskiGlowneMenu, SliderType, StartBtnStatus, Temp, TextInputType, UiPods, Ustawienia, UstawieniaThemeWsio};
use enumy::opcje::{OptInterpolacja, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::kompresje::{ForAvifKompresja, ForDds, ForDdsKompresja, ForFfKompresja};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszerzenia::{ImgExt, RozszerzeniaPojedyncze, ImgExtTag};
pub(crate) use enumy::wybranie_jezykowe::{UstawieniaMenu, WybórJęzyka};
use iced::widget::{image, stack, Column, Row};
use iced::{Border, Color, Element, Length};
use iced_core::{Shadow, Theme, Vector};
use std::path::PathBuf;
use strum::IntoEnumIterator;


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
    startowy_jezyk: String,
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
use crate::ui::wiadomosci::wiadomosci_do_dds_enum::DdsMsg;
use crate::ui::wiadomosci::wiadomosci_do_laczenia_zdjec_enum::MergeMsg;
use crate::ui::wiadomosci::wiadomosci_do_zbiorowe_przetwarzanie_zdjec_enum::KonwMsg;
use crate::ui::wiadomosci::wiadomosci_pakowanie_bin_enum::BinPakMsg;
use crate::ui::wiadomosci::wiadomosci_rozpakowanie_binarki_enum::BinUnpakMsg;
use crate::widget::button::przycisk_glowne_menu;
use crate::widget::colors_n_stuff::{KOLOR_BRILIANT_CRIMSON, KOLOR_COTTON_CANDY, KOLOR_CRIMSON_GLORY, KOLOR_CZCIONKI_SREDNI, KOLOR_LIGHT_PINK, KOLOR_PEACH_PUFF, KOLOR_SPANISH_ORANGE, KOLOR_TŁA};
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
                dane_merge: DaneMerge {
                    sciezka_r: None,
                    sciezka_g: None,
                    sciezka_b: None,
                    sciezka_a: None,
                    sciezka_out: PathBuf::new(),
                    rozszerzenie: RozszerzeniaPojedyncze::Jpg {
                        jakosc: 90,
                        bit_depth: BdepthJpg::Rgb8,
                        sampling: ForJpgSamplingFac::R444,
                        progresywny: false,
                        quant: ForJpgQuant::Default,
                        scans:4,
                    },
                    tag: ImgExtTag::Jpg,
                    nazwa: String::new(),
                },
                dane_bin_pak: DaneBinPak {
                    ścieżka_in: PathBuf::new(),
                    ścieżka_out: PathBuf::new(),
                    kompresja: OptKompresjaPlikówPoziomKompresjiZstd::Standard,
                    nazwa: String::new(),
                    foldery: true,
                    filtracja: OptKompresjaPlikówFiltracjaPlików::Wszystkie,
                },
                dane_bin_unpak: DaneBinUnpak {
                    ścieżka_pliku: PathBuf::new(),
                    ścieżka_docelowa: PathBuf::new(),
                },
                log_prawe_okno: Vec::new(),
                status_pakowanie_log:  Default::default(),
                status_rozpakowywania_log:  Default::default(),
                status_zmiany_fot_log: Default::default(),
                status_dds_pakowanie:  Default::default(),
                status_dds_rozpakowywanie: Default::default(),
                zdjecia_edycja_co_jest_wybrane: (false, false),
                zdjecia_edycja_co_jest_na_out: false,

                dane_konw:  DaneKonw {
                    ścieżka_wejściowa: Default::default(),
                    ścieżka_wyjściowa: Default::default(),
                    opcje_rozdzielczości: Vec::from([Rozdzielczości::R2k]),
                    noising: None,
                    rozszerzenia:  Vec::from([ImgExt::Jpg {
                        jakosc: 90,
                        progresywny: false,
                        bit_depth: Vec::from([BdepthJpg::Rgb8]),
                        sampling: Default::default(),
                        quant: Default::default(),
                        scans: 4,
                    }]),
                    tag: Vec::from([ImgExtTag::Jpg]),
                    inter: OptInterpolacja::Nearest,
                    alfa_rgb: (0, 0, 0),
                } ,
                do_nothing: false,
                startowy_jezyk: locale.clone(),

                dane_dds_pak: DaneDdsPak {
                    ścieżka_wejściowa: None,
                    ścieżka_wyjściowa: PathBuf::new(),
                    nazwa: String::new(),
                    format: ForDds::DxgiFormatBc7Unorm,
                    kompresja: ForDdsKompresja::Normal,
                },
                dane_dds_rozpak: DaneDdsUnpak {
                    ścieżka_wejściowa: PathBuf::new(),
                    ścieżka_wyjściowa: PathBuf::new(),
                    nazwa: String::new(),
                    rozszerzenie: ImgExt::Jpg {
                        jakosc: 90,
                        progresywny: false,
                        bit_depth: vec![BdepthJpg::Rgb8],
                        sampling: ForJpgSamplingFac::R420,
                        quant: ForJpgQuant::Default,
                        scans: 4,
                    },
                    tag: ImgExtTag::Jpg,
                },
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
                        err_font: Color::from_rgba(1.,0.5,0.5,0.8),
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
                        halp_menu: false,
                        debug_menu: false,
                    },
                    temp: Temp { 
                        act_proc: None,
                        act_window: UiPods::BinPak,
                        start_btn_status: StartBtnStatus{
                            bin_pak: BtnState::LackData,
                            bin_unpak: BtnState::LackData,
                            konwersja: BtnState::LackData,
                            dds_pak: BtnState::LackData,
                            dds_unpak: BtnState::LackData,
                            laczenie: BtnState::LackData,
                        },
                    },
                    btn_state: HashMap::new()
                    /*BUTTON_IDS.iter().map(|&id| (id, BtnState::Active)).collect() */,
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
                    "{}!!!!!\n {}: {}\n  {}: {}\n   {}, \n    {}: {}\n{}\n---------------------------------------",
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
                    self.startowy_jezyk,
                    aktualny_jezyk.t("log_help_menu")
                );
                self.log_prawe_okno.push(powitanie);

            }
            
            Message::UsuńLogi => self.log_prawe_okno = Vec::new(),
            Message::DevZmienJezyk(nowy) => {
                self.ui_ustawienia = UstawieniaMenu::UstawieniaJęzyka { jezyk: nowy };
            }

            Message::TextInputHandling(string, typ) => {
                match typ{
                    TextInputType::BinKompPathIn => {
                        self.dane_bin_pak.ścieżka_in = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinKompPathOut => {
                        self.dane_bin_pak.ścieżka_out = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinKompNazwa => {
                        self.dane_bin_pak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinDekompPathIn => {
                        self.dane_bin_unpak.ścieżka_pliku = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::BinDekompPathOut => {
                        self.dane_bin_unpak.ścieżka_docelowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::KonwPathIn => {
                        self.dane_konw.ścieżka_wejściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::KonwPathOut => {
                        self.dane_konw.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInR => {
                        self.dane_merge.sciezka_r =
                        if string.len() > 0 {
                             Some(PathBuf::from(string))
                        } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInG => {
                        self.dane_merge.sciezka_g =
                        if string.len() > 0 {
                             Some(PathBuf::from(string))
                        } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInB => {
                        self.dane_merge.sciezka_b =
                            if string.len() > 0 {
                                Some(PathBuf::from(string))
                            } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergPathInA => {
                        self.dane_merge.sciezka_a =
                        if string.len() > 0 {
                             Some(PathBuf::from(string))
                        } else { None };
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergePathOut => {
                        self.dane_merge.sciezka_out = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::MergeNazwa => {
                        self.dane_merge.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsPathOut => {
                        self.dane_dds_pak.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsNazwa => {
                        self.dane_dds_pak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozPathIn => {
                        self.dane_dds_rozpak.ścieżka_wejściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozPathOut => {
                        self.dane_dds_rozpak.ścieżka_wyjściowa = PathBuf::from(string);
                        let _ = self.update(Message::ChckStatus);
                    }
                    TextInputType::DdsRozNazwa => {
                        self.dane_dds_rozpak.nazwa = string;
                        let _ = self.update(Message::ChckStatus);
                    }
                }
            }
            
            Message::Dropdown(wybrane, proces) => {

                    match proces {
                        DropdownType::KonwersjaInterpolacja => {
                            if let Some(v) = wybrane.downcast_ref::<OptInterpolacja>() {
                                self.dane_konw.inter = *v;
                            }
                        }
                        DropdownType::BinFilter => {
                            if let Some(v) = wybrane.downcast_ref::<OptKompresjaPlikówFiltracjaPlików>() {
                                self.dane_bin_pak.filtracja = v.clone();
                            }
                        }
                        DropdownType::BinKompresja => {
                            if let Some(v) = wybrane.downcast_ref::<OptKompresjaPlikówPoziomKompresjiZstd>() {
                                self.dane_bin_pak.kompresja = v.clone();
                            }
                        }
                        DropdownType::KonwersjaJpgQuant => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>() {
                                if let Some(ImgExt::Jpg { quant, .. }) = self.dane_konw
                                    .rozszerzenia
                                    .iter_mut()
                                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                                {
                                    *quant = v.clone();
                                }
                            }
                        }
                        DropdownType::KonwersjaJpgSample => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>() {
                                if let Some(ImgExt::Jpg { sampling, .. }) = self.dane_konw
                                    .rozszerzenia
                                    .iter_mut()
                                    .find(|f| matches!(f, ImgExt::Jpg { .. }))
                                {
                                    *sampling = v.clone();
                                }
                            }
                        }
                        DropdownType::KonwersjaKompresjaFf => {
                            if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>() {
                                if let Some(ImgExt::Ff { metoda_kompresji, .. }) = self.dane_konw
                                    .rozszerzenia
                                    .iter_mut()
                                    .find(|f| matches!(f, ImgExt::Ff { .. }))
                                {
                                    *metoda_kompresji = v.clone().ustaw_domyslny_poziom();
                                }
                            }
                        }
                        DropdownType::KonwersjaAvifKompresja => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>() {
                                if let Some(ImgExt::Avif { metoda_kompresji, .. }) = self.dane_konw
                                    .rozszerzenia
                                    .iter_mut()
                                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                                {
                                    *metoda_kompresji = v.clone();
                                }
                            }
                        }
                        DropdownType::KonwersjaAvifChroma => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>() {
                                if let Some(ImgExt::Avif { chroma, .. }) = self.dane_konw
                                    .rozszerzenia
                                    .iter_mut()
                                    .find(|f| matches!(f, ImgExt::Avif { .. }))
                                {
                                    *chroma = v.clone();
                                }
                            }
                        }
                        DropdownType::MergeJpgSample => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>() {
                                if let RozszerzeniaPojedyncze::Jpg { ref mut sampling, .. } = self.dane_merge.rozszerzenie
                                {
                                    *sampling = v.clone();
                                }
                            }
                        }
                        DropdownType::MergeJpgQuant => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>() {
                                if let RozszerzeniaPojedyncze::Jpg { ref mut quant, .. } = self.dane_merge.rozszerzenie
                                {
                                    *quant = v.clone();
                                }
                            }
                        }
                        DropdownType::MergeAvifChroma => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>() {
                                if let RozszerzeniaPojedyncze::Avif { ref mut chroma, .. } = self.dane_merge.rozszerzenie
                                {
                                    *chroma = v.clone();
                                }
                            }
                        }
                        DropdownType::MergeAvifKompresja => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>() {
                                if let RozszerzeniaPojedyncze::Avif { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie
                                {
                                    *metoda_kompresji = v.clone();
                                }
                            }
                        }
                        DropdownType::MergeKompresjaFf => {
                            if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>() {
                                if let RozszerzeniaPojedyncze::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie
                                {
                                    *metoda_kompresji = v.clone().ustaw_domyslny_poziom();
                                }
                            }
                        }
                        DropdownType::DdsPakComp => {
                            if let Some(v) = wybrane.downcast_ref::<ForDdsKompresja>() {
                                self.dane_dds_pak.kompresja= v.clone();
                            }
                        }
                        DropdownType::DdsPakFormat => {
                            if let Some(v) = wybrane.downcast_ref::<ForDds>() {
                                self.dane_dds_pak.format= v.clone();
                            }
                        }
                        DropdownType::DdsKompresjaFf => {
                            if let Some(v) = wybrane.downcast_ref::<ForFfKompresja>() {
                                if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie
                                {
                                    *metoda_kompresji = v.clone().ustaw_domyslny_poziom();
                                }
                            }
                        }
                        DropdownType::DdsAvifKompresja => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifKompresja>() {
                                if let ImgExt::Avif { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie
                                {
                                    *metoda_kompresji = v.clone();
                                }
                            }
                        }
                        DropdownType::DdsAvifChroma => {
                            if let Some(v) = wybrane.downcast_ref::<ForAvifChroma>() {
                                if let ImgExt::Avif { ref mut chroma, .. } = self.dane_dds_rozpak.rozszerzenie
                                {
                                    *chroma = v.clone();
                                }
                            }
                        }
                        DropdownType::DdsJpgQuant => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgQuant>() {
                                if let ImgExt::Jpg { ref mut quant, .. } = self.dane_dds_rozpak.rozszerzenie
                                {
                                    *quant = v.clone();
                                }
                            }
                        }
                        DropdownType::DdsJpgSample => {
                            if let Some(v) = wybrane.downcast_ref::<ForJpgSamplingFac>() {
                                if let ImgExt::Jpg { ref mut sampling, .. } = self.dane_dds_rozpak.rozszerzenie
                                {
                                    *sampling = v.clone();
                                }
                            }
                        }
                    }

            }
            
            Message::Przyciski(typ) => {
                return match typ {
                    ButtonType::BinKompPathIn => {
                        self.update_message_pakowanie_binarki(BinPakMsg::InputPath)
                            .map(Message::PakowanieBinarki)
                    }
                    ButtonType::BinKompPathOut => {
                        self.update_message_pakowanie_binarki(BinPakMsg::OutputPath)
                            .map(Message::PakowanieBinarki)
                    }
                    ButtonType::BinDekompPathIn => {
                        self.update_message_rozpakowanie_binarki(BinUnpakMsg::InputFile)
                            .map(Message::RozpakowanieBinarki)
                    }
                    ButtonType::BinDekompPathOut => {
                        self.update_message_rozpakowanie_binarki(BinUnpakMsg::OutputPath)
                            .map(Message::RozpakowanieBinarki)
                    }
                    ButtonType::KonwPathInFile => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathInFile)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwPathInFolder => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathInFolder)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwPathOut => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::PathOutFolder)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwRozszerzenia => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::Nic)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwJpgProg => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::JpgProg)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwAvifLoss => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::AvifLossyToggle)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::KonwWebpLoss => {
                        self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::WebpLossless)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć)
                    }
                    ButtonType::MergPathInR => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieR)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInG => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieG)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInB => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieB)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergPathInA => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzPlikInFotoLaczenieA)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeRozszerzenia => {
                        self.update_message_łączenie_zdjęć(MergeMsg::Nic)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeJpgProg => {
                        self.update_message_łączenie_zdjęć(MergeMsg::JpgProg)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeAvifLoss => {
                        self.update_message_łączenie_zdjęć(MergeMsg::AvifLossyToggle)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergePathOut => {
                        self.update_message_łączenie_zdjęć(MergeMsg::WybierzFolderOutFotoLaczenie)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::MergeWebpLoss => {
                        self.update_message_łączenie_zdjęć(MergeMsg::ZdjeciaLaczenieZmianalosslessWebp)
                            .map(Message::ŁączenieZdjęć)
                    }
                    ButtonType::DdsPathInFiles => {
                        self.update_message_dds(DdsMsg::PakowaniePathInFiles)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsPathInFolders => {
                        self.update_message_dds(DdsMsg::PakowaniePathInFolders)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsPathOut => {
                        self.update_message_dds(DdsMsg::PakowaniePathOutBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozPathIn => {
                        self.update_message_dds(DdsMsg::RozpakInPathBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozPathOut => {
                        self.update_message_dds(DdsMsg::RozpakOutPathBtn)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsRozszerzenia => {
                        self.update_message_dds(DdsMsg::Nic)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsJpgProg => {
                        self.update_message_dds(DdsMsg::JpgProg)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsWebpLoss => {
                        self.update_message_dds(DdsMsg::WebpLoss)
                            .map(Message::Dds)
                    }
                    ButtonType::DdsAvifLoss => {
                        self.update_message_dds(DdsMsg::AvifLoss)
                            .map(Message::Dds)
                    }
                }
            }

            Message::Slidery(typ,wartość ) => {
                match typ{
                    SliderType::KonwJpgQuality => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            if let ImgExt::Jpg { jakosc, .. } = format {
                                *jakosc = wartość as u8;
                            }
                        }
                    }
                    SliderType::KonwersjaJpgScans => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Jpg { .. }))
                        {
                            if let ImgExt::Jpg { scans, .. } = format {
                                *scans = wartość as u8;
                            }
                        }
                    }
                    SliderType::KonwersjaAvifSpeed => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            if let ImgExt::Avif { speed, .. } = format {
                                *speed = wartość;
                            }
                        }
                    
                    }
                    SliderType::KonwersjaAvifQuality => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Avif { .. }))
                        {
                            if let ImgExt::Avif { lossy, .. } = format {
                                *lossy = Some(wartość as u8);
                            }
                        }
                    }
                    SliderType::KonwersjaPngKompresja => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Png { .. }))
                        {
                            if let ImgExt::Png { kompresja, .. } = format {
                                *kompresja = wartość as u8;
                            }
                        }
                    }
                    SliderType::KonwersjaWebpJakosc => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Webp { .. }))
                        {
                            if let ImgExt::Webp { jakosc, .. } = format {
                                *jakosc = wartość as u8;
                            }
                        }
                    }
                    SliderType::KonwersjaFfZstd => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            if let ImgExt::Ff { metoda_kompresji, .. } = format {
                                *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                            }
                        }
                    }
                    SliderType::KonwersjaFfBzip2 => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            if let ImgExt::Ff { metoda_kompresji, .. } = format {
                                *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                            }
                        }
                    }
                    SliderType::KonwersjaFfXz => {
                        if let Some(format) = self.dane_konw
                            .rozszerzenia
                            .iter_mut()
                            .find(|f| matches!(f, ImgExt::Ff { .. }))
                        {
                            if let ImgExt::Ff { metoda_kompresji, .. } = format {
                                *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                            }
                        }
                    }
                    SliderType::KonwersjaNoising => {
                        if wartość == 0 {
                            self.dane_konw.noising = None;
                        } else {
                            self.dane_konw.noising = Some(wartość as u8);
                        }
                    }
                    SliderType::MergeJpgQuality => {
                        if let RozszerzeniaPojedyncze::Jpg { ref mut jakosc, .. } = self.dane_merge.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::MergeJpgScans => {
                        if let RozszerzeniaPojedyncze::Jpg { ref mut scans, .. } = self.dane_merge.rozszerzenie {
                            *scans = wartość as u8;
                        }
                    }
                    SliderType::MergeAvifSpeed => {
                        if let RozszerzeniaPojedyncze::Avif { ref mut speed, .. } = self.dane_merge.rozszerzenie {
                            *speed = wartość ;
                        }
                    }
                    SliderType::MergeAvifQuality => {
                        if let RozszerzeniaPojedyncze::Avif { ref mut lossy, .. } = self.dane_merge.rozszerzenie {
                            *lossy = Some(wartość as u8) ;
                        }
                    }
                    SliderType::MergePngKompresja => {
                        if let RozszerzeniaPojedyncze::Png { ref mut kompresja, .. } = self.dane_merge.rozszerzenie {
                            *kompresja = wartość as u8 ;
                        }
                    }
                    SliderType::MergeWebpJakosc => {
                            if let RozszerzeniaPojedyncze::Webp { ref mut jakosc, .. } = self.dane_merge.rozszerzenie {
                                *jakosc = wartość as u8 ;
                            }
                    }
                    SliderType::MergeFfZstd => {
                        if let RozszerzeniaPojedyncze::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                        }
                    }
                    SliderType::MergeFfBzip2 => {
                        if let RozszerzeniaPojedyncze::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                        }
                    }
                    SliderType::MergeFfXz => {
                        if let RozszerzeniaPojedyncze::Ff { ref mut metoda_kompresji, .. } = self.dane_merge.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                        }
                    }
                    SliderType::DdsJpgScans => {
                        if let ImgExt::Jpg { ref mut scans, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *scans = wartość as u8;
                        }
                    }
                    SliderType::DdsJpgQuality => {
                        if let ImgExt::Jpg { ref mut jakosc, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::DdsWebpJakosc => {
                        if let ImgExt::Webp { ref mut jakosc, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *jakosc = wartość as u8;
                        }
                    }
                    SliderType::DdsFfBzip2 => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Bzip2(wartość as u8);
                        }
                    }
                    SliderType::DdsFfZstd => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Zstd(wartość as u8);
                        }
                    }
                    SliderType::DdsFfXz => {
                        if let ImgExt::Ff { ref mut metoda_kompresji, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *metoda_kompresji = ForFfKompresja::Xz(wartość as u8);
                        }
                    }
                    SliderType::DdsAvifSpeed => {
                        if let ImgExt::Avif { ref mut speed, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *speed = wartość;
                        }
                    }
                    SliderType::DdsAvifQuality => {
                        if let ImgExt::Avif { ref mut lossy, .. } = self.dane_dds_rozpak.rozszerzenie {
                            *lossy = match lossy{
                                Some(_) =>  Some(wartość as u8),
                                None => None
                            };

                        }
                    }
                    SliderType::DdsPngKompresja => {
                        if let ImgExt::Png { ref mut kompresja, .. } = self.dane_dds_rozpak.rozszerzenie {
                                *kompresja = wartość as u8;
                        }
                    }
                }
            }
            
            Message::Startujemy( proces ) => {
                match proces {
                    ActProces::BinPak => {
                        return self.update_message_pakowanie_binarki(BinPakMsg::Uruchom)
                            .map(Message::PakowanieBinarki);
                    }
                    ActProces::BinUnpak => {
                        return self.update_message_rozpakowanie_binarki(BinUnpakMsg::Uruchom)
                            .map(Message::RozpakowanieBinarki);
                    }
                    ActProces::DdsPak => {}
                    ActProces::DdsUnpak => {}
                    ActProces::Merge => {
                        return self.update_message_łączenie_zdjęć(MergeMsg::Uruchom)
                            .map(Message::ŁączenieZdjęć);
                    }
                    ActProces::Konw => {
                        return self.update_message_zbiorowe_przetwarzanie_zdjec(KonwMsg::Uruchom)
                            .map(Message::ZbiorowePrzetwarzanieZdjęć);
                    }
                }
            }

            Message::ZmienWariant(w) => { self.temat.temp.act_window = w; }


            Message::DoNothingxD(xx) => self.do_nothing = xx,
            Message::DoNothingU8xD(_) => {}
            Message::DoNothingStringxD(_x) => self.do_nothing = false,


            Message::PakowanieBinarki(msg) => {
                return self.update_message_pakowanie_binarki(msg)
                    .map(Message::PakowanieBinarki);
            },
            Message::RozpakowanieBinarki(msg) => {
                return self.update_message_rozpakowanie_binarki(msg)
                    .map(Message::RozpakowanieBinarki);
            },
            Message::ZbiorowePrzetwarzanieZdjęć(msg) => {
                return self.update_message_zbiorowe_przetwarzanie_zdjec(msg)
                    .map(Message::ZbiorowePrzetwarzanieZdjęć);
            },
            Message::ŁączenieZdjęć(msg) => {
                return self.update_message_łączenie_zdjęć(msg)
                    .map(Message::ŁączenieZdjęć);
            },
            Message::Dds(msg) => {
                return self.update_message_dds(msg)
                    .map(Message::Dds);
            },





            Message::ChckStatus => {
                // bin kompresja
                let check_bin_kompresja = self.dane_bin_pak.ścieżka_in.exists() &&
                    self.dane_bin_pak.ścieżka_out.exists() &&
                    !self.dane_bin_pak.nazwa.is_empty();

                self.temat.temp.start_btn_status.bin_pak = match (check_bin_kompresja, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::BinUnpak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                // bin dekompresja
                let check_bin_dekompresja = self.dane_bin_unpak.ścieżka_pliku.is_file() &&
                    self.dane_bin_unpak.ścieżka_docelowa.exists();

                self.temat.temp.start_btn_status.bin_unpak = match (check_bin_dekompresja, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::BinPak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                // konwersja
                let konwersja_bdepth_check = self.dane_konw.rozszerzenia
                    .iter()
                    .all(|format| format.ma_wybrany_bit_depth());
                let check_konwersja = self.dane_konw.ścieżka_wejściowa.exists() &&
                    self.dane_konw.ścieżka_wyjściowa.exists() &&
                    self.dane_konw.opcje_rozdzielczości.len() != 0 &&
                    self.dane_konw.rozszerzenia.len() != 0 &&
                    konwersja_bdepth_check;

                self.temat.temp.start_btn_status.konwersja = match (check_konwersja, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::Konw)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                //łączenie fot
                let check_laczenie =
                    (
                        self.dane_merge.sciezka_r.is_some() ||
                        self.dane_merge.sciezka_g.is_some() ||
                        self.dane_merge.sciezka_b.is_some() ||
                        self.dane_merge.sciezka_a.is_some()
                    ) &&
                        !self.dane_merge.nazwa.is_empty() &&
                        self.dane_merge.sciezka_out.exists();

                self.temat.temp.start_btn_status.laczenie = match (check_laczenie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::Merge)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                // dds pakowanie
                let check_dds_pakowanie =
                    self.dane_dds_pak.ścieżka_wejściowa.as_ref().is_some_and(|xx| xx.len() != 0) &&
                        self.dane_dds_pak.ścieżka_wyjściowa.exists() &&
                        !self.dane_dds_pak.nazwa.is_empty();

                self.temat.temp.start_btn_status.dds_pak = match ( check_dds_pakowanie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::DdsPak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };

                //dds rozpakowanie

                let check_dds_rozpakowanie =
                    self.dane_dds_rozpak.ścieżka_wejściowa.is_file() &&
                        self.dane_dds_rozpak.ścieżka_wyjściowa.exists() ;

                self.temat.temp.start_btn_status.dds_unpak = match ( check_dds_rozpakowanie, self.temat.temp.act_proc.clone()) {
                    (true, None)  => BtnState::Active,
                    (false, None) => BtnState::LackData,
                    (_, Some(ActProces::DdsUnpak)) => BtnState::Processing,
                    (_, Some(_))  => BtnState::Disabled,
                };
            }
            Message::EventOccurred(Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            })) => {

                
                if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                    self.temat.ustawienia.debug_menu = !self.temat.ustawienia.debug_menu;
                    // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                    self.log_prawe_okno
                        .push(format!("Dev Mode: {}", self.temat.ustawienia.debug_menu));
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
            _ => column![text("Opcja jest, lecz UI jeszcze nie").size(50)].into(),
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
