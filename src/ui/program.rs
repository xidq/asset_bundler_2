use iced::{font, widget::{
    button, column, container,
    row, scrollable, text, Space
    ,
}, Event, Task};
use std::fmt;
mod ui_pakowanie;
mod ui_rozpakowanie;
mod lang;
mod dev;
pub(crate) mod czcionki;
mod ui_zdjecia_edycja;
mod podmenu_zdjecia_edycja;
mod style_fn;

use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Radians};


use crate::ui::program::ui_zdjecia_edycja::KOLORSPANISHORANGE;
use chrono::Local;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use std::path::PathBuf;

use crate::foty::zmiana_fot::{ogarnianie_foto, EdycjaZdjęć, Interpolacja, Obraz, ObrazTGA, PostepMieleniaZdjec, Rozdzielczości, Rozszerzenia};
use crate::io::enums_structs_io::{DaneDoDekompresji, DaneDoKompresji, FiltracjaPlików, PoziomKompresji, Progress, ProgressDe, StrukturaFolderów};
use crate::io::export_with_compression::ogarnianie_dekompresji;
use crate::io::import_for_compression::ogarnianie_eksportu;
use crate::styl_przycisków;
use crate::ui::program::czcionki::{FONT_DEFAULT, FONT_JAPANESE, FONT_KOREAN, FONT_THAI, KOLORBRILIANTCRIMSON, KOLORCRIMSONGLORY, KOLORFLIRT, KOLORPEACHPUFF};
use crate::ui::program::lang::{odmiana_liczbowa, status_to_string, zmieniacz_ilosci_bajtow};
use crate::ui::program::ui_zdjecia_edycja::WybraneOknoEdycjiZdjęć;
use iced::widget::{Column, Row};
use iced::{Border, Color, Element, Length, };
use iced_core::{Shadow, Theme, Vector};


