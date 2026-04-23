use crate::opcje::{
    OptInterpolacja, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd,
    OptMetodaKompresjiZdjecia,
};
use std::fmt;

impl OptKompresjaPlikówPoziomKompresjiZstd {
    pub const WSIOKOMPRESJI: [Self; 3] = [Self::Brak, Self::Standard, Self::Duża];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Brak => "comp_none",
            Self::Standard => "comp_std",
            Self::Duża => "comp_max",
        }
    }
}

impl OptKompresjaPlikówFiltracjaPlików {
    pub const WSIOPLIKOW: [Self; 5] = [
        Self::Wszystkie,
        Self::Graficzne,
        Self::Audio,
        Self::Tekstowe,
        Self::Pdf,
    ];

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
impl fmt::Display for OptMetodaKompresjiZdjecia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptMetodaKompresjiZdjecia::Zstd(_) => write!(f, "Zstd"),
            OptMetodaKompresjiZdjecia::Bzip2(_) => write!(f, "Bzip2"),
            OptMetodaKompresjiZdjecia::Xz(_) => write!(f, "Xz"),
            OptMetodaKompresjiZdjecia::Brak => write!(f, "Brak"),
        }
    }
}
impl OptInterpolacja {
    pub const WSIOINTERPOLACJI: [Self; 5] = [
        Self::Nearest,
        Self::Triangle,
        Self::CatmullRom,
        Self::Gaussian,
        Self::Lanczos3,
    ];

    pub fn klucz(&self) -> &'static str {
        match self {
            Self::Nearest => "OptInterpolacja_nearest",
            Self::Triangle => "OptInterpolacja_triangle",
            Self::CatmullRom => "OptInterpolacja_catmull",
            Self::Gaussian => "OptInterpolacja_gaussian",
            Self::Lanczos3 => "OptInterpolacja_lanczos",
        }
    }
}
#[allow(dead_code)]
impl OptKompresjaPlikówFiltracjaPlików {
    pub const ALL: [Self; 5] = [
        Self::Wszystkie,
        Self::Graficzne,
        Self::Audio,
        Self::Tekstowe,
        Self::Pdf,
    ];
}
impl fmt::Display for OptKompresjaPlikówFiltracjaPlików {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Możesz tu użyć metody .t() jeśli chcesz tłumaczyć nazwy filtrów

        write!(f, "{:?}", self)
        // f.write_str(match self {
        //     Self::Wszystkie => "Apple",
        //     Self::Graficzne => "Orange",
        //     Self::Audio => "Strawberry",
        //     Self::Tekstowe => "Tomato",
        //     Self::Pdf => "PDF",
        // })
    }
}

impl fmt::Display for OptKompresjaPlikówPoziomKompresjiZstd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
