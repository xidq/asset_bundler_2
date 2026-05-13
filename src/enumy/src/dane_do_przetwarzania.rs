use crate::opcje::{OptInterpolacja, OptKompresjaPlikówFiltracjaPlików, OptKompresjaPlikówPoziomKompresjiZstd};
use crate::rozszerzenia::ext::{ImgExt, ImgExtTag, ImgExtSingle};
use crate::rozszerzenia::kompresje::{ForDds, ForDdsKompresja};
use crate::rozszerzenia::rozdzielczosci::Rozdzielczości;
use std::any::Any;
use std::cmp::PartialEq;
use std::path::PathBuf;
use image::imageops::FilterType;

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
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct DaneKonw {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub opcje_rozdzielczości: Vec<Rozdzielczości>,
    pub noising: Option<u8>,
    pub rozszerzenia: Vec<ImgExt>,
    pub tag:Vec<ImgExtTag>,
    pub inter: OptInterpolacja,
    pub alfa_rgb: (u16, u16, u16),
}
impl Default for DaneKonw {
    fn default() -> Self {
        DaneKonw {
            ścieżka_wejściowa: PathBuf::new(),
            ścieżka_wyjściowa: PathBuf::new(),
            opcje_rozdzielczości: Vec::from([Rozdzielczości::R2k]),
            noising: None,
            rozszerzenia: Vec::from([ Default::default() ]),
            tag: Vec::from([ ImgExtTag::Jpg ]),
            inter: OptInterpolacja::Nearest,
            alfa_rgb: (0, 0, 0),
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
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DaneDdsPak {
    pub ścieżka_wejściowa:  Option<Vec<PathBuf>>,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub format: ForDds,
    pub kompresja: ForDdsKompresja,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DaneDdsUnpak {
    pub ścieżka_wejściowa: PathBuf,
    pub ścieżka_wyjściowa: PathBuf,
    pub nazwa: String,
    pub rozszerzenie: ImgExt,
    pub tag: ImgExtTag,
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

pub trait DaneDoObrbki{
    fn jako_any(&self) -> &dyn Any;
    fn jest_rowny(&self, inny: &dyn Any) -> bool;

    fn tag_master(&self) -> Vec<ImgExtTag>;



}


impl DaneDoObrbki for DaneKonw {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    fn tag_master(&self) -> Vec<ImgExtTag> { self.tag.clone() }


}
impl DaneDoObrbki for DaneMerge {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    fn tag_master(&self) -> Vec<ImgExtTag> {
        Vec::from([self.tag.clone()])
    }

}
impl DaneDoObrbki for DaneDdsUnpak {
    fn jako_any(&self) -> &dyn Any { self }
    fn jest_rowny(&self, inny: &dyn Any) -> bool {
        if let Some(v) = inny.downcast_ref::<Self>() {
            return v == self;
        }
        false
    }
    fn tag_master(&self) -> Vec<ImgExtTag> {
        Vec::from([self.tag.clone()])
    }
}