#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct LogPakowanie{
    zbieranie_plików: String,
    zbieranie_plików_licznik: u8,
    pakowanie: String,
    pakowanie_licznik: u8,
    kompresja:String,
    kompresja_licznik: u8,
    szyfrowanie:String,
    szyfrowanie_licznik:u8,
    czas:String,
    błąd:String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct LogRozpakowywanie{
    kontrola_pliku: String,
    kontrola_pliku_licznik: u8,
    rozpakowanie: String,
    rozpakowanie_licznik: u8,
    dekompresja:String,
    dekompresja_licznik: u8,
    deszyfrowanie:String,
    deszyfrowanie_licznik:u8,
    czas:String,
    błąd:String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct LogPrzetwarzanieFot{
    msg_start:String,
    msg_proces:String,
    msg_end:String,
    plik_początek:String,
    plik_procent: u8,
    błąd: String,
}

// impl Default for LogPakowanie {
//     fn default() -> Self {
//         Self {
//             zbieranie_plików: String::new(),
//             zbieranie_plików_licznik: 0,
//             pakowanie: String::new(),
//             pakowanie_licznik: 0,
//             kompresja: String::new(),
//             kompresja_licznik: 0,
//             szyfrowanie: String::new(),
//             szyfrowanie_licznik: 0,
//             czas: String::new(),
//             błąd: String::new(),
//         }
//     }
// }
#[allow(dead_code)]
pub struct Program {
    aktywny_wariant: Wariant,
    // Korzystamy bezpośrednio z Twoich struktur danych
    dane_do_pakowania: DaneDoKompresji,
    dane_do_rozpakowania: DaneDoDekompresji,

    // UI state
    status_logi: Vec<String>,
    status_pakowanie_log: LogPakowanie,
    status_rozpakowywania_log: LogRozpakowywanie,
    status_zmiany_fot_log:LogPrzetwarzanieFot,
    is_loading_pakowanie: bool,
    is_loading_rozpakowywanie: bool,
    is_loading_edycja_fot: bool,
    // procent_postepu: u8,
    czy_dev_mode: bool,
    dev_tools: DevToolsMenu,
    // filtr_state: combo_box::State<FiltracjaPlików>,
    menu_edycji_zdjec_state: WybraneOknoEdycjiZdjęć,
    zdjecia_edycja_co_jest_wybrane: (bool, bool),
    zdjecia_edycja_co_jest_na_out: bool,
    dane_do_edycji_zdjec:EdycjaZdjęć,
    do_nothing: bool,
    stan_boolean_do_klikacza_zdjec: StanKlikaczaDoEdycjiZdjec,
    main_process_check:bool,
    startowy_jezyk:String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wariant {
    Pakowanie,
    Rozpakowanie,
    KonwersjaFoto,
    LaczenieFoto,
    // Ustawienia,
    // Logi,
    Dev,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StanKlikaczaDoEdycjiZdjec{
    jpg_wybrany:bool,
    jpg_progres:bool,
    jpg_kolor:bool,
    png_wybrany: bool,
    png_alpha:bool,
    alpha_kolor_16b:(u16,u16,u16),
    jpg_wybrany_rgb:bool,
    jpg_wybrany_bw:bool,
    png_kompresja:bool,
    png_wybrane_8bit:bool,
    png_wybrane_16bit:bool,
    png_wybrane_8bita:bool,
    png_wybrane_16bita:bool,
    png_wybrane_l8bit:bool,
    png_wybrane_l16bit:bool,
    png_wybrane_l8bita:bool,
    png_wybrane_l16bita:bool,
    zakladka_menu_wybrana:(bool,bool,bool, bool, bool),
    rozdzielczości_wybrane_16:bool,
    rozdzielczości_wybrane_32:bool,
    rozdzielczości_wybrane_64:bool,
    rozdzielczości_wybrane_128:bool,
    rozdzielczości_wybrane_256:bool,
    rozdzielczości_wybrane_512:bool,
    rozdzielczości_wybrane_1k:bool,
    rozdzielczości_wybrane_2k:bool,
    rozdzielczości_wybrane_4k:bool,
    rozdzielczości_wybrane_6k:bool,
    rozdzielczości_wybrane_8k:bool,
    rozdzielczości_wybrane_16k:bool,
    rozdzielczości_wybrane_org:bool,
    pub tga_wybrany: bool,
    pub tga_wybrany_32b: bool,
    pub tga_wybrany_24b: bool,
    pub tga_wybrany_16b: bool,
    pub tga_wybrany_szary: bool,
    webp_wybrany: bool,
    webp_lossless: bool,
    webp_wybrany_rgb: bool,
    webp_wybrany_alpha: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    ZmienWariant(Wariant),
    // Aktualizacja pól Eksportu
    PakowaniePathChanged(String),
    PakowanieOutPathChanged(String),
    DekompresjaPlikPathChanged(String),
    DekompresjaOutPathChanged(String),
    NazwaPaczkiChanged(String),
    ResetLogPakowanie,
    // Akcje
    UruchomProcesPakowania,
    PostepPakowania(crate::io::enums_structs_io::Progress),
    PakowanieZakonczone(Result<(), String>),
    ProcesZakonczony(Result<(), String>),
    PostepRozpakowania(crate::io::enums_structs_io::ProgressDe),
    LogDodaj(String),
    EventOccurred(Event),

    DevZmienJezyk(WybórJęzyka),
    WybierzFolderInPakowanie,
    WybierzFolderOutPakowanie,
    PoziomKompresjiChanged(String),
    PoziomInterpolacjiChanged(String),
    PoziomKompresjiSearch(String),
    FiltracjaChanged(FiltracjaPlików),
    Nic,
    None,
    FilterChanged(String),
    FilterSearch(String),
    WybierzFolderOutExPakowanie,
    WybierzPlikExPakowanie,
    UruchomProcesRozpakowania,
    ResetLogRozpakowania,
    ZmienMenuEdycjiNaSciezki(WybraneOknoEdycjiZdjęć),
    ZdjeciaZmienPlikInPathChanged(String),
    ZdjeciaZmienFolderInPathChanged(String),
    ZdjeciaZmienFolderOutPathChanged(String),
    ZdjeciaZmienFolderOutPathTenSam(bool),
    DoNothingxD(bool),
    DoNothingStringxD(String),
    WybierzPlikInFotoEdycjaPakowanie,
    WybierzFolderInFotoEdycjaPakowanie,
    WybierzFolderOutFotoEdycjaPakowanie,
    ResetujStanWejsciowychSciezekEdycjaFoto,
    ZdjeciaEdycjaZmianaJakosciJpg(u8),
    ZdjeciaEdycjaZmianaProgresJpg,
    ZdjeciaEdycjaZmianaKolorJpg(Obraz),
    ZdjeciaEdycjaZmianaWybranyJpg,
    ZdjeciaEdycjaZmianaKompresjiPng(u8),
    ZdjeciaEdycjaZmianaWybranyPng,
    ZdjeciaEdycjaZmianaFiltraPng,
    ZdjeciaEdycjaZmianaKolorPng(u8,u16),
    ZdjeciaEdycjaZmianaBitDepthPng(Obraz),
    ZdjeciaEdycjaZmianaAlphaPng,
    DopasujRozdzielczosci(Rozdzielczości),
    UsuńLogi,
    ZdjeciaEdycjaZmianaZaszumiania(u8),
    WysylkaDanychDoObrobkiZdjec,
    PostepEdycjaFot(PostepMieleniaZdjec),
    ZdjeciaEdycjaZmianaWybranyTga,
    ZdjeciaEdycjaZmianaBitDepthTga(ObrazTGA),
    ZdjeciaEdycjaZmianaLosslessWebp,
    ZdjeciaEdycjaZmianaWybranyWebp,
    ZdjeciaEdycjaZmianaJakosciWebp(u8),
    ZdjeciaEdycjaZmianaKolorWebp(Obraz),
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WybórJęzyka {
    PL,
    EN,
    DE,
    HU,
    ES,
    KR,
    JP,
    TH,
}

impl WybórJęzyka {
    pub const ALL: [WybórJęzyka; 8] = [
        Self::PL,
        Self::EN,
        Self::DE,
        Self::HU,
        Self::ES,
        Self::KR,
        Self::JP,
        Self::TH,
    ];

    pub fn z_systemu(kod: String) -> Self {
        let kod = kod.to_lowercase();

        // Mapowanie kodów ISO na Twoje warianty
        if kod.starts_with("pl") {
            Self::PL
        } else if kod.starts_with("de") {
            Self::DE
        } else if kod.starts_with("hu") {
            Self::HU
        } else if kod.starts_with("es") {
            Self::ES
        } else if kod.starts_with("kr") {
            Self::KR
        } else if kod.starts_with("jp") {
            Self::JP
        } else if kod.starts_with("th") {
            Self::TH
        } else {
            // Jeśli system to cokolwiek innego (np. fr, it, jp) -> English jako fallback
            Self::EN
        }
    }
    pub fn get_font(&self) -> iced::Font {
        match self {
            WybórJęzyka::KR => iced::Font {
                family: iced::font::Family::Name("Noto Serif KR"), // Nazwa z pliku TTF
                ..Default::default()
            },
            WybórJęzyka::JP => iced::Font {
                family: iced::font::Family::Name("Noto Serif JP"),
                ..Default::default()
            },
            WybórJęzyka::TH => iced::Font {
                family: iced::font::Family::Name("Noto Sans Thai Condensed"),
                ..Default::default()
            },
            _ => iced::Font {
                family: iced::font::Family::Name("Lato"),
                ..Default::default()
            },
        }
    }
}

impl fmt::Display for WybórJęzyka {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Używamy składni strukturalnej wewnątrz Enuma
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DevToolsMenu {
    // Wariant z polem 'jezyk'
    UstawieniaJęzyka { jezyk: WybórJęzyka },
    // Tutaj możesz dodawać kolejne warianty w ten sam sposób:
    TrybDebugowania { aktywny: bool },
}



impl Program {
    // 1. POPRAWKA: Sygnatura NEW (musi zwracać stan początkowy i Task)
    pub fn new() -> (Self, Task<Message>) {
        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

        // Mapujemy go na Twój Enum
        let startowy_jezyk = WybórJęzyka::z_systemu(locale.clone());
        (
            Self {
                aktywny_wariant: Wariant::Pakowanie,
                dane_do_pakowania: DaneDoKompresji {
                    ścieżka_in: PathBuf::new(),
                    ścieżka_out: PathBuf::new(),
                    kompresja: PoziomKompresji::Standard,
                    nazwa: String::new(),
                    foldery: StrukturaFolderów::Tak,
                    filtracja: FiltracjaPlików::Wszystkie,
                    // filtracja_opcje: vec![],
                    // filtracja_state: Default::default(),
                    // kompresja_opcje: vec![],
                    // kompresja_state: Default::default(),
                },
                dane_do_rozpakowania: DaneDoDekompresji {
                    ścieżka_pliku: PathBuf::new(),
                    ścieżka_docelowa: PathBuf::new(),
                },

                status_logi: vec![],
                status_pakowanie_log: LogPakowanie::default(),
                status_rozpakowywania_log: LogRozpakowywanie::default(),
                status_zmiany_fot_log: Default::default(),
                is_loading_pakowanie: false,
                // procent_postepu: 0.0,
                is_loading_rozpakowywanie: false,
                // procent_postepu: 0,
                is_loading_edycja_fot: false,
                czy_dev_mode: false,
                dev_tools: DevToolsMenu::UstawieniaJęzyka { jezyk: startowy_jezyk },
                // filtr_state: combo_box::State::new(FiltracjaPlików::ALL.to_vec()),
                menu_edycji_zdjec_state: WybraneOknoEdycjiZdjęć::Ścieżki,
                zdjecia_edycja_co_jest_wybrane: (false, false),
                zdjecia_edycja_co_jest_na_out: false,

                dane_do_edycji_zdjec: EdycjaZdjęć {
                    ścieżka_wejściowa: PathBuf::new(),
                    ścieżka_wyjściowa: PathBuf::new(),
                    opcje_rozdzielczości: vec![Rozdzielczości::R2k],
                    // dane_exif: DaneExif,
                    noising: None,
                    rozszerzenia: vec![
                        Rozszerzenia::Jpg {
                            jakosc: 90,
                            progresywny: false,
                            bit_depth: vec![Obraz::B8],
                        },
                        Rozszerzenia::Png {
                                kompresja: 3,
                                bit_depth: vec![Obraz::B8],
                        },
                        Rozszerzenia::Webp{ jakosc: 90, lossless: false, bit_depth: vec![Obraz::B8] },
                        Rozszerzenia::Tga { bit_depth: vec![ObrazTGA::TrueColor24] }
                    ],
                    inter: Interpolacja::Lanczos3,
                    alfa_rgb: (0, 0, 0),
                },
                do_nothing: false,
                stan_boolean_do_klikacza_zdjec: StanKlikaczaDoEdycjiZdjec {
                    jpg_wybrany: true,
                    png_wybrany: false,
                    webp_wybrany:false,
                    tga_wybrany:false,
                    jpg_progres: true,
                    jpg_kolor: true,
                    png_alpha: true,
                    alpha_kolor_16b: (0, 0, 0),
                    jpg_wybrany_rgb: true,
                    jpg_wybrany_bw: false,
                    png_kompresja: true,
                    png_wybrane_8bit: true,
                    png_wybrane_16bit: false,
                    png_wybrane_8bita: false,
                    png_wybrane_16bita: false,
                    png_wybrane_l8bit: false,
                    png_wybrane_l16bit: false,
                    png_wybrane_l8bita: false,
                    png_wybrane_l16bita: false,
                    tga_wybrany_szary:false,
                    tga_wybrany_16b:false,
                    tga_wybrany_24b:true,
                    tga_wybrany_32b:false,
                    zakladka_menu_wybrana: (true, false, false, false, false),
                    rozdzielczości_wybrane_16: false,
                    rozdzielczości_wybrane_32: false,
                    rozdzielczości_wybrane_64: false,
                    rozdzielczości_wybrane_128: false,
                    rozdzielczości_wybrane_256: false,
                    rozdzielczości_wybrane_512: false,
                    rozdzielczości_wybrane_1k: false,
                    rozdzielczości_wybrane_2k: true,
                    rozdzielczości_wybrane_4k: false,
                    rozdzielczości_wybrane_6k: false,
                    rozdzielczości_wybrane_8k: false,
                    rozdzielczości_wybrane_16k: false,
                    rozdzielczości_wybrane_org: false,
                    webp_lossless: false,
                    webp_wybrany_rgb: true,
                    webp_wybrany_alpha: false,
                },
                main_process_check: false,
                startowy_jezyk: locale.clone(),
            },
            Task::batch(vec![
                font::load(FONT_DEFAULT).map(|_| Message::None),
                font::load(FONT_KOREAN).map(|_| Message::None),
                font::load(FONT_JAPANESE).map(|_| Message::None),
                font::load(FONT_THAI).map(|_| Message::None),
            ])
        )
    }

    // 2. POPRAWKA: Sygnatura UPDATE (Iced 0.13+ oczekuje 2 argumentów: &mut self i Message)
    // Usunąłem trzeci argument, który generował błąd E0593
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let aktualny_jezyk = match self.dev_tools {
            DevToolsMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => WybórJęzyka::EN,
        };
        self.main_process_check =
            self.is_loading_pakowanie ||
            self.is_loading_rozpakowywanie||
            self.is_loading_edycja_fot;
        
        match message {
            Message::UsuńLogi => self.status_logi = Vec::new(),
            Message::DevZmienJezyk(nowy) => {
                self.dev_tools = DevToolsMenu::UstawieniaJęzyka { jezyk: nowy };
                // KLUCZOWE: Po zmianie języka odświeżamy listę w ComboBoxie
                // self.dane_do_pakowania.odswiez_tlumaczenia_kompresja(&nowy);
            }

            Message::PoziomKompresjiChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom = PoziomKompresji::WSIOKOMPRESJI.iter().find(|p| {
                    // Używamy tego samego tłumacza (jezyk), którego używamy w view
                    aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                });

                if let Some(p) = znaleziony_poziom {
                    self.dane_do_pakowania.kompresja = *p;
                    self.status_logi.push(format!("[System] Zmieniono kompresję na: {:?}", p));
                }
            }
            Message::PoziomInterpolacjiChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom = Interpolacja::WSIOINTERPOLACJI.iter().find(|p| {
                    // Używamy tego samego tłumacza (jezyk), którego używamy w view
                    aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                });

                if let Some(p) = znaleziony_poziom {
                    self.dane_do_edycji_zdjec.inter = *p;
                    self.status_logi.push(format!("[System] Zmieniono interpolację na: {:?}", p));
                }
            }
            Message::FilterChanged(wybrany_tekst) => {
                // Szukamy w ALL, który klucz po przetłumaczeniu pasuje do tego, co kliknięto
                let znaleziony_poziom = FiltracjaPlików::WSIOPLIKOW.iter().find(|p| {
                    // Używamy tego samego tłumacza (jezyk), którego używamy w view
                    aktualny_jezyk.t(p.klucz()) == wybrany_tekst
                });

                if let Some(p) = znaleziony_poziom {
                    self.dane_do_pakowania.filtracja = *p;
                    self.status_logi.push(format!("[System] Zmieniono filtrację na: {:?}", p));
                }
            }

            Message::PoziomKompresjiSearch(_) => {} // Wymagane przez ComboBox
            Message::ZmienWariant(w) => {
                self.aktywny_wariant = w ;

                match w  {
                    Wariant::Pakowanie=> self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = (true, false, false, false, false),
                    Wariant::Rozpakowanie=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = ( false,true, false, false, false),
                    Wariant::KonwersjaFoto=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = ( false, false,true, false, false),
                    Wariant::LaczenieFoto=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = ( false, false, false,true, false),
                    // Wariant::Ustawienia=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = (false, false, false, false, false),
                    // Wariant::Logi=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = (false, false, false, false, false),
                    Wariant::Dev=>self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana = (false, false, false, false, true),
                };

            },
            Message::PakowaniePathChanged(s) => self.dane_do_pakowania.ścieżka_in = PathBuf::from(s),
            Message::PakowanieOutPathChanged(s) => self.dane_do_pakowania.ścieżka_out = PathBuf::from(s),
            Message::DekompresjaPlikPathChanged(s) => self.dane_do_rozpakowania.ścieżka_pliku = PathBuf::from(s),
            Message::DekompresjaOutPathChanged(s) => self.dane_do_rozpakowania.ścieżka_docelowa = PathBuf::from(s),
            Message::DoNothingxD(xx) => self.do_nothing = xx,
            Message::DoNothingStringxD(_x) => self.do_nothing = false,
            Message::ZdjeciaZmienPlikInPathChanged(s) =>
                {
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa = PathBuf::from(s);
                    self.zdjecia_edycja_co_jest_wybrane = (false, true)
                },
            Message::ZdjeciaZmienFolderInPathChanged(s) =>
                {
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa = PathBuf::from(s);
                    self.zdjecia_edycja_co_jest_wybrane = (true, false)
                },
            Message::ZdjeciaZmienFolderOutPathTenSam(zdjecia_edycja_co_jest_na_out) => {
                self.zdjecia_edycja_co_jest_na_out = zdjecia_edycja_co_jest_na_out;
                let xoxo = if self.dane_do_edycji_zdjec.ścieżka_wejściowa.is_file(){
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| self.dane_do_edycji_zdjec.ścieżka_wejściowa.clone())
                } else{
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa.clone()
                };
                self.dane_do_edycji_zdjec.ścieżka_wyjściowa = xoxo
            },
            Message::ZdjeciaZmienFolderOutPathChanged(s) => self.dane_do_rozpakowania.ścieżka_docelowa = PathBuf::from(s),
            Message::NazwaPaczkiChanged(s) => self.dane_do_pakowania.nazwa = s,
            Message::LogDodaj(txt) => self.status_logi.push(txt),
            Message::ResetLogPakowanie => self.status_pakowanie_log = LogPakowanie::default(),
            Message::ResetLogRozpakowania => self.status_pakowanie_log = LogPakowanie::default(),
            Message::ZmienMenuEdycjiNaSciezki(nowy_stan_menu) => self.menu_edycji_zdjec_state = nowy_stan_menu,
            Message::FiltracjaChanged(f) => {
                self.dane_do_pakowania.filtracja = f;
            }
            Message::WybierzFolderInPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_do_pakowania.ścieżka_in = path;
                }
            }
            Message::ResetujStanWejsciowychSciezekEdycjaFoto => {
                self.zdjecia_edycja_co_jest_wybrane = (false, false) ;
                self.dane_do_edycji_zdjec.ścieżka_wejściowa = PathBuf::new();
            },
            Message::WybierzFolderOutPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_do_pakowania.ścieżka_out = path;
                }
            }
            Message::WybierzPlikExPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.dane_do_rozpakowania.ścieżka_pliku = path;
                }
            }
            Message::WybierzFolderOutExPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_do_rozpakowania.ścieżka_docelowa = path;
                }
            }
            Message::WybierzPlikInFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Obrazy",&["jpg", "jpeg", "webp", "png"])
                    .pick_file() {
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa = path;
                    self.zdjecia_edycja_co_jest_wybrane = (false, true)
                }

            }
            Message::WybierzFolderInFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_do_edycji_zdjec.ścieżka_wejściowa = path;
                    self.zdjecia_edycja_co_jest_wybrane = (true, false)
                }

            }
            Message::WybierzFolderOutFotoEdycjaPakowanie => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dane_do_edycji_zdjec.ścieżka_wyjściowa = path;
                }
            }
            Message::UruchomProcesPakowania => {
                self.status_pakowanie_log = LogPakowanie::default();
                self.is_loading_pakowanie = true;
                let zestaw = self.dane_do_pakowania.clone();

                let (tx, rx) = mpsc::channel::<Progress>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle.spawn(async move {
                            let _ = ogarnianie_eksportu(zestaw, tx).await;
                        }).await
                    },
                    |_| Message::Nic
                );

                let nasluchiwanie = Task::run(rx, Message::PostepPakowania);

                return Task::batch(vec![operacja, nasluchiwanie]);
            }
            Message::UruchomProcesRozpakowania => {
                self.is_loading_rozpakowywanie = true;
                let zestaw = self.dane_do_rozpakowania.clone();

                let (tx, rx) = mpsc::channel::<ProgressDe>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();

                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle.spawn(async move {
                            let _ = ogarnianie_dekompresji(zestaw, tx).await;
                        }).await
                    },
                    |_| Message::Nic
                );

                let nasluchiwanie = Task::run(rx, Message::PostepRozpakowania);

                return Task::batch(vec![operacja, nasluchiwanie]);
            }

            Message::PostepPakowania(progres) => {
                match progres {
                    Progress::ZnalezionoPliki { etap, pliki, status } => {
                        // self.status_pakowanie_log.zbieranie_plików_max = pliki;
                        // self.status_logi.push(format!("[{:?}] Znaleziono {} plików do przetworzenia.", etap, pliki));
                        if self.status_pakowanie_log.zbieranie_plików_licznik ==0 {
                            self.status_logi.push(format!("{} [Zbieranie plików] Rozpoczęte", Local::now().format("%H:%M:%S")));
                            self.status_pakowanie_log.zbieranie_plików_licznik += 1;
                        }

                        // 1. Aktualizuj tekst o plikach TYLKO jeśli przyszły (Some)
                        if let Some(p) = pliki {
                            self.status_pakowanie_log.zbieranie_plików = format!("[Zbieranie plików] Znaleziono {} {} do przetworzenia.", p, aktualny_jezyk.t(&format!("files_counting_{}",odmiana_liczbowa(p))));
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self.status_pakowanie_log.zbieranie_plików.split("   ---").next().unwrap_or("");
                        self.status_pakowanie_log.zbieranie_plików = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));
                        // self.status_pakowanie_log.zbieranie_plików = format!("[Zbieranie plików] Znaleziono {} plików do przetworzenia. --{:?}", pliki.unwrap(), status);
                    }

                    Progress::Pakowanie { etap, aktualny, suma,status } => {
                        // Obliczamy procent dla etapu pakowania
                        // self.procent_postepu = (aktualny as f32 / suma as f32) * 100.0;
                        // Logujemy tylko co jakiś czas, żeby nie zapchać UI tysiącami linii
                        if self.status_pakowanie_log.pakowanie_licznik == 0 {
                            self.status_logi.push(format!("{} [Pakowanie] Rozpoczęte", Local::now().format("%H:%M:%S")));
                            self.status_pakowanie_log.pakowanie_licznik += 1;
                        }

                            // self.status_logi.push(format!("[{:?}] Pakowanie: {}/{}", etap, aktualny, suma));
                            // 1. Aktualizujemy bazę napisu TYLKO jeśli przyszły obie liczby (Some)
                            if let (Some(akt), Some(sum)) = (aktualny, suma) {
                                self.status_pakowanie_log.pakowanie = format!("[Pakowanie] Pakowanie: {} z {} {}", akt, sum, aktualny_jezyk.t(&format!("files_counting_{}",odmiana_liczbowa(sum))));
                            }

                            // 2. Czyścimy stary status i doklejamy nowy do tego, co jest w zmiennej
                            let baza = self.status_pakowanie_log.pakowanie.split("   ---").next().unwrap_or("");
                            self.status_pakowanie_log.pakowanie = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));
                            // self.status_pakowanie_log.pakowanie = format!("[Pakowanie] Pakowanie: {}/{}   ---{:?}", aktualny.unwrap(), suma.unwrap(), status);

                    }

                    Progress::ProcesKompresji { etap, procent,status } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_pakowanie_log.kompresja_licznik == 0 {
                            self.status_logi.push(format!("{} [Kompresja] Rozpoczęta", Local::now().format("%H:%M:%S")));
                            self.status_pakowanie_log.kompresja_licznik +=1;
                        }

                            // self.status_logi.push(format!("[{:?}] Kompresja: {}%", etap, procent));
                            // self.status_logi.push(format!("{} [Kompresja] Rozpoczęta", Local::now().format("%H:%M:%S")));
                            if let Some(p) = procent{
                                self.status_pakowanie_log.kompresja = format!("[Kompresja] Kompresja {}%.", p);
                            }

                            // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                            let baza = self.status_pakowanie_log.kompresja.split("   ---").next().unwrap_or("");
                            self.status_pakowanie_log.kompresja = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));
                            // self.status_pakowanie_log.kompresja = format!("[Kompresja] Kompresja: {}%   ---{:?}", procent.unwrap(), status);
                        }


                    Progress::ProcesSzyfrowania { etap, procent, status } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_pakowanie_log.szyfrowanie_licznik ==0{

                            self.status_logi.push(format!("{} [Szyfrowanie] Rozpoczęte", Local::now().format("%H:%M:%S")));
                            self.status_pakowanie_log.szyfrowanie_licznik += 1;
                        }

                            // self.status_logi.push(format!("[{:?}] Rozpoczęto szyfrowanie XOR...", etap));
                            // self.status_logi.push(format!("{} [Szyfrowanie] Rozpoczęte", Local::now().format("%H:%M:%S")));
                            if let Some(p) = procent {
                                self.status_pakowanie_log.szyfrowanie = format!("[Szyfrowanie] Szyfrowanie {}%.", p);
                            }

                            // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                            let baza = self.status_pakowanie_log.szyfrowanie.split("   ---").next().unwrap_or("");
                            self.status_pakowanie_log.szyfrowanie = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));
                            // self.status_pakowanie_log.szyfrowanie = format!("[Szyfrowanie] Rozpoczęto szyfrowanie XOR: {}%   --{:?}", procent.unwrap(),status);
                        }


                    Progress::Zakonczono { etap, czas } => {
                        self.is_loading_pakowanie = false;
                        // self.procent_postepu = 100.0;
                        // self.status_logi.push(format!("--- PROCES ZAKOŃCZONY [{:?}] ---", etap));
                        // self.status_logi.push(format!("Łączny czas: {}", czas));

                        self.status_logi.push(format!("{} [Pakowanie] Zakończone, minęło: {}", Local::now(), czas));
                        self.status_pakowanie_log.czas = format!("Zakończono w łącznym czasie: {}",czas);
                    }

                    Progress::Błąd(err) => {
                        self.is_loading_pakowanie = false;
                        self.status_logi.push(format!("!!! {}: {} !!!", aktualny_jezyk.t("log_status_critical_error"),err));
                        self.status_pakowanie_log.błąd = format!("!!! {}: {} !!!",aktualny_jezyk.t("log_status_critical_error"), err);
                    }
                }
            }
            Message::ZdjeciaEdycjaZmianaJakosciJpg(procent) => {
                // 1. Pobieramy referencję do pierwszego elementu (mutowalną)
                if let Some(Rozszerzenia::Jpg { jakosc, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(0) {
                        *jakosc = procent;
                }
            },
            Message::ZdjeciaEdycjaZmianaZaszumiania(procent) => {
                // Jeśli suwak jest na 0, ustawiamy None (wyłączone)
                // W każdym innym przypadku (1-100) ustawiamy Some(procent)
                // let xoxo = procent.unwrap();
                self.dane_do_edycji_zdjec.noising = if procent == 0u8 {
                    None
                } else {
                    Some(procent)
                };
            }
            Message::ZdjeciaEdycjaZmianaKolorJpg(wybrany_bit_depth) => {
                match wybrany_bit_depth {
                    Obraz::L8 => {self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_bw = !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_bw}
                    Obraz::B8 => {self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_rgb = !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_rgb}
                    _ =>{}
                }
                // 1. Pobieramy referencję do pierwszego elementu (mutowalną)
                if let Some(Rozszerzenia::Jpg { bit_depth, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(0) {
                    // if bit_depth.contains(&wybrany_bit_depth) && !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_rgb{}
                    match wybrany_bit_depth {
                        Obraz::L8 => {
                            // Jeśli zawiera L8 i bool jest false -> USUŃ
                            if !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_bw {
                                bit_depth.retain(|x| x != &Obraz::L8);
                            } else if !bit_depth.contains(&Obraz::L8) {
                                bit_depth.push(Obraz::L8);
                            }
                        }
                        Obraz::B8 => {
                            // Analogiczna logika dla B8
                            if !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_rgb {
                                bit_depth.retain(|x| x != &Obraz::B8);
                            } else if !bit_depth.contains(&Obraz::B8) {
                                bit_depth.push(Obraz::B8);
                            }
                        }
                        _ => {}
                    }
                    // *bit_depth = wybrany_bit_depth;
                }
            },
            Message::ZdjeciaEdycjaZmianaWybranyJpg => {
                self.stan_boolean_do_klikacza_zdjec.jpg_wybrany = !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany;
            },
            Message::ZdjeciaEdycjaZmianaWybranyTga => {
                self.stan_boolean_do_klikacza_zdjec.tga_wybrany = !self.stan_boolean_do_klikacza_zdjec.tga_wybrany;
            },
            Message::ZdjeciaEdycjaZmianaProgresJpg => {
                if let Some(Rozszerzenia::Jpg { progresywny, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(0) {
                        *progresywny = !*progresywny;
                        self.stan_boolean_do_klikacza_zdjec.jpg_progres = *progresywny;
                }
            },

            Message::ZdjeciaEdycjaZmianaKompresjiPng(procent) => {
                if let Some(Rozszerzenia::Png { kompresja, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(1) {
                    *kompresja = procent;
                }
            },
            // Message::ZdjeciaEdycjaZmianaFiltraPng => {
            //     if let Some(Rozszerzenia::Png { filtr_alfa, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.iter_mut().find(|r| matches!(r, Rozszerzenia::Png { .. })) {
            //         *filtr_alfa = !*filtr_alfa;
            //         self.stan_boolean_do_klikacza_zdjec.png_alpha = *filtr_alfa;
            //         println!("{:?}", self.stan_boolean_do_klikacza_zdjec.png_alpha)
            //     }
            // },
            //usuwanie z vec przed wyslaniem do fn ;)
            // // Klonujemy dane, żeby oryginał w UI został nienaruszony
            // let mut dane_do_obrobki = self.dane_do_edycji_zdjec.clone();
            //
            // // Usuwamy z kopii to, czego użytkownik nie zaznaczył w toggle'ach
            // dane_do_obrobki.rozszerzenia.retain(|opt| {
            // match opt.rozszerzenie {
            // Rozszerzenia::Jpg { .. } => self.stan_boolean_do_klikacza_zdjec.jpg_wybrany,
            // Rozszerzenia::Png { .. } => self.stan_boolean_do_klikacza_zdjec.png_wybrany,
            // _ => true,
            // }
            // });
            //
            // // Teraz wysyłasz 'dane_do_obrobki' do funkcji, która zapisuje pliki
            // self.wykonaj_akcje(dane_do_obrobki);

            Message::WysylkaDanychDoObrobkiZdjec =>{
                let mut dane_do_obrobki = self.dane_do_edycji_zdjec.clone();
                self.is_loading_edycja_fot = true;
                self.status_zmiany_fot_log = Default::default();
                println!("ścieżka przekazywana to: {:?}", self.dane_do_edycji_zdjec.ścieżka_wejściowa);

                dane_do_obrobki.rozszerzenia.retain(|opt| {
                match opt {
                Rozszerzenia::Jpg { .. } => self.stan_boolean_do_klikacza_zdjec.jpg_wybrany,
                Rozszerzenia::Png { .. } => self.stan_boolean_do_klikacza_zdjec.png_wybrany,
                Rozszerzenia::Webp { .. } => self.stan_boolean_do_klikacza_zdjec.webp_wybrany,
                Rozszerzenia::Tga { .. } => self.stan_boolean_do_klikacza_zdjec.tga_wybrany,
                }
                });

                let (tx, rx) = mpsc::channel::<PostepMieleniaZdjec>(100);

                // Pobieramy uchwyt do działającego runtime'u Tokio
                let handle = tokio::runtime::Handle::current();
                let operacja = Task::perform(
                    async move {
                        // Zmuszamy funkcję do wejścia w kontekst pobranego uchwytu
                        handle.spawn(async move {
                            let _ = ogarnianie_foto(dane_do_obrobki, tx).await;
                        }).await
                    },
                    |_| Message::Nic
                );

                let nasluchiwanie = Task::run(rx, Message::PostepEdycjaFot);

                return Task::batch(vec![operacja, nasluchiwanie]);


            }
            Message::PostepEdycjaFot(progress) => {
                match progress{
                    PostepMieleniaZdjec::Start => {
                        self.status_zmiany_fot_log.msg_start = "Rozpoczęto".to_string();
                    },
                    PostepMieleniaZdjec::Rozpoczęto(wczytano,procent) => {
                        // println!("Update dostał procent: {}", procent); // <-- DEBUG
                        self.status_zmiany_fot_log.plik_procent = procent;
                        self.status_zmiany_fot_log.msg_proces = format!("{}",procent);
                    },
                    PostepMieleniaZdjec::FiltrowaniePlików(xoxo )=>{
                        if xoxo > 0{self.status_zmiany_fot_log.plik_początek = format!("Zebrano {} plików",xoxo);}

                    },
                    PostepMieleniaZdjec::PominiętePliki {sciezka, powod} =>{
                        self.status_logi.push(format!("[Obrazy] Pominięto plik z:\n{}\n z powodu: {}",sciezka,powod));
                    },
                    PostepMieleniaZdjec::Koniec(czas) => {
                        self.status_zmiany_fot_log.msg_end = format!("Zakończono w czasie: {}",czas);
                        self.is_loading_edycja_fot = false;
                    },
                    PostepMieleniaZdjec::Błąd(err) => {
                        self.status_zmiany_fot_log.błąd = format!("Błąd: {}",err);
                        self.is_loading_edycja_fot = false;
                    },
                }
            },
            Message::ZdjeciaEdycjaZmianaWybranyPng => {
                self.stan_boolean_do_klikacza_zdjec.png_wybrany = !self.stan_boolean_do_klikacza_zdjec.png_wybrany;
            },
            // Message::ZdjeciaEdycjaZmianaAlphaPng => {
            //     self.stan_boolean_do_klikacza_zdjec.png_alpha = !self.stan_boolean_do_klikacza_zdjec.png_alpha
            // },
            Message::ZdjeciaEdycjaZmianaKolorPng(indeks, wartosc) => {
                
                        match indeks {
                            0 => self.dane_do_edycji_zdjec.alfa_rgb.0 = wartosc, // Zmieniamy R
                            1 => self.dane_do_edycji_zdjec.alfa_rgb.1 = wartosc, // Zmieniamy G
                            2 => self.dane_do_edycji_zdjec.alfa_rgb.2 = wartosc, // Zmieniamy B
                            _ => {}
                        }
                        // Aktualizacja Twojego pomocniczego stanu boolean/kolor
                        self.stan_boolean_do_klikacza_zdjec.alpha_kolor_16b = self.dane_do_edycji_zdjec.alfa_rgb;
                    
            },
            Message::ZdjeciaEdycjaZmianaBitDepthPng(wybrany_bit_depth) => {
                // Zakładamy, że PNG jest pod indeksem 1 w wektorze
                match wybrany_bit_depth {
                    Obraz::L8 => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bit = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bit;
                    }
                    Obraz::L8a => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bita = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bita;
                    }
                    Obraz::B8 => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bit = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bit;
                    }
                    Obraz::B8a => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bita = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bita;
                    }
                    Obraz::L16 => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bit = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bit;
                    }
                    Obraz::L16a => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bita = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bita;
                    }
                    Obraz::B16 => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bit = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bit;
                    }
                    Obraz::B16a => {
                        self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bita = !self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bita;
                    }
                    Obraz::B32 | Obraz::B32a => {
                        // Tutaj pusto, zgodnie z Twoim wcześniejszym założeniem dla 32-bit
                    }
                }
                if let Some(Rozszerzenia::Png { bit_depth, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(1) {
                    // *bit_depth = wybrany_bit_depth;

                    match wybrany_bit_depth {
                        // Obraz::B8=> {
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bit = true;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bit = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bita = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bita = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bit = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bit = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bita = false;
                        //     self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bita = false;
                        // },
                        Obraz::B8 => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bit {
                                bit_depth.retain(|x| x != &Obraz::B8);
                            } else if !bit_depth.contains(&Obraz::B8) {
                                bit_depth.push(Obraz::B8);
                            }
                        },
                        Obraz::B16 => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bit {
                                bit_depth.retain(|x| x != &Obraz::B16);
                            } else if !bit_depth.contains(&Obraz::B16) {
                                bit_depth.push(Obraz::B16);
                            }
                        },
                        Obraz::B8a => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_8bita {
                                bit_depth.retain(|x| x != &Obraz::B8a);
                            } else if !bit_depth.contains(&Obraz::B8a) {
                                bit_depth.push(Obraz::B8a);
                            }
                        },
                        Obraz::B16a => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_16bita {
                                bit_depth.retain(|x| x != &Obraz::B16a);
                            } else if !bit_depth.contains(&Obraz::B16a) {
                                bit_depth.push(Obraz::B16a);
                            }
                        },
                        Obraz::L8 => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bit {
                                bit_depth.retain(|x| x != &Obraz::L8);
                            } else if !bit_depth.contains(&Obraz::L8) {
                                bit_depth.push(Obraz::L8);
                            }
                        },
                        Obraz::L8a => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l8bita {
                                bit_depth.retain(|x| x != &Obraz::L8a);
                            } else if !bit_depth.contains(&Obraz::L8a) {
                                bit_depth.push(Obraz::L8a);
                            }
                        },
                        Obraz::L16 => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bit {
                                bit_depth.retain(|x| x != &Obraz::L16);
                            } else if !bit_depth.contains(&Obraz::L16) {
                                bit_depth.push(Obraz::L16);
                            }
                        },
                        Obraz::L16a => {
                            if !self.stan_boolean_do_klikacza_zdjec.png_wybrane_l16bita {
                                bit_depth.retain(|x| x != &Obraz::L16a);
                            } else if !bit_depth.contains(&Obraz::L16a) {
                                bit_depth.push(Obraz::L16a);
                            }
                        },
                        _ => {}
                    }
                }
            },
            Message::ZdjeciaEdycjaZmianaWybranyWebp => {
                self.stan_boolean_do_klikacza_zdjec.webp_wybrany = !self.stan_boolean_do_klikacza_zdjec.webp_wybrany;
            },
            Message::ZdjeciaEdycjaZmianaLosslessWebp => {
                if let Some(Rozszerzenia::Webp { lossless, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(2) {
                    *lossless = !*lossless;
                    self.stan_boolean_do_klikacza_zdjec.webp_lossless = *lossless;
                }
            },
            Message::ZdjeciaEdycjaZmianaJakosciWebp(procent) => {
                // 1. Pobieramy referencję do pierwszego elementu (mutowalną)
                if let Some(Rozszerzenia::Webp { jakosc, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(2) {
                    *jakosc = procent;
                }
            },
            Message::ZdjeciaEdycjaZmianaKolorWebp(wybrany_bit_depth) => {
                match wybrany_bit_depth {
                    Obraz::B8a => {self.stan_boolean_do_klikacza_zdjec.webp_wybrany_alpha = !self.stan_boolean_do_klikacza_zdjec.webp_wybrany_alpha}
                    Obraz::B8 => {self.stan_boolean_do_klikacza_zdjec.webp_wybrany_rgb = !self.stan_boolean_do_klikacza_zdjec.webp_wybrany_rgb}
                    _ =>{}
                }
                // 1. Pobieramy referencję do pierwszego elementu (mutowalną)
                if let Some(Rozszerzenia::Webp { bit_depth, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(2) {
                    // if bit_depth.contains(&wybrany_bit_depth) && !self.stan_boolean_do_klikacza_zdjec.jpg_wybrany_rgb{}
                    match wybrany_bit_depth {
                        Obraz::B8 => {
                            // Jeśli zawiera L8 i bool jest false -> USUŃ
                            if !self.stan_boolean_do_klikacza_zdjec.webp_wybrany_rgb {
                                bit_depth.retain(|x| x != &Obraz::B8);
                            } else if !bit_depth.contains(&Obraz::B8) {
                                bit_depth.push(Obraz::B8);
                            }
                        }
                        Obraz::B8a => {
                            // Analogiczna logika dla B8
                            if !self.stan_boolean_do_klikacza_zdjec.webp_wybrany_alpha {
                                bit_depth.retain(|x| x != &Obraz::B8a);
                            } else if !bit_depth.contains(&Obraz::B8a) {
                                bit_depth.push(Obraz::B8a);
                            }
                        }
                        _ => {}
                    }
                    // *bit_depth = wybrany_bit_depth;
                }
            },
            Message::ZdjeciaEdycjaZmianaBitDepthTga(wybrany_bit_depth) => {
                // Zakładamy, że PNG jest pod indeksem 1 w wektorze
                match wybrany_bit_depth {
                    ObrazTGA::Szary8 => {
                        self.stan_boolean_do_klikacza_zdjec.tga_wybrany_szary = !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_szary;
                    }

                    ObrazTGA::HighColor16 => {
                        self.stan_boolean_do_klikacza_zdjec.tga_wybrany_16b = !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_16b;
                    }
                    ObrazTGA::TrueColor24 => {
                        self.stan_boolean_do_klikacza_zdjec.tga_wybrany_24b = !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_24b;
                    }
                    ObrazTGA::TrueColorA32 => {
                        self.stan_boolean_do_klikacza_zdjec.tga_wybrany_32b = !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_32b;
                    }
                }
                if let Some(Rozszerzenia::Tga { bit_depth, .. }) = self.dane_do_edycji_zdjec.rozszerzenia.get_mut(3) {
                    // *bit_depth = wybrany_bit_depth;

                    match wybrany_bit_depth {
                        ObrazTGA::Szary8 => {
                            if !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_szary {
                                bit_depth.retain(|x| x != &ObrazTGA::Szary8);
                            } else if !bit_depth.contains(&ObrazTGA::Szary8) {
                                bit_depth.push(ObrazTGA::Szary8);
                            }
                        },
                        ObrazTGA::HighColor16 => {
                            if !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_16b {
                                bit_depth.retain(|x| x != &ObrazTGA::HighColor16);
                            } else if !bit_depth.contains(&ObrazTGA::HighColor16) {
                                bit_depth.push(ObrazTGA::HighColor16);
                            }
                        },
                        ObrazTGA::TrueColor24 => {
                            if !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_24b {
                                bit_depth.retain(|x| x != &ObrazTGA::TrueColor24);
                            } else if !bit_depth.contains(&ObrazTGA::TrueColor24) {
                                bit_depth.push(ObrazTGA::TrueColor24);
                            }
                        },
                        ObrazTGA::TrueColorA32 => {
                            if !self.stan_boolean_do_klikacza_zdjec.tga_wybrany_32b {
                                bit_depth.retain(|x| x != &ObrazTGA::TrueColorA32);
                            } else if !bit_depth.contains(&ObrazTGA::TrueColorA32) {
                                bit_depth.push(ObrazTGA::TrueColorA32);
                            }
                        },
                    }
                }
            },
            Message::PostepRozpakowania(progres) => {
                match progres {

                    ProgressDe::ZnalezionoPlikDe{etap, procent, status} => {

                        if self.status_rozpakowywania_log.kontrola_pliku_licznik ==0 {
                            self.status_logi.push(format!("{} [Analiza] Rozpoczęte", Local::now().format("%H:%M:%S")));
                            self.status_rozpakowywania_log.kontrola_pliku_licznik += 1;
                        }

                        if let Some(p) = procent {
                            self.status_rozpakowywania_log.kontrola_pliku = format!("[Analiza] Przetworzono{}%", p);
                        }

                        let baza = self.status_rozpakowywania_log.kontrola_pliku.split("   ---").next().unwrap_or("");
                        self.status_rozpakowywania_log.kontrola_pliku = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));

                    }

                    ProgressDe::Deszyfracja { etap, procent, status } => {
                        // self.procent_postepu = procent as f32;
                        if self.status_rozpakowywania_log.deszyfrowanie_licznik ==0{

                            self.status_logi.push(format!("{} [Deszyfracja] Rozpoczęta", Local::now().format("%H:%M:%S")));
                            self.status_rozpakowywania_log.deszyfrowanie_licznik += 1;
                        }

                        if let Some(p) = procent {
                            self.status_rozpakowywania_log.deszyfrowanie = format!("[Deszyfrowanie] postęp {}%.", p);
                        }

                        // 2. Pobierz bazę (z poprzednią liczbą) i doklej nowy status
                        let baza = self.status_rozpakowywania_log.deszyfrowanie.split("   ---").next().unwrap_or("");
                        self.status_rozpakowywania_log.deszyfrowanie = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));

                    }

                    ProgressDe::Dekompresja { etap, pamięć,status, .. } => {
                        if self.status_rozpakowywania_log.dekompresja_licznik == 0 {
                            self.status_logi.push(format!("{} [{}] {}", Local::now().format("%H:%M:%S"),aktualny_jezyk.t("log_status_operation_decompression"),aktualny_jezyk.t("log_status_started")));
                            self.status_rozpakowywania_log.dekompresja_licznik +=1;
                        }

                        if let Some(p) = pamięć{
                            self.status_rozpakowywania_log.dekompresja  = format!("[{}] Zdekompresowano {}.",aktualny_jezyk.t("log_status_operation_decompression"), zmieniacz_ilosci_bajtow(p), );
                        }

                        let baza = self.status_rozpakowywania_log.dekompresja.split("   ---").next().unwrap_or("");
                        self.status_rozpakowywania_log.dekompresja = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));

                    }


                    ProgressDe::RozpakowywanieDe { etap, aktualny, suma,status } => {
                        if self.status_rozpakowywania_log.rozpakowanie_licznik == 0 {
                            self.status_logi.push(format!("{} [{}] Rozpoczęte", Local::now().format("%H:%M:%S"),aktualny_jezyk.t("log_status_operation_unpacking")));
                            self.status_rozpakowywania_log.rozpakowanie_licznik += 1;
                        }


                            if let (Some(akt), Some(sum)) = (aktualny, suma) {
                                self.status_rozpakowywania_log.rozpakowanie = format!("[{}] postęp: {} z {} {}",aktualny_jezyk.t("log_status_operation_unpacking"), akt, sum, aktualny_jezyk.t("files_counting_numbers_3"));
                            }

                            let baza = self.status_rozpakowywania_log.rozpakowanie.split("   ---").next().unwrap_or("");
                            self.status_rozpakowywania_log.rozpakowanie = format!("{}   ---{}", baza, aktualny_jezyk.t(status_to_string(status)));
                        }






                    ProgressDe::ZakonczonoDe { etap, czas } => {
                        self.is_loading_rozpakowywanie = false;
                        // self.procent_postepu = 100.0;
                        // self.status_logi.push(format!("--- PROCES ZAKOŃCZONY [{:?}] ---", etap));
                        // self.status_logi.push(format!("Łączny czas: {}", czas));

                        self.status_logi.push(format!("{} [{}] {}, {}: {}", Local::now().format("%H:%M:%S"),aktualny_jezyk.t("log_status_operation_unpacking"),aktualny_jezyk.t("log_status_finished"), aktualny_jezyk.t("log_status_in_time").to_lowercase(), czas));
                        self.status_rozpakowywania_log.czas = format!("Zakończono w łącznym czasie: {}",czas);
                    }

                    ProgressDe::BłądDe(err) => {
                        self.is_loading_rozpakowywanie = false;
                        self.status_logi.push(format!("!!! {}: {} !!!",aktualny_jezyk.t("log_status_critical_error"), err));
                        self.status_rozpakowywania_log.błąd= format!("!!! {}: {} !!!",aktualny_jezyk.t("log_status_critical_error"), err);
                    }
                }
            }
            Message::DopasujRozdzielczosci(rozdzielczosc) => {
                // 1. Wyciągamy referencję do booleana w stanie, który odpowiada temu enumowi
                // Musimy wiedzieć, czy przycisk właśnie włączamy, czy wyłączamy.
                let stan_bool = match rozdzielczosc {
                    Rozdzielczości::R16 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_16,
                    Rozdzielczości::R32 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_32,
                    Rozdzielczości::R64 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_64,
                    Rozdzielczości::R128 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_128,
                    Rozdzielczości::R256 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_256,
                    Rozdzielczości::R512 => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_512,
                    Rozdzielczości::R1k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_1k,
                    Rozdzielczości::R2k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_2k,
                    Rozdzielczości::R4k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_4k,
                    Rozdzielczości::R6k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_6k,
                    Rozdzielczości::R8k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_8k,
                    Rozdzielczości::R16k => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_16k,
                    Rozdzielczości::Oryginalna => &mut self.stan_boolean_do_klikacza_zdjec.rozdzielczości_wybrane_org,
                };

                // 2. Odwracamy stan (kliknięcie przełącza on/off)
                *stan_bool = !*stan_bool;

                // 3. Aktualizujemy wektor z opcjami (synchronizacja)
                if *stan_bool {
                    // Jeśli włączyliśmy przycisk -> dodaj do listy, jeśli jeszcze go tam nie ma
                    if !self.dane_do_edycji_zdjec.opcje_rozdzielczości.contains(&rozdzielczosc) {
                        self.dane_do_edycji_zdjec.opcje_rozdzielczości.push(rozdzielczosc);
                    }
                } else {
                    // Jeśli wyłączyliśmy przycisk -> usuń z listy
                    self.dane_do_edycji_zdjec.opcje_rozdzielczości.retain(|x| x != &rozdzielczosc);
                }
            }

            Message::EventOccurred(Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. })) => {
                // match event {
                //     // ... Twoja obsługa FileDropped ...
                //
                //     Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                //         // 1. Obsługa Ctrl + E (Przełączanie Dev Mode)
                //         if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                //             self.czy_dev_mode = !self.czy_dev_mode;
                //             // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                //             self.status_logi.push(format!("Dev Mode: {}", self.czy_dev_mode));
                //         }
                //     }
                //     _ => {}
                // }

                         // 1. Obsługa Ctrl + E (Przełączanie Dev Mode)
                         if modifiers.control() && key == iced::keyboard::Key::Character("e".into()) {
                                self.czy_dev_mode = !self.czy_dev_mode;
                                 // Opcjonalnie dodaj log, żebyś wiedział, że zadziałało
                                self.status_logi.push(format!("Dev Mode: {}", self.czy_dev_mode));
                            }


            }

            _ => {}
        }
        Task::none()
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            // 1. Nasłuchiwanie na zmianę rozmiaru okna


            // 2. Nasłuchiwanie na ogólne zdarzenia (klawiatura, mysz itp.)
            iced::event::listen().map(Message::EventOccurred),
        ])
    }

    // 3. POPRAWKA: Sygnatura VIEW (Iced 0.13+ oczekuje TYLKO &self)
    // Błąd E0593 brał się stąd, że w poprzednim przykładzie sygnatura mogła sugerować trait-object lub stare API
    pub fn view(&self) -> Element<'_, Message> {
        let aktualny_jezyk = match self.dev_tools {
            DevToolsMenu::UstawieniaJęzyka { jezyk } => jezyk,
            // Fallback jeśli dodasz inne warianty dev menu
            _ => WybórJęzyka::EN,
        };

        // --- LEWA STRONA ---
        let mut przyciski_menu = container(row![
            button(text(aktualny_jezyk.t("main_toggle_export")).width(Length::Fill).height(Length::Fill).center()).on_press(Message::ZmienWariant(Wariant::Pakowanie)).height(Length::Fill).width(Length::FillPortion(5))
                                .style(styl_przycisków!(self.is_loading_pakowanie,self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana.0,KOLORBRILIANTCRIMSON)),
            button(text(aktualny_jezyk.t("main_toggle_import")).width(Length::Fill).height(Length::Fill).center()).on_press(Message::ZmienWariant(Wariant::Rozpakowanie)).height(Length::Fill).width(Length::FillPortion(5))
                                .style(styl_przycisków!(self.is_loading_rozpakowywanie,self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana.1,KOLORFLIRT)),
            button(text(aktualny_jezyk.t("main_toggle_photo_compil")).width(Length::Fill).height(Length::Fill).center()).on_press(Message::ZmienWariant(Wariant::KonwersjaFoto)).height(Length::Fill).width(Length::FillPortion(5))
                                .style(styl_przycisków!(self.is_loading_edycja_fot,self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana.2,KOLORSPANISHORANGE)),
            button(text(aktualny_jezyk.t("main_toggle_photo_merge")).width(Length::Fill).height(Length::Fill).center()).on_press(Message::ZmienWariant(Wariant::LaczenieFoto)).height(Length::Fill).width(Length::FillPortion(5))
                                .style(styl_przycisków!(self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana.3,false,KOLORPEACHPUFF)),
            button(text("Dev").width(Length::Fill).height(Length::Fill).center()).on_press(Message::ZmienWariant(Wariant::Dev)).height(Length::Fill).width(Length::FillPortion(5))
                                .style(styl_przycisków!(self.stan_boolean_do_klikacza_zdjec.zakladka_menu_wybrana.4,false,KOLORCRIMSONGLORY)),

    ]
            .spacing(10))
            .width(Length::Fill)
            .height(Length::Fixed(50.))
            .style(move |_theme| {
                iced::widget::container::Style {
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
            })
            ;


        // WYWOŁANIE WYDZIELONYCH MODUŁÓW
        let content_lewy = match self.aktywny_wariant {
            Wariant::Pakowanie => ui_pakowanie::view_eksport(&self.dane_do_pakowania, aktualny_jezyk, self.status_pakowanie_log.clone(), self.is_loading_pakowanie,self.main_process_check),
            Wariant::Rozpakowanie => ui_rozpakowanie::view_import(&self.dane_do_rozpakowania, aktualny_jezyk, self.status_rozpakowywania_log.clone(), self.is_loading_rozpakowywanie,self.main_process_check),
            Wariant::KonwersjaFoto => ui_zdjecia_edycja::view_foto_change(self.dane_do_edycji_zdjec.clone(), &self.menu_edycji_zdjec_state,aktualny_jezyk,self.zdjecia_edycja_co_jest_wybrane, self.zdjecia_edycja_co_jest_na_out, self.stan_boolean_do_klikacza_zdjec.clone(), self.is_loading_edycja_fot,self.status_zmiany_fot_log.clone(),self.main_process_check),
            Wariant::Dev => dev::dev_tools(&self.dev_tools),
            _ => column![text(aktualny_jezyk.t("Opcja jeszcze niedostępna")).size(20)].into(),
        };

        let lewa_kolumna = container(
            column![przyciski_menu, Space::new().height(30), content_lewy].padding(20)
        )
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(
                move |theme: &Theme| {
                    iced::widget::container::Style {
                        // Tło: r: 0.1, g: 0.15, b: 0.2, alpha: 1.0 (zakładam pełne krycie)
                        background: Some(Color::from_rgb(0.11, 0.11, 0.10).into()),

                        // Cień o tym samym kolorze
                        shadow: Shadow {
                            color: Color::from_rgb(0.1, 0.15, 0.2),
                            offset: Vector::new(0.0, 0.0), // Przesunięcie cienia w dół
                            blur_radius: 0.0,             // Rozmycie cienia
                        },

                        border: Border {
                            // radius: 8.0.into(), // Zaokrąglone rogi, żeby cień ładniej wyglądał
                            ..Border::default()
                        },
                        ..iced::widget::container::Style::default()
                    }
                }
            );
        // self.status_logi.push(format!("{}!!!!!\n {}: {}\n  {}: {}\n   {}, \n    {}: {}\n---------------------------------------",aktualny_jezyk.t("log_status_welcome_msg_welcome"),aktualny_jezyk.t("log_status_welcome_msg_today"),Local::now().format("%d.%m.%Y"),aktualny_jezyk.t("log_status_welcome_msg_today"), Local::now().format("%H:%M:%S"),aktualny_jezyk.t("log_status_welcome_msg_sys_rdy"),aktualny_jezyk.t("log_status_welcome_msg_lang_detected"),self.startowy_jezyk));

        // --- PRAWA STRONA (Logi zostawiamy tutaj, bo są proste) ---
        let logi_column = column(

            self.status_logi.iter()
                .map(|l| text(l).size(19).color(Color::from_rgba(1.,1.,1.,0.7)).font( iced::Font {family: iced::font::Family::Name("VT323"),
                     ..Default::default()}).into())
                .collect::<Vec<Element<'_, Message>>>()
        ).spacing(5);

        let prawa_kolumna = container(
            Column::new()
                .push(Row::new()
                    .push(text(aktualny_jezyk.t("console_menu_status")).size(19).color(Color::from_rgba(1.,1.,1.,0.8)).font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}),)
                    .push(button(text(aktualny_jezyk.t("console_menu_reset")).size(19).width(Length::Fill).height(Length::Fill).font( iced::Font {family: iced::font::Family::Name("VT323"),
                             ..Default::default()}).center()).height(Length::Fixed(25.)).on_press(Message::UsuńLogi).style(styl_przycisków!(false,false,(0.1, 0.11, 0.13)))
                    ).height(Length::Fixed(30.)),)
                .push(text("---------------------------------------").size(19).color(Color::from_rgba(1.,1.,1.,0.5)).width(Length::Fill).height(Length::Fill).font( iced::Font {family: iced::font::Family::Name("VT323"),
                         ..Default::default()}).center().height(Length::Fixed(25.)),)
                .push(Space::new().height(25),)
                .push(scrollable(logi_column),)
        .padding(15)
        )
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .style(
                move |theme: &Theme| {
                    iced::widget::container::Style {
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
                        ..iced::widget::container::Style::default()
                    }
                }
            );

        row![lewa_kolumna, prawa_kolumna].into()
    }
}