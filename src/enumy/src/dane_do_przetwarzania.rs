use crate::opcje::{OptIstniejePlik, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use crate::rozszerzenia::ext::{ImgExt, ImgExtSingle, ImgExtTag};
use crate::rozszerzenia::kompresje::{DdxDxVersion, ForDds, ForDdsKompresja};
use crate::rozszerzenia::rozdzielczosci::Rozdzielczości;
use image::imageops::FilterType;
use std::any::Any;
use std::cmp::PartialEq;
use std::path::PathBuf;
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneBinPak {
    pub ścieżka_in: PathBuf,
    pub ścieżka_out: PathBuf,
    pub kompresja: OptKompresjaPlikówPoziomKompresjiZstd,
    pub nazwa: String,
    pub foldery: bool,
    pub filtracja: OptKompresjaPlikówFiltracjaPlików,
}
impl Default for DaneBinPak {
    fn default() -> DaneBinPak {
        Self{
            ścieżka_in: PathBuf::new(),
            ścieżka_out: PathBuf::new(),
            kompresja: OptKompresjaPlikówPoziomKompresjiZstd::Brak,
            nazwa: String::new(),
            foldery: true,
            filtracja: OptKompresjaPlikówFiltracjaPlików::Wszystkie,
        }
    }
}
// impl ElementyDane for DaneDoKompresjaPlików{}
// impl ElementyDane for DaneDoDekompresjaPlików{}
// impl ElementyDane for DaneDoBathKonwersjaZdjec{}
// impl ElementyDane for DaneDoŁączeniaZdjęć{}
// impl ElementyDane for DaneDoPakowaniaDds{}
// impl ElementyDane for DaneDoRozpakowaniaDds{}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneBinUnpak {
    pub ścieżka_pliku: PathBuf,
    pub ścieżka_docelowa: PathBuf,
}
impl Default for DaneBinUnpak {
    fn default() -> Self {
        Self{
            ścieżka_pliku: PathBuf::new(),
            ścieżka_docelowa: PathBuf::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneMerge {
    pub sciezka_r: Option<PathBuf>,
    pub sciezka_g: Option<PathBuf>,
    pub sciezka_b: Option<PathBuf>,
    pub sciezka_a: Option<PathBuf>,
    pub sciezka_out: PathBuf,
    pub rozszerzenie: ImgExtSingle,
    pub tag: ImgExtTag,
    pub nazwa: String,
    pub istniejace_pliki: OptIstniejePlik,
}
impl Default for DaneMerge {
    fn default() -> Self {
        Self {
            sciezka_r: None,
            sciezka_g: None,
            sciezka_b: None,
            sciezka_a: None,
            sciezka_out: PathBuf::new(),
            // Tutaj musisz podać domyślne warianty swoich enumów:
            rozszerzenie: ImgExtSingle::def_jpg(), // Zmień na swój domyślny wariant
            tag: ImgExtTag::Jpg,             // Zmień na swój domyślny wariant
            nazwa: String::new(),
            istniejace_pliki: OptIstniejePlik::Zamień,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DaneDdsPak {
    pub ścieżka_wejściowa:  Option<Vec<PathBuf>>,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub format: ForDds,
    pub dx_ver: DdxDxVersion,
    pub kompresja: ForDdsKompresja,
}
impl Default for DaneDdsPak {
    fn default() -> Self {
        Self {
            ścieżka_wejściowa: None,
            ścieżka_wyjściowa: PathBuf::new(),
            nazwa: String::new(),
            format: ForDds::DxgiFormatBc7Unorm,
            dx_ver: DdxDxVersion::Dx10,
            kompresja: ForDdsKompresja::Normal,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DaneDdsUnpak {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub rozszerzenie: ImgExt,
    pub tag: ImgExtTag,
    pub istniejace_pliki: OptIstniejePlik
}
impl Default for DaneDdsUnpak {
    fn default() -> Self {
        Self{
            ścieżka_wejściowa: PathBuf::new(),
            ścieżka_wyjściowa: PathBuf::new(),
            nazwa: String::new(),
            rozszerzenie: ImgExt::def_jpg(),
            tag: ImgExtTag::Jpg,
            istniejace_pliki: OptIstniejePlik::Zamień,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneProces {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub opcje_rozdzielczości: Vec<Rozdzielczości>,
    pub noising: Option<u8>,
    pub rozszerzenia: Vec<ImgExt>,
    pub inter: FilterType,
    pub alfa_rgb: (u16, u16, u16),
}
impl Default for DaneProces {
    fn default() -> Self {
        Self{
            ścieżka_wejściowa: PathBuf::new(),
            ścieżka_wyjściowa: PathBuf::new(),
            opcje_rozdzielczości: vec![Rozdzielczości::R2k],
            noising: None,
            rozszerzenia: vec![ImgExt::def_jpg()],
            inter: FilterType::Lanczos3,
            alfa_rgb: (0, 0, 0),
        }
    }
}

pub trait DaneDoObrbki{
    fn jako_any(&self) -> &dyn Any;
    fn jest_rowny(&self, inny: &dyn Any) -> bool;
    // fn tag_master(&self) -> Vec<ImgExtTag>;

    // fn dane_do_spr(&self) -> ;

}


impl DaneDoObrbki for DaneMerge {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    // fn tag_master(&self) -> Vec<ImgExtTag> {
    //     Vec::from([self.tag.clone()])
    // }

}
impl DaneDoObrbki for DaneDdsUnpak {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    // fn tag_master(&self) -> Vec<ImgExtTag> {
    //     Vec::from([self.tag.clone()])
    // }
}


