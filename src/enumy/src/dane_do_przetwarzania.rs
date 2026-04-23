use crate::opcje::{OptFormatDds, OptInterpolacja, OptKompresjaDds, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Clone)]
pub struct DaneDoKompresjaPlików {
    pub ścieżka_in: PathBuf,
    pub ścieżka_out: PathBuf,
    pub kompresja: OptKompresjaPlikówPoziomKompresjiZstd,
    pub nazwa: String,
    pub foldery: bool,
    pub filtracja: OptKompresjaPlikówFiltracjaPlików,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DaneDoDekompresjaPlików {
    pub ścieżka_pliku: PathBuf,
    pub ścieżka_docelowa: PathBuf,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct DaneDoBathKonwersjaZdjec {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub opcje_rozdzielczości: Vec<OptRozdzielczościObrazów>,
    // dane_exif: DaneExif,
    pub noising: Option<u8>,
    pub rozszerzenia_plików_zdjęciowych: Vec<OptRozszerzeniaPlikówZdjęciowych>,
    pub inter: OptInterpolacja,
    pub alfa_rgb: (u16, u16, u16),
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DaneDoŁączeniaZdjęć {
    pub sciezka_r: Option<PathBuf>,
    pub sciezka_g: Option<PathBuf>,
    pub sciezka_b: Option<PathBuf>,
    pub sciezka_a: Option<PathBuf>,
    pub sciezka_out: PathBuf,
    pub out_format: OptRozszerzeniaPlikówZdjęciowych,
    pub nazwa: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DaneDoPakowaniaDds {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub format: OptFormatDds,
    pub kompresja: OptKompresjaDds,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DaneDoRozpakowaniaDds {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub rozszerzenie: OptRozszerzeniaPlikówZdjęciowych,
}
