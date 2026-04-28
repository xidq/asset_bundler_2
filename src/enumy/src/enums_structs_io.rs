use crate::wybranie_jezykowe;
use iced::widget::combo_box;
use std::fmt;
use std::path::PathBuf;

pub const FILTERFOTO: [&str; 16] = [
    "jpg", "jpeg", "webp", "png", "tga", "bmp", "ff", "exr", "ico", "hdr", "pnm", "qoi", "tiff",
    "ff.zst", "ff.bz2", "ff.xz",
];

// #[derive(Debug, Clone)]
// pub struct OptRozszerzeniaPlikówZdjęciowychIopcje{
//     pub(crate) rozszerzenie: rozszerzenia_plików_zdjęciowych,
//     pub(crate) bit_depth: Vec<Obraz>,
// }

// #[derive(Clone)]
// pub enum CoRobimyZExif{
//     Usuwamy,
//     Zostawiamy,
//     Zmieniamy
// }

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPakowanie {
    pub zbieranie_plików: String,
    pub zbieranie_plików_licznik: u8,
    pub pakowanie: String,
    pub pakowanie_licznik: u8,
    pub kompresja: String,
    pub kompresja_licznik: u8,
    pub szyfrowanie: String,
    pub szyfrowanie_licznik: u8,
    pub czas: String,
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogRozpakowywanie {
    pub kontrola_pliku: String,
    pub kontrola_pliku_licznik: u8,
    pub rozpakowanie: String,
    pub rozpakowanie_licznik: u8,
    pub StatusDekompresjaPlikówDekompresja: String,
    pub StatusDekompresjaPlikówDekompresja_licznik: u8,
    pub deszyfrowanie: String,
    pub deszyfrowanie_licznik: u8,
    pub czas: String,
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPrzetwarzanieFot {
    pub msg_start: String,
    pub msg_walidacja: String,
    pub msg_proces: String,
    pub msg_end: String,
    pub plik_początek: String,
    pub plik_procent: u8,
    pub błąd: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogPakowaniaDds{
    pub w_trakcie: u8,
    pub koniec:String,
    pub err:String,
    
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LogRozpakowywanieDds{
    
}

