use image::imageops::FilterType;
use strum::{Display, EnumIter};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
pub enum OptKompresjaPlikówFiltracjaPlików {
    Wszystkie,
    Graficzne,
    Audio,
    Tekstowe,
    Pdf,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
/// # Existing file handling
/// What should happen to already existing files?
/// 
/// Change? Leave them be? Change name?
/// 
/// Who knows what'll happen?
pub enum OptIstniejePlik{
    Zamień,
    Zostaw,
    ZmieńNazwę,
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display)]
/// Zstd compression for binary packing
pub enum OptKompresjaPlikówPoziomKompresjiZstd {
    Brak = 0,
    Standard = 3,
    Duża = 22,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum OptInterpolacja {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}
impl OptInterpolacja{
    /// Getting FilterType style enum from OptInterpolacja
    pub fn konwertuj(&self) -> FilterType {
        match self{
            Self::Nearest => FilterType::Nearest,
            Self::Triangle => FilterType::Triangle,
            Self::CatmullRom => FilterType::CatmullRom,
            Self::Gaussian => FilterType::Gaussian,
            Self::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum OptEfektZaszumiania {
    /// 0-100 (like percent)
    Tak(u8),
    Nie,
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum FolderCzyPlik{
    Folder,
    Plik,
    Puste
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptUIWariantPodstrony {
    Pakowanie,
    Rozpakowanie,
    KonwersjaFoto,
    DaneDoŁączeniaZdjęć,
    ObslugaDds,
    Ustawienia,
}