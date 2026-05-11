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
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display)]
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

#[allow(dead_code)]
#[derive(Clone)]
pub enum OptEfektZaszumiania {
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