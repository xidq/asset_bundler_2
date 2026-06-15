use crate::rozszerzenia::bdepth::{BdepthAvif, BdepthExr, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use crate::rozszerzenia::kolor::{ForAvifChroma, ForJpgQuant, ForJpgSamplingFac};
use crate::rozszerzenia::kompresje::{ForAvifKompresja, ForExrKompresja, ForFfKompresja};

#[derive(Clone, Debug, PartialEq)]
pub struct FormatyWyjściowe {
    pub jpg: Option<ImgExtJpg>,
    pub png: Option<ImgExtPng>,
    pub webp: Option<ImgExtWebp>,
    pub tga: Option<ImgExtTga>,
    pub ff: Option<ImgExtFf>,
    pub qoi: Option<ImgExtQoi>,
    pub avif: Option<ImgExtAvif>,
    pub exr: Option<ImgExtExr>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtJpg {
    pub jakosc: u8,
    pub progresywny: bool,
    pub bit_depth: Vec<BdepthJpg>,
    pub sampling: ForJpgSamplingFac,
    pub quant: ForJpgQuant,
    pub scans:u8,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtPng {
    pub kompresja: u8,
    pub bit_depth: Vec<BdepthPng>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtWebp {
    pub jakosc: u8,
    pub lossless: bool,
    pub bit_depth: Vec<BdepthWebp>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtTga {
    pub bit_depth: Vec<BdepthTga>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtFf {
    pub metoda_kompresji: ForFfKompresja,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtQoi {
    pub bit_depth: Vec<BdepthQoi>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtAvif{
    pub chroma: ForAvifChroma,
    pub speed:u8,
    pub metoda_kompresji: ForAvifKompresja,
    pub lossy:Option<u8>,
    pub bit_depth: Vec<BdepthAvif>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImgExtExr{
    pub bit_depth: Vec<BdepthExr>,
    pub kompresja: ForExrKompresja,
}