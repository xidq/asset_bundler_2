use crate::opcje::OptInterpolacja;
use crate::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use crate::rozszerzenia::ext::ImgExtTag;
use crate::rozszerzenia::kolor::{ColorProfilePhoto, ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};
use crate::rozszerzenia::rozdzielczosci::Rozdzielczości;
use image::DynamicImage;
use std::path::PathBuf;

pub trait DaneDoPrzetwarzania<T>{
    fn daj_dane(&self) -> Self;
    fn czym_jestem(&self) -> ImgExtTag;
    fn bufor(&self) -> &DynamicImage;
    /// get resolution
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości>;
    /// get input path
    fn sciezka_wejsciowa(&self) -> PathBuf;
    /// get interpoolation method
    fn interpolacja(&self) -> &OptInterpolacja;
    /// get bit_depth
    fn bdepth(&self) -> &Vec<T>;
    fn jako_enum(self) -> TypyPrzetwarzania;
}
pub enum TypyPrzetwarzania{
    PrzJpg(PrzetwarzanieJpg),
    PrzPng(PrzetwarzaniePng),
    PrzAvif(PrzetwarzanieAvif),
    PrzWebp(PrzetwarzanieWebp),
    PrzQoi(PrzetwarzanieQoi),
    PrzTga(PrzetwarzanieTga),
    PrzFf(PrzetwarzanieFf),
    PrzExr(PrzetwarzanieExr),
}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieJpg{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub jakosc: u8,
    pub progresywny: bool,
    pub bit_depth:  Vec<BdepthJpg>,
    pub sampling: ForJpgSamplingFac,
    pub quant: ForJpgQuant,
    pub scans: u8,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}
impl DaneDoPrzetwarzania<BdepthJpg> for PrzetwarzanieJpg{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Jpg}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthJpg> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzJpg(self)}
}
#[derive(Debug, Clone)]
pub struct PrzetwarzaniePng{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub kompresja: u8,
    pub bit_depth:  Vec<BdepthPng>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}
impl DaneDoPrzetwarzania<BdepthPng> for PrzetwarzaniePng{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthPng> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzPng(self)}

}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieAvif{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub bit_depth:  Vec<BdepthAvif>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
    pub chroma: ForAvifChroma,
    pub speed:i32,
    pub metoda_kompresji: ForAvifKompresja,
    pub lossy:Option<u8>,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}
impl DaneDoPrzetwarzania<BdepthAvif> for PrzetwarzanieAvif{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthAvif> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzAvif(self)}

}

#[derive(Debug, Clone)]
pub struct PrzetwarzanieWebp{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub bit_depth:  Vec<BdepthWebp>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
    pub lossy: Option<u8>,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}
impl DaneDoPrzetwarzania<BdepthWebp> for PrzetwarzanieWebp{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthWebp> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzWebp(self)}

}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieQoi{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub bit_depth:  Vec<BdepthQoi>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
}
impl DaneDoPrzetwarzania<BdepthQoi> for PrzetwarzanieQoi{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthQoi> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzQoi(self)}

}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieTga{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub bit_depth:  Vec<BdepthTga>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
}
impl DaneDoPrzetwarzania<BdepthTga> for PrzetwarzanieTga{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthTga> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzTga(self)}

}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieFf{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub kompresja:  Vec<ForFfKompresja>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
}
impl DaneDoPrzetwarzania<ForFfKompresja> for PrzetwarzanieFf{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Png}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<ForFfKompresja> {&self.kompresja}
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzFf(self)}

}
#[derive(Debug, Clone)]
pub struct PrzetwarzanieExr{
    pub bufor: DynamicImage,
    pub rozdzielczosci:  Vec<Rozdzielczości>,
    pub sciezka_wyjsciowa: PathBuf,
    pub nazwa: String,
    pub interpolacja: OptInterpolacja,
    pub kompresja: ForExrKompresja,
    pub bit_depth:  Vec<BdepthExr>,
    pub alpha: (u16, u16, u16),
    pub zaszumienie: Option<u8>,
    pub kolor: ColorProfilePhoto,
}
impl DaneDoPrzetwarzania<BdepthExr> for PrzetwarzanieExr{
    fn daj_dane(&self) -> Self {self.clone()}
    fn czym_jestem(&self) -> ImgExtTag {ImgExtTag::Exr}
    fn bufor(&self) -> &DynamicImage {&self.bufor}
    fn rozdzielczosci(&self) -> &Vec<Rozdzielczości> {&self.rozdzielczosci}
    fn sciezka_wejsciowa(&self) -> PathBuf {self.sciezka_wyjsciowa.clone()}
    fn interpolacja(&self) -> &OptInterpolacja {&self.interpolacja}
    fn bdepth(&self) -> &Vec<BdepthExr> {&self.bit_depth }
    fn jako_enum(self) -> TypyPrzetwarzania {TypyPrzetwarzania::PrzExr(self)}
}