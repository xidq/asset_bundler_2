use std::path::PathBuf;
use iced::widget::combo_box;
use crate::foty::zmiana_fot::{EdycjaZdjęć, Interpolacja};
use crate::ui::program::WybórJęzyka;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum KolejnośćDziałańDe{
    SprawdzaniePliku,
    Deszyfrowanie,
    Dekompresja,
    Rozpakowywanie,
    Qniec,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum ProgressDe {
    ZnalezionoPlikDe { etap: KolejnośćDziałańDe, procent:Option<u8>, status: ProcesStatus},
    Deszyfracja { etap: KolejnośćDziałańDe, procent: Option<u8>, status: ProcesStatus },
    Dekompresja {etap:KolejnośćDziałańDe, pamięć: Option<u64>, status: ProcesStatus },
    RozpakowywanieDe { etap: KolejnośćDziałańDe, aktualny: Option<i32>, suma: Option<i32>, status: ProcesStatus },
    BłądDe(String),
    ZakonczonoDe{etap: KolejnośćDziałańDe, czas:String},
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct DaneDoDekompresji {
    pub(crate) ścieżka_pliku: PathBuf,
    pub(crate) ścieżka_docelowa: PathBuf,
}
#[allow(dead_code)]
#[derive( Clone)]
pub struct DaneDoKompresji{
    pub ścieżka_in:PathBuf,
    pub ścieżka_out:PathBuf,
    pub kompresja:PoziomKompresji,
    pub nazwa:String,
    pub foldery:StrukturaFolderów,
    pub filtracja: FiltracjaPlików,
    // pub filtracja_opcje: Vec<String>,
    // pub filtracja_state: combo_box::State<String>,
    // pub kompresja_opcje: Vec<String>,
    // pub kompresja_state: combo_box::State<String>,
}
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) enum StrukturaFolderów{
    Tak,
    Nie
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProcesStatus{
    Rozpoczęte,
    Wtrakcie,
    IO,
    Zakończone,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum KolejnośćDziałań{
    ZbieraniePlików,
    Binarka,
    Kompresja,
    Szyfrowanie,
    Qniec,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum FiltracjaPlików{
    Wszystkie,
    Graficzne,
    Audio,
    Tekstowe,
    Pdf
}
#[allow(dead_code)]
#[derive( Clone, Debug, PartialEq, Copy)]
pub enum PoziomKompresji{
    Brak,
    Standard,
    Duża,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum Progress {
    ZnalezionoPliki { etap: KolejnośćDziałań, pliki:Option<i32>, status:ProcesStatus},
    Pakowanie { etap: KolejnośćDziałań, aktualny: Option<i32>, suma: Option<i32>, status:ProcesStatus },
    ProcesKompresji { etap: KolejnośćDziałań, procent: Option<u8>, status:ProcesStatus },
    ProcesSzyfrowania { etap: KolejnośćDziałań, procent: Option<u8>, status:ProcesStatus },
    Błąd(String),
    Zakonczono{etap:KolejnośćDziałań, czas:String},
}

impl PoziomKompresji {
    pub const WSIOKOMPRESJI: [Self; 3] = [Self::Brak, Self::Standard, Self::Duża];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Brak => "comp_none",
            Self::Standard => "comp_std",
            Self::Duża => "comp_max",
        }
    }
}

impl Interpolacja {
    pub const WSIOINTERPOLACJI: [Self; 5] = [Self::Nearest, Self::Triangle, Self::CatmullRom, Self::Gaussian, Self::Lanczos3];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Nearest => "interpolacja_nearest",
            Self::Triangle => "interpolacja_triangle",
            Self::CatmullRom => "interpolacja_catmull",
            Self::Gaussian => "interpolacja_gaussian",
            Self::Lanczos3 => "interpolacja_lanczos",
        }
    }
}
impl FiltracjaPlików {
    pub const WSIOPLIKOW: [Self; 5] = [Self::Wszystkie, Self::Graficzne, Self::Audio, Self::Tekstowe, Self::Pdf];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Wszystkie => "filter_all",
            Self::Graficzne => "filter_graphic",
            Self::Audio => "filter_audio",
            Self::Tekstowe => "filter_text",
            Self::Pdf => "filter_pdf",
        }
    }
}

// impl EdycjaZdjęć{
//     pub fn odswiez_tlumaczenia_interpolacji(&mut self, jezyk: &WybórJęzyka) {
//
//         self.
//     }
// }

// impl DaneDoKompresji {
//     pub fn odswiez_tlumaczenia_kompresja(&mut self, jezyk: &WybórJęzyka) {
//         // Generujemy nowe stringi na podstawie aktualnego języka
//         self.kompresja_opcje = PoziomKompresji::WSIOKOMPRESJI.iter()
//             .map(|p| jezyk.t(p.klucz()).to_string())
//             .collect();
//
//         // Aktualizujemy listę wewnątrz stanu ComboBoxa
//         self.kompresja_state = combo_box::State::new(self.kompresja_opcje.clone());
//     }
//     pub fn odswiez_tlumaczenia_filtry(&mut self, jezyk: &WybórJęzyka) {
//         // Generujemy nowe stringi na podstawie aktualnego języka
//         self.kompresja_opcje = FiltracjaPlików::WSIOPLIKOW.iter()
//             .map(|p| jezyk.t(p.klucz()).to_string())
//             .collect();
//
//         // Aktualizujemy listę wewnątrz stanu ComboBoxa
//         self.kompresja_state = combo_box::State::new(self.kompresja_opcje.clone());
//     }
// }